use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};

use adw::glib;
use adw::glib::Object;
use adw::prelude::ComboRowExt;
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::blocking::Connection;
use dbus::message::SignalArgs;
use dbus::{Error, Path};
use glib::{clone, Cast};
use gtk::glib::Variant;
use gtk::prelude::{ActionableExt, ListBoxRowExt, WidgetExt};
use gtk::{gio, StringObject};
use ReSet_Lib::bluetooth::bluetooth::{BluetoothAdapter, BluetoothDevice};
use ReSet_Lib::signals::{BluetoothDeviceAdded, BluetoothDeviceChanged, BluetoothDeviceRemoved};

use crate::components::base::listEntry::ListEntry;
use crate::components::base::utils::Listeners;
use crate::components::bluetooth::bluetoothBoxImpl;
use crate::components::bluetooth::bluetoothEntry::BluetoothEntry;

glib::wrapper! {
    pub struct BluetoothBox(ObjectSubclass<bluetoothBoxImpl::BluetoothBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for BluetoothBox {}
unsafe impl Sync for BluetoothBox {}

impl BluetoothBox {
    pub fn new(listeners: Arc<Listeners>) -> Arc<Self> {
        let obj: Arc<Self> = Arc::new(Object::builder().build());
        setupCallbacks(listeners, obj)
    }

    pub fn setupCallbacks(&self) {}
}

fn setupCallbacks(
    listeners: Arc<Listeners>,
    bluetooth_box: Arc<BluetoothBox>,
) -> Arc<BluetoothBox> {
    let bluetooth_box_ref = bluetooth_box.clone();
    let imp = bluetooth_box.imp();
    // let bluetooth_box_ref = bluetooth_box.clone();
    imp.resetVisibility.set_activatable(true);
    imp.resetVisibility.set_action_name(Some("navigation.push"));
    imp.resetVisibility
        .set_action_target_value(Some(&Variant::from("visibility")));

    imp.resetBluetoothMainTab
        .set_action_name(Some("navigation.pop"));
    // TODO add a manual search button here
    imp.resetBluetoothSwitch.connect_state_set(move |_, state| {
        if !state {
            let imp = bluetooth_box_ref.imp();
            imp.resetBluetoothConnectedDevices.remove_all();
            listeners.bluetooth_listener.store(false, Ordering::SeqCst);
            set_adapter_enabled(
                imp.resetCurrentBluetoothAdapter.borrow().path.clone(),
                false,
            );
        } else {
            let imp = bluetooth_box_ref.imp();
            set_adapter_enabled(imp.resetCurrentBluetoothAdapter.borrow().path.clone(), true);
            start_bluetooth_listener(listeners.clone(), bluetooth_box_ref.clone());
        }
        glib::Propagation::Proceed
    });
    bluetooth_box
}

pub fn populate_conntected_bluetooth_devices(bluetooth_box: Arc<BluetoothBox>) {
    gio::spawn_blocking(move || {
        let ref_box = bluetooth_box.clone();
        let devices = get_connected_devices();
        let adapters = get_bluetooth_adapters();
        {
            let imp = bluetooth_box.imp();
            let list = imp.resetModelList.write().unwrap();
            let mut model_index = imp.resetModelIndex.write().unwrap();
            let mut map = imp.resetBluetoothAdapters.write().unwrap();
            imp.resetCurrentBluetoothAdapter
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

                let list = imp.resetModelList.read().unwrap();
                imp.resetBluetoothAdapter.set_model(Some(&*list));
                let map = imp.resetBluetoothAdapters.read().unwrap();
                let device = imp.resetCurrentBluetoothAdapter.borrow();
                if let Some(index) = map.get(&device.alias) {
                    imp.resetBluetoothAdapter.set_selected(index.1);
                }
                imp.resetBluetoothAdapter.connect_selected_notify(
                    clone!(@weak imp => move |dropdown| {
                        let selected = dropdown.selected_item();
                        if selected.is_none() {
                            return;
                        }
                        let selected = selected.unwrap();
                        let selected = selected.downcast_ref::<StringObject>().unwrap();
                        let selected = selected.string().to_string();

                        let device = imp.resetBluetoothAdapters.read().unwrap();
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
                    let bluetooth_entry = Arc::new(BluetoothEntry::new(&device));
                    let entry = Arc::new(ListEntry::new(&*bluetooth_entry));
                    imp.availableDevices
                        .borrow_mut()
                        .insert(path, (bluetooth_entry.clone(), entry.clone(), device));
                    if connected {
                        imp.resetBluetoothConnectedDevices.append(&*entry);
                    } else {
                        imp.resetBluetoothAvailableDevices.append(&*entry);
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
            "org.Xetibo.ReSetDaemon",
            "/org/Xetibo/ReSetDaemon",
            Duration::from_millis(1000),
        );
        let _: Result<(), Error> = proxy.method_call(
            "org.Xetibo.ReSetBluetooth",
            "StartBluetoothListener",
            #[allow(clippy::unnecessary_cast)]
            (25000 as u32,),
            // leave me alone clippy, I am dealing with C code
        );
        let device_added = BluetoothDeviceAdded::match_rule(
            Some(&"org.Xetibo.ReSetDaemon".into()),
            Some(&Path::from("/org/Xetibo/ReSetDaemon")),
        )
        .static_clone();
        let device_removed = BluetoothDeviceRemoved::match_rule(
            Some(&"org.Xetibo.ReSetDaemon".into()),
            Some(&Path::from("/org/Xetibo/ReSetDaemon")),
        )
        .static_clone();
        let device_changed = BluetoothDeviceChanged::match_rule(
            Some(&"org.Xetibo.ReSetDaemon".into()),
            Some(&Path::from("/org/Xetibo/ReSetDaemon")),
        )
        .static_clone();
        let device_added_box = bluetooth_box.clone();
        let device_removed_box = bluetooth_box.clone();
        let device_changed_box = bluetooth_box.clone();
        let loop_box = bluetooth_box.clone();

        let res = conn.add_match(device_added, move |ir: BluetoothDeviceAdded, _, _| {
            let bluetooth_box = device_added_box.clone();
            println!("added");
            glib::spawn_future(async move {
                glib::idle_add_once(move || {
                    println!("{}", ir.bluetooth_device.icon);
                    let imp = bluetooth_box.imp();
                    let path = ir.bluetooth_device.path.clone();
                    let connected = ir.bluetooth_device.connected;
                    let bluetooth_entry = Arc::new(BluetoothEntry::new(&ir.bluetooth_device));
                    let entry = Arc::new(ListEntry::new(&*bluetooth_entry));
                    imp.availableDevices.borrow_mut().insert(
                        path,
                        (bluetooth_entry.clone(), entry.clone(), ir.bluetooth_device),
                    );
                    if connected {
                        imp.resetBluetoothConnectedDevices.append(&*entry);
                    } else {
                        imp.resetBluetoothAvailableDevices.append(&*entry);
                    }
                });
            });
            true
        });
        if res.is_err() {
            println!("fail on bluetooth device add");
            return;
        }

        let res = conn.add_match(device_removed, move |ir: BluetoothDeviceRemoved, _, _| {
            let bluetooth_box = device_removed_box.clone();
            println!("removed");
            glib::spawn_future(async move {
                glib::idle_add_once(move || {
                    let imp = bluetooth_box.imp();
                    let map = imp.availableDevices.borrow_mut();
                    if let Some(list_entry) = map.get(&ir.bluetooth_device) {
                        imp.resetBluetoothAvailableDevices.remove(&*list_entry.1);
                        imp.resetBluetoothConnectedDevices.remove(&*list_entry.1);
                    }
                });
            });
            true
        });
        if res.is_err() {
            println!("fail on bluetooth device remove");
            return;
        }

        let res = conn.add_match(device_changed, move |ir: BluetoothDeviceChanged, _, _| {
            let bluetooth_box = device_changed_box.clone();
            println!("removed");
            glib::spawn_future(async move {
                glib::idle_add_once(move || {
                    let imp = bluetooth_box.imp();
                    let map = imp.availableDevices.borrow_mut();
                    if let Some(list_entry) = map.get(&ir.bluetooth_device.path) {
                        if list_entry.2.connected != ir.bluetooth_device.connected {
                            if ir.bluetooth_device.connected {
                                imp.resetBluetoothConnectedDevices.append(&*list_entry.1);
                                imp.resetBluetoothAvailableDevices.remove(&*list_entry.1);
                            } else {
                                imp.resetBluetoothAvailableDevices.append(&*list_entry.1);
                                imp.resetBluetoothConnectedDevices.remove(&*list_entry.1);
                            }
                        }
                        if list_entry.2.paired != ir.bluetooth_device.paired {
                            if ir.bluetooth_device.paired {
                                list_entry.0.imp().resetBluetoothButton.set_sensitive(true);
                            } else {
                                list_entry.0.imp().resetBluetoothButton.set_sensitive(false);
                            }
                        }
                    }
                });
            });
            true
        });
        if res.is_err() {
            println!("fail on bluetooth device remove");
            return;
        }

        listeners.bluetooth_listener.store(true, Ordering::SeqCst);
        let time = SystemTime::now();

        loop {
            let _ = conn.process(Duration::from_millis(1000));
            if !listeners.bluetooth_listener.load(Ordering::SeqCst)
                || time.elapsed().unwrap() > Duration::from_millis(25000)
            {
                glib::spawn_future(async move {
                    glib::idle_add_once(move || {
                        let imp = loop_box.imp();
                        imp.resetBluetoothAvailableDevices.remove_all();
                    });
                });
                println!("stopping bluetooth listener");
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
}

fn get_connected_devices() -> Vec<BluetoothDevice> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ReSetDaemon",
        Duration::from_millis(1000),
    );
    let res: Result<(Vec<BluetoothDevice>,), Error> = proxy.method_call(
        "org.Xetibo.ReSetBluetooth",
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
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ReSetDaemon",
        Duration::from_millis(1000),
    );
    let res: Result<(Vec<BluetoothAdapter>,), Error> =
        proxy.method_call("org.Xetibo.ReSetBluetooth", "GetBluetoothAdapters", ());
    if res.is_err() {
        return Vec::new();
    }
    res.unwrap().0
}

fn set_bluetooth_adapter(path: Path<'static>) {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ReSetDaemon",
        Duration::from_millis(1000),
    );
    let _: Result<(Vec<BluetoothAdapter>,), Error> =
        proxy.method_call("org.Xetibo.ReSetBluetooth", "SetBluetoothAdapter", (path,));
}

fn set_adapter_enabled(path: Path<'static>, enabled: bool) {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ReSetDaemon",
        Duration::from_millis(1000),
    );
    let _: Result<(Vec<BluetoothAdapter>,), Error> = proxy.method_call(
        "org.Xetibo.ReSetBluetooth",
        "SetBluetoothAdapterEnabled",
        (path, enabled),
    );
}
