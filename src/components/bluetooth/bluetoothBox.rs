use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use adw::glib;
use adw::glib::Object;
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::blocking::Connection;
use dbus::message::SignalArgs;
use dbus::{Error, Path};
use gtk::gio;
use gtk::glib::Variant;
use gtk::prelude::ActionableExt;
use ReSet_Lib::signals::{BluetoothDeviceAdded, BluetoothDeviceRemoved};

use crate::components::base::listEntry::ListEntry;
use crate::components::base::utils::Listeners;
use crate::components::bluetooth::bluetoothBoxImpl;
use crate::components::bluetooth::bluetoothEntry::BluetoothEntry;
use crate::components::bluetooth::bluetoothEntryImpl::DeviceTypes;

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

    pub fn scanForDevices(&self) {
        let selfImp = self.imp();
        let mut wifiEntries = selfImp.availableDevices.borrow_mut();
        wifiEntries.push(ListEntry::new(&BluetoothEntry::new(
            DeviceTypes::Mouse,
            "ina mouse",
        )));
        wifiEntries.push(ListEntry::new(&BluetoothEntry::new(
            DeviceTypes::Keyboard,
            "inaboard",
        )));
        wifiEntries.push(ListEntry::new(&BluetoothEntry::new(
            DeviceTypes::Controller,
            "ina controller",
        )));
        wifiEntries.push(ListEntry::new(&BluetoothEntry::new(
            DeviceTypes::Controller,
            "ina best waifu",
        )));

        for wifiEntry in wifiEntries.iter() {
            selfImp.resetBluetoothAvailableDevices.append(wifiEntry);
        }
    }

    pub fn addConnectedDevices(&self) {
        let selfImp = self.imp();
        let mut wifiEntries = selfImp.connectedDevices.borrow_mut();
        wifiEntries.push(ListEntry::new(&BluetoothEntry::new(
            DeviceTypes::Mouse,
            "why are we still here?",
        )));
        wifiEntries.push(ListEntry::new(&BluetoothEntry::new(
            DeviceTypes::Keyboard,
            "just to suffer?",
        )));

        for wifiEntry in wifiEntries.iter() {
            selfImp.resetBluetoothConnectedDevices.append(wifiEntry);
        }
    }
}

pub fn start_bluetooth_listener(listeners: Arc<Listeners>, bluetooth_box: Arc<BluetoothBox>) {
    gio::spawn_blocking(move || {
        if listeners.bluetooth_listener.load(Ordering::SeqCst) {
            return;
        }

        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.xetibo.ReSet",
            "/org/xetibo/ReSet",
            Duration::from_millis(1000),
        );
        let _: Result<(), Error> =
            proxy.method_call("org.xetibo.ReSet", "StartBluetoothSearch", (5000,));
        let device_added = BluetoothDeviceAdded::match_rule(
            Some(&"org.xetibo.ReSet".into()),
            Some(&Path::from("/org/xetibo/ReSet")),
        )
        .static_clone();
        let device_removed = BluetoothDeviceRemoved::match_rule(
            Some(&"org.xetibo.ReSet".into()),
            Some(&Path::from("/org/xetibo/ReSet")),
        )
        .static_clone();
        let device_added_box = bluetooth_box.clone();
        let device_removed_box = bluetooth_box.clone();

        let res = conn.add_match(device_added, move |ir: BluetoothDeviceAdded, _, _| {
            let bluetooth_box = device_added_box.clone();
            println!("added");
            glib::spawn_future(async move {
                glib::idle_add_once(move || {
                    //
                    //
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
                    //
                    //
                });
            });
            true
        });
        if res.is_err() {
            println!("fail on bluetooth device remove");
            return;
        }

        listeners.bluetooth_listener.store(true, Ordering::SeqCst);

        loop {
            let _ = conn.process(Duration::from_millis(1000));
            if !listeners.bluetooth_listener.load(Ordering::SeqCst) {
                println!("stopping bluetooth listener");
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
}

