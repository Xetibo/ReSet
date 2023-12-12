use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};

use adw::glib;
use adw::glib::Object;
use adw::prelude::{ComboRowExt, ListModelExtManual, PreferencesGroupExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::blocking::Connection;
use dbus::message::SignalArgs;
use dbus::{Error, Path};
use glib::{clone, Cast};
use gtk::glib::Variant;
use gtk::prelude::{ActionableExt, ButtonExt, ListBoxRowExt, WidgetExt};
use gtk::{gio, StringObject, Widget};
use re_set_lib::bluetooth::bluetooth_structures::{BluetoothAdapter, BluetoothDevice};
use re_set_lib::signals::{BluetoothDeviceAdded, BluetoothDeviceChanged, BluetoothDeviceRemoved};

use crate::components::base::utils::Listeners;
use crate::components::bluetooth::bluetooth_box_impl;
use crate::components::bluetooth::bluetooth_entry::BluetoothEntry;

glib::wrapper! {
    pub struct BluetoothBox(ObjectSubclass<bluetooth_box_impl::BluetoothBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for BluetoothBox {}
unsafe impl Sync for BluetoothBox {}

impl BluetoothBox {
    pub fn new(listeners: Arc<Listeners>) -> Arc<Self> {
        let obj: Arc<Self> = Arc::new(Object::builder().build());
        setup_callbacks(listeners, obj)
    }
}

// TODO
// handle bonded -> this means saved but not connected
// handle rssi below x -> don't show device

fn setup_callbacks(
    listeners: Arc<Listeners>,
    bluetooth_box: Arc<BluetoothBox>,
) -> Arc<BluetoothBox> {
    let bluetooth_box_ref = bluetooth_box.clone();
    let listeners_ref = listeners.clone();
    let imp = bluetooth_box.imp();
    imp.reset_visibility.set_activatable(true);
    imp.reset_visibility
        .set_action_name(Some("navigation.push"));
    imp.reset_visibility
        .set_action_target_value(Some(&Variant::from("visibility")));

    imp.reset_bluetooth_main_tab.set_activatable(true);
    imp.reset_bluetooth_main_tab
        .set_action_name(Some("navigation.pop"));

    imp.reset_bluetooth_refresh_button.set_sensitive(false);
    imp.reset_bluetooth_refresh_button
        .connect_clicked(move |button| {
            button.set_sensitive(false);
            listeners
                .bluetooth_scan_requested
                .store(true, Ordering::SeqCst);
        });

    imp.reset_bluetooth_discoverable_switch
        .connect_active_notify(clone!(@weak imp => move |state| {
            set_bluetooth_adapter_visibility(imp.reset_current_bluetooth_adapter.borrow().path.clone(), state.is_active());
        }));

    imp.reset_bluetooth_pairable_switch
        .connect_active_notify(clone!(@weak imp => move |state| {
            set_bluetooth_adapter_pairability(imp.reset_current_bluetooth_adapter.borrow().path.clone(), state.is_active());
        }));

    imp.reset_bluetooth_switch
        .connect_state_set(move |_, state| {
            if !state {
                let imp = bluetooth_box_ref.imp();
                for x in imp
                    .reset_bluetooth_connected_devices
                    .observe_children()
                    .iter::<Object>()
                    .flatten()
                {
                    // todo test this
                    if let Some(item) = x.downcast_ref::<Widget>() {
                        imp.reset_bluetooth_available_devices.remove(item);
                    }
                }
                listeners_ref
                    .bluetooth_listener
                    .store(false, Ordering::SeqCst);
                set_adapter_enabled(
                    imp.reset_current_bluetooth_adapter.borrow().path.clone(),
                    false,
                );
            } else {
                let imp = bluetooth_box_ref.imp();
                set_adapter_enabled(
                    imp.reset_current_bluetooth_adapter.borrow().path.clone(),
                    true,
                );
                start_bluetooth_listener(listeners_ref.clone(), bluetooth_box_ref.clone());
            }
            glib::Propagation::Proceed
        });
    bluetooth_box
}

pub fn populate_conntected_bluetooth_devices(bluetooth_box: Arc<BluetoothBox>) {
    // TODO handle saved devices -> they also exist
    gio::spawn_blocking(move || {
        let ref_box = bluetooth_box.clone();
        let devices = get_connected_devices();
        let adapters = get_bluetooth_adapters();
        {
            let imp = bluetooth_box.imp();
            let list = imp.reset_model_list.write().unwrap();
            let mut model_index = imp.reset_model_index.write().unwrap();
            let mut map = imp.reset_bluetooth_adapters.write().unwrap();
            {
                if imp.reset_bluetooth_adapters.read().unwrap().is_empty() {
                    return;
                }
            }
            imp.reset_current_bluetooth_adapter
                .replace(adapters.last().unwrap().clone());
            for (index, adapter) in adapters.into_iter().enumerate() {
                list.append(&adapter.alias);
                map.insert(adapter.alias.clone(), (adapter, index as u32));
                *model_index += 1;
            }
        }
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let imp = ref_box.imp();

                let list = imp.reset_model_list.read().unwrap();
                imp.reset_bluetooth_adapter.set_model(Some(&*list));
                let map = imp.reset_bluetooth_adapters.read().unwrap();
                let device = imp.reset_current_bluetooth_adapter.borrow();
                if let Some(index) = map.get(&device.alias) {
                    imp.reset_bluetooth_adapter.set_selected(index.1);
                }

                {
                    let current_adapter = imp.reset_current_bluetooth_adapter.borrow();
                    imp.reset_bluetooth_switch
                        .set_state(current_adapter.powered);
                    imp.reset_bluetooth_switch
                        .set_active(current_adapter.powered);
                    imp.reset_bluetooth_discoverable_switch
                        .set_active(current_adapter.discoverable);
                    imp.reset_bluetooth_discoverable_switch
                        .set_active(current_adapter.discoverable);
                    imp.reset_bluetooth_pairable_switch
                        .set_active(current_adapter.pairable);
                    imp.reset_bluetooth_pairable_switch
                        .set_active(current_adapter.pairable);
                }

                imp.reset_bluetooth_adapter.connect_selected_notify(
                    clone!(@weak imp => move |dropdown| {
                        let selected = dropdown.selected_item();
                        if selected.is_none() {
                            return;
                        }
                        let selected = selected.unwrap();
                        let selected = selected.downcast_ref::<StringObject>().unwrap();
                        let selected = selected.string().to_string();

                        let device = imp.reset_bluetooth_adapters.read().unwrap();
                        let device = device.get(&selected);
                        if device.is_none() {
                            return;
                        }
                        set_bluetooth_adapter(device.unwrap().0.path.clone());
                    }),
                );

                for device in devices {
                    let path = device.path.clone();
                    let connected = device.connected;
                    let bluetooth_entry = BluetoothEntry::new(device);
                    imp.available_devices
                        .borrow_mut()
                        .insert(path, bluetooth_entry.clone());
                    if connected {
                        imp.reset_bluetooth_connected_devices.add(&*bluetooth_entry);
                    } else {
                        imp.reset_bluetooth_available_devices.add(&*bluetooth_entry);
                    }
                }
            });
        });
    });
}

