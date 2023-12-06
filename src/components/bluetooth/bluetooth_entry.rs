use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use crate::components::bluetooth::bluetooth_entry_impl;
use adw::glib::Object;
use adw::{glib, ActionRow};
use adw::prelude::{ActionRowExt, PreferencesRowExt};
use dbus::blocking::Connection;
use dbus::{Error, Path};
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{ButtonExt, ListBoxRowExt, WidgetExt};
use gtk::{gio, GestureClick, Image, Button, Align};
use re_set_lib::bluetooth::bluetooth_structures::BluetoothDevice;

glib::wrapper! {
    pub struct BluetoothEntry(ObjectSubclass<bluetooth_entry_impl::BluetoothEntry>)
        @extends ActionRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget, gtk::ListBoxRow, adw::PreferencesRow;
}

unsafe impl Send for BluetoothEntry {}
unsafe impl Sync for BluetoothEntry {}

impl BluetoothEntry {
    pub fn new(device: &BluetoothDevice) -> Self {
        let entry: BluetoothEntry = Object::builder().build();
        let entry_imp = entry.imp();

        entry.set_title(&device.alias);
        entry.set_subtitle(&device.address);
        entry.set_activatable(true);

        entry_imp.remove_device_button.replace(Button::builder().icon_name("user-trash-symbolic").valign(Align::Center).build());
        entry.add_suffix(entry_imp.remove_device_button.borrow().deref());
        if device.icon.is_empty() {
            entry.add_prefix(&Image::from_icon_name("dialog-question-symbolic"));
        } else {
            entry.add_prefix(&Image::from_icon_name(&device.icon));
        }
        if device.connected || device.paired {
            entry_imp.remove_device_button.borrow().set_sensitive(true);
        } else {
            entry_imp.remove_device_button.borrow().set_sensitive(false);
        }
        let path = Arc::new(device.path.clone());
        entry_imp.remove_device_button.borrow().connect_clicked(move |_| {
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
