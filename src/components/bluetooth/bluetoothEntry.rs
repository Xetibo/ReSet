use std::time::Duration;

use crate::components::bluetooth::bluetoothEntryImpl;
use crate::components::bluetooth::bluetoothEntryImpl::DeviceTypes;
use adw::glib;
use adw::glib::Object;
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::blocking::Connection;
use dbus::{Error, Path};
use glib::clone;
use gtk::prelude::WidgetExt;
use gtk::{gio, GestureClick};
use ReSet_Lib::bluetooth::bluetooth::BluetoothDevice;

glib::wrapper! {
    pub struct BluetoothEntry(ObjectSubclass<bluetoothEntryImpl::BluetoothEntry>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget;
}

impl BluetoothEntry {
    pub fn new(device: BluetoothDevice) -> Self {
        let entry: BluetoothEntry = Object::builder().build();
        let entryImp = entry.imp();
        entryImp.resetBluetoothLabel.get().set_text(&device.name);
        entryImp.resetBluetoothAddress.get().set_text(&device.address);
        // entryImp
        //     .resetBluetoothDeviceType
        //     .get()
        //     .set_from_icon_name(match deviceType {
        //         DeviceTypes::Mouse => Some("input-mouse-symbolic"),
        //         DeviceTypes::Keyboard => Some("input-keyboard-symbolic"),
        //         DeviceTypes::Headset => Some("output-headset-symbolic"),
        //         DeviceTypes::Controller => Some("input-gaming-symbolic"),
        //         DeviceTypes::None => Some("text-x-generic-symbolic"), // no generic bluetooth device icon found
        //     });
        let gesture = GestureClick::new();
        let connected = false;
        // TODO implement this connected
        entryImp.device.replace(device);
        gesture.connect_released(clone!(@weak entryImp => move |_, _, _, _| {
            let device = entryImp.device.borrow_mut();
            if connected {
                disconnect_from_device(device.path.clone());
            } else if device.paired {
                connect_to_device(device.path.clone());
            } else {
                pair_with_device(device.path.clone());
            }
        }));
        entry.add_controller(gesture);
        entry
    }
}

fn connect_to_device(path: Path<'static>) {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.xetibo.ReSet",
            "/org/xetibo/ReSet",
            Duration::from_millis(1000),
        );
        let _: Result<(bool,), Error> =
            proxy.method_call("org.xetibo.ReSet", "ConnectToBluetoothDevice", (path,));
    });
}

fn pair_with_device(path: Path<'static>) {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.xetibo.ReSet",
            "/org/xetibo/ReSet",
            Duration::from_millis(1000),
        );
        let _: Result<(bool,), Error> =
            proxy.method_call("org.xetibo.ReSet", "PairWithBluetoothDevice", (path,));
    });
}

fn disconnect_from_device(path: Path<'static>) {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.xetibo.ReSet",
            "/org/xetibo/ReSet",
            Duration::from_millis(1000),
        );
        let _: Result<(bool,), Error> =
            proxy.method_call("org.xetibo.ReSet", "DisconnectFromBluetoothDevice", (path,));
    });
}
