use std::sync::Arc;
use std::time::Duration;

use crate::components::bluetooth::bluetoothEntryImpl;
use adw::glib;
use adw::glib::Object;
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::blocking::Connection;
use dbus::{Error, Path};
use gtk::prelude::{ButtonExt, WidgetExt};
use gtk::{gio, GestureClick};
use ReSet_Lib::bluetooth::bluetooth::BluetoothDevice;

glib::wrapper! {
    pub struct BluetoothEntry(ObjectSubclass<bluetoothEntryImpl::BluetoothEntry>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget;
}

unsafe impl Send for BluetoothEntry {}
unsafe impl Sync for BluetoothEntry {}

impl BluetoothEntry {
    pub fn new(device: &BluetoothDevice) -> Self {
        let entry: BluetoothEntry = Object::builder().build();
        let entryImp = entry.imp();
        entryImp.resetBluetoothLabel.get().set_text(&device.alias);
        entryImp
            .resetBluetoothAddress
            .get()
            .set_text(&device.address);
        if device.icon.is_empty() {
            entryImp
                .resetBluetoothDeviceType
                .set_icon_name(Some("dialog-question-symbolic"));
        } else {
            entryImp
                .resetBluetoothDeviceType
                .set_icon_name(Some(&device.icon));
        }
        if device.paired {
            entryImp.resetBluetoothButton.set_sensitive(true);
        } else {
            entryImp.resetBluetoothButton.set_sensitive(false);
        }
        let path = Arc::new(device.path.clone());
        entryImp.resetBluetoothButton.connect_clicked(move |_| {
            remove_device_pairing((*path).clone());
        });
        let gesture = GestureClick::new();
        let connected = device.connected;
        // let paired = device.paired;
        // paired is not what we think
        // TODO implement paired
        let path = device.path.clone();
        gesture.connect_released(move |_, _, _, _| {
            connect_to_device(path.clone());
            if connected {
                disconnect_from_device(path.clone());
            } else {
                connect_to_device(path.clone());
            }
        });
        entry.add_controller(gesture);
        entry
    }
}

fn connect_to_device(path: Path<'static>) {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSetDaemon",
            "/org/Xetibo/ReSetDaemon",
            Duration::from_millis(1000),
        );
        let _: Result<(bool,), Error> = proxy.method_call(
            "org.Xetibo.ReSetBluetooth",
            "ConnectToBluetoothDevice",
            (path,),
        );
    });
}

// fn pair_with_device(path: Path<'static>) {
//     gio::spawn_blocking(move || {
//         let conn = Connection::new_session().unwrap();
//         let proxy = conn.with_proxy(
//             "org.Xetibo.ReSetDaemon",
//             "/org/Xetibo/ReSetDaemon",
//             Duration::from_millis(1000),
//         );
//         let _: Result<(bool,), Error> = proxy.method_call(
//             "org.Xetibo.ReSetBluetooth",
//             "PairWithBluetoothDevice",
//             (path,),
//         );
//     });
// }

fn disconnect_from_device(path: Path<'static>) {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSetDaemon",
            "/org/Xetibo/ReSetDaemon",
            Duration::from_millis(1000),
        );
        let _: Result<(bool,), Error> = proxy.method_call(
            "org.Xetibo.ReSetBluetooth",
            "DisconnectFromBluetoothDevice",
            (path,),
        );
    });
}

fn remove_device_pairing(path: Path<'static>) {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSetDaemon",
            "/org/Xetibo/ReSetDaemon",
            Duration::from_millis(1000),
        );
        let _: Result<(bool,), Error> =
            proxy.method_call("org.Xetibo.ReSetBluetooth", "RemoveDevicePairing", (path,));
    });
}