pub fn start_bluetooth_listener(listeners: Arc<Listeners>, bluetooth_box: Arc<BluetoothBox>) {
    gio::spawn_blocking(move || {
        if listeners.bluetooth_listener.load(Ordering::SeqCst) {
            return;
        }

        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSet.Daemon",
            "/org/Xetibo/ReSet/Daemon",
            Duration::from_millis(1000),
        );
        let _: Result<(), Error> =
            proxy.method_call("org.Xetibo.ReSet.Bluetooth", "StartBluetoothListener", ());
        let device_added = BluetoothDeviceAdded::match_rule(
            Some(&"org.Xetibo.ReSet.Daemon".into()),
            Some(&Path::from("/org/Xetibo/ReSet/Daemon")),
        )
        .static_clone();
        let device_removed = BluetoothDeviceRemoved::match_rule(
            Some(&"org.Xetibo.ReSet.Daemon".into()),
            Some(&Path::from("/org/Xetibo/ReSet/Daemon")),
        )
        .static_clone();
        let device_changed = BluetoothDeviceChanged::match_rule(
            Some(&"org.Xetibo.ReSet.Daemon".into()),
            Some(&Path::from("/org/Xetibo/ReSet/Daemon")),
        )
        .static_clone();
        let device_added_box = bluetooth_box.clone();
        let device_removed_box = bluetooth_box.clone();
        let device_changed_box = bluetooth_box.clone();
        let loop_box = bluetooth_box.clone();

        let res = conn.add_match(device_added, move |ir: BluetoothDeviceAdded, _, _| {
            let bluetooth_box = device_added_box.clone();
            glib::spawn_future(async move {
                glib::idle_add_once(move || {
                    let imp = bluetooth_box.imp();
                    let path = ir.bluetooth_device.path.clone();
                    let connected = ir.bluetooth_device.connected;
                    let bluetooth_entry = BluetoothEntry::new(ir.bluetooth_device);
                    imp.available_devices
                        .borrow_mut()
                        .insert(path, bluetooth_entry.clone());
                    if connected {
                        imp.reset_bluetooth_connected_devices.add(&*bluetooth_entry);
                    } else {
                        imp.reset_bluetooth_available_devices.add(&*bluetooth_entry);
                    }
                });
            });
            true
        });
        if res.is_err() {
            println!("fail on bluetooth device add event");
            return;
        }

        let res = conn.add_match(device_removed, move |ir: BluetoothDeviceRemoved, _, _| {
            let bluetooth_box = device_removed_box.clone();
            glib::spawn_future(async move {
                glib::idle_add_once(move || {
                    let imp = bluetooth_box.imp();
                    let mut map = imp.available_devices.borrow_mut();
                    if let Some(list_entry) = map.remove(&ir.bluetooth_device) {
                        if list_entry.imp().bluetooth_device.borrow().connected {
                            imp.reset_bluetooth_connected_devices.remove(&*list_entry);
                        } else {
                            imp.reset_bluetooth_available_devices.remove(&*list_entry);
                        }
                    }
                });
            });
            true
        });
        if res.is_err() {
            println!("fail on bluetooth device remove event");
            return;
        }

        let res = conn.add_match(device_changed, move |ir: BluetoothDeviceChanged, _, _| {
            let bluetooth_box = device_changed_box.clone();
            glib::spawn_future(async move {
                glib::idle_add_once(move || {
                    let imp = bluetooth_box.imp();
                    let mut map = imp.available_devices.borrow_mut();
                    if let Some(list_entry) = map.get_mut(&ir.bluetooth_device.path) {
                        let mut existing_bluetooth_device =
                            list_entry.imp().bluetooth_device.borrow_mut();
                        if existing_bluetooth_device.connected != ir.bluetooth_device.connected {
                            if ir.bluetooth_device.connected {
                                imp.reset_bluetooth_connected_devices.add(&**list_entry);
                                imp.reset_bluetooth_available_devices.remove(&**list_entry);
                            } else {
                                imp.reset_bluetooth_available_devices.add(&**list_entry);
                                imp.reset_bluetooth_connected_devices.remove(&**list_entry);
                            }
                        }
                        if existing_bluetooth_device.bonded != ir.bluetooth_device.bonded {
                            if ir.bluetooth_device.bonded {
                                list_entry
                                    .imp()
                                    .remove_device_button
                                    .borrow()
                                    .set_sensitive(true);
                            } else {
                                list_entry
                                    .imp()
                                    .remove_device_button
                                    .borrow()
                                    .set_sensitive(false);
                            }
                        }
                        *existing_bluetooth_device = ir.bluetooth_device;
                    }
                });
            });
            true
        });
        if res.is_err() {
            println!("fail on bluetooth device remove event");
            return;
        }

        listeners.bluetooth_listener.store(true, Ordering::SeqCst);
        let mut time = SystemTime::now();
        let mut listener_active = true;

        loop {
            let _ = conn.process(Duration::from_millis(1000));
            if !listeners.bluetooth_listener.load(Ordering::SeqCst) {
                let _: Result<(), Error> =
                    proxy.method_call("org.Xetibo.ReSet.Bluetooth", "StopBluetoothListener", ());
                break;
            }
            if listener_active && time.elapsed().unwrap() > Duration::from_millis(10000) {
                listener_active = false;
                let instance_ref = loop_box.clone();
                glib::spawn_future(async move {
                    glib::idle_add_once(move || {
                        instance_ref
                            .imp()
                            .reset_bluetooth_refresh_button
                            .set_sensitive(true);
                    });
                });
                let _: Result<(), Error> =
                    proxy.method_call("org.Xetibo.ReSet.Bluetooth", "StopBluetoothScan", ());
            }
            if !listener_active && listeners.bluetooth_scan_requested.load(Ordering::SeqCst) {
                listeners
                    .bluetooth_scan_requested
                    .store(false, Ordering::SeqCst);
                listener_active = true;
                let _: Result<(), Error> =
                    proxy.method_call("org.Xetibo.ReSet.Bluetooth", "StartBluetoothListener", ());
                time = SystemTime::now();
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
}

fn get_connected_devices() -> Vec<BluetoothDevice> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSet.Daemon",
        "/org/Xetibo/ReSet/Daemon",
        Duration::from_millis(1000),
    );
    let res: Result<(Vec<BluetoothDevice>,), Error> = proxy.method_call(
        "org.Xetibo.ReSet.Bluetooth",
        "GetConnectedBluetoothDevices",
        (),
    );
    if res.is_err() {
        return Vec::new();
    }
    res.unwrap().0
}

fn get_bluetooth_adapters() -> Vec<BluetoothAdapter> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSet.Daemon",
        "/org/Xetibo/ReSet/Daemon",
        Duration::from_millis(1000),
    );
    let res: Result<(Vec<BluetoothAdapter>,), Error> =
        proxy.method_call("org.Xetibo.ReSet.Bluetooth", "GetBluetoothAdapters", ());
    if res.is_err() {
        return Vec::new();
    }
    res.unwrap().0
}

