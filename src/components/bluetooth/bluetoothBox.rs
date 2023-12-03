use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};

use adw::glib;
use adw::glib::Object;
use adw::prelude::ListModelExtManual;
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::blocking::Connection;
use dbus::message::SignalArgs;
use dbus::{Error, Path};
use glib::Cast;
use gtk::{gio, Widget};
use gtk::glib::Variant;
use gtk::prelude::{ActionableExt, BoxExt, ListBoxRowExt, WidgetExt};
use ReSet_Lib::bluetooth::bluetooth::BluetoothDevice;
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
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn setupCallbacks(&self) {
        let selfImp = self.imp();
        selfImp.resetVisibility.set_activatable(true);
        selfImp
            .resetVisibility
            .set_action_name(Some("navigation.push"));
        selfImp
            .resetVisibility
            .set_action_target_value(Some(&Variant::from("visibility")));

        selfImp
            .resetBluetoothMainTab
            .set_action_name(Some("navigation.pop"));
    }
}

impl Default for BluetoothBox {
    fn default() -> Self {
        Self::new()
    }
}

pub fn populate_conntected_bluetooth_devices(bluetooth_box: Arc<BluetoothBox>) {
    gio::spawn_blocking(move || {
        let ref_box = bluetooth_box.clone();
        let devices = get_connected_devices();

        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let imp = ref_box.imp();
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
                        for x in imp.resetBluetoothAvailableDevices.observe_children().iter::<Object>() {
                            if let Ok(entry) = x { // todo test this
                                if let Some(item) = entry.downcast_ref::<Widget>() {
                                    imp.resetBluetoothAvailableDevices.remove(item);
                                }
                            }
                        };
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