fn set_bluetooth_adapter(path: Path<'static>) {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSet.Daemon",
        "/org/Xetibo/ReSet/Daemon",
        Duration::from_millis(1000),
    );
    let _: Result<(Vec<BluetoothAdapter>,), Error> =
        proxy.method_call("org.Xetibo.ReSet.Bluetooth", "SetBluetoothAdapter", (path,));
}

fn set_bluetooth_adapter_visibility(path: Path<'static>, visible: bool) {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSet.Daemon",
        "/org/Xetibo/ReSet/Daemon",
        Duration::from_millis(1000),
    );
    let _: Result<(bool,), Error> = proxy.method_call(
        "org.Xetibo.ReSet.Bluetooth",
        "SetBluetoothAdapterDiscoverability",
        (path, visible),
    );
}

fn set_bluetooth_adapter_pairability(path: Path<'static>, visible: bool) {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSet.Daemon",
        "/org/Xetibo/ReSet/Daemon",
        Duration::from_millis(1000),
    );
    let _: Result<(bool,), Error> = proxy.method_call(
        "org.Xetibo.ReSet.Bluetooth",
        "SetBluetoothAdapterPairability",
        (path, visible),
    );
}

fn set_adapter_enabled(path: Path<'static>, enabled: bool) {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSet.Daemon",
        "/org/Xetibo/ReSet/Daemon",
        Duration::from_millis(1000),
    );
    let _: Result<(Vec<BluetoothAdapter>,), Error> = proxy.method_call(
        "org.Xetibo.ReSet.Bluetooth",
        "SetBluetoothAdapterEnabled",
        (path, enabled),
    );
}
