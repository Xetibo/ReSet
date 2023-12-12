use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use crate::components::bluetooth::bluetooth_entry_impl;
use adw::glib::Object;
use adw::prelude::{ActionRowExt, PreferencesRowExt};
use adw::{glib, ActionRow};
use dbus::blocking::Connection;
use dbus::{Error, Path};
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{ButtonExt, ListBoxRowExt, WidgetExt};
use gtk::{gio, Align, Button, GestureClick, Image, Label};
use re_set_lib::bluetooth::bluetooth_structures::BluetoothDevice;

glib::wrapper! {
    pub struct BluetoothEntry(ObjectSubclass<bluetooth_entry_impl::BluetoothEntry>)
        @extends ActionRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget, gtk::ListBoxRow, adw::PreferencesRow;
}

unsafe impl Send for BluetoothEntry {}
unsafe impl Sync for BluetoothEntry {}

impl BluetoothEntry {
    pub fn new(device: BluetoothDevice) -> Arc<Self> {
        let entry: Arc<BluetoothEntry> = Arc::new(Object::builder().build());
        let entry_imp = entry.imp();
        let entry_ref = entry.clone();
        let entry_ref_remove = entry.clone();
        entry.set_title(&device.alias);
        entry.set_subtitle(&device.address);
        entry.set_activatable(true);

        entry_imp.remove_device_button.replace(
            Button::builder()
                .icon_name("user-trash-symbolic")
                .valign(Align::Center)
                .build(),
        );
        entry_imp
            .connecting_label
            .replace(Label::builder().label("").build());
        entry.add_suffix(entry_imp.remove_device_button.borrow().deref());
        if device.icon.is_empty() {
            entry.add_prefix(&Image::from_icon_name("dialog-question-symbolic"));
        } else {
            entry.add_prefix(&Image::from_icon_name(&device.icon));
        }
        if device.connected || device.bonded {
            entry_imp.remove_device_button.borrow().set_sensitive(true);
        } else {
            entry_imp.remove_device_button.borrow().set_sensitive(false);
        }

        entry_imp.bluetooth_device.replace(device);
        entry_imp
            .remove_device_button
            .borrow()
            .connect_clicked(move |_| {
                let imp = entry_ref_remove.imp();
                remove_device_pairing(imp.bluetooth_device.borrow().path.clone());
            });
        let gesture = GestureClick::new();
        // paired is not what we think
        // TODO implement paired
        gesture.connect_released(move |_, _, _, _| {
            let imp = entry_ref.imp();
            let borrow = imp.bluetooth_device.borrow();
            if borrow.connected {
                let imp = entry_ref.imp();
                imp.remove_device_button.borrow().set_sensitive(false);
                imp.connecting_label.borrow().set_text("Disconnecting...");
                disconnect_from_device(entry_ref.clone(), borrow.path.clone());
            } else {
                entry_ref.set_sensitive(false);
                imp.connecting_label.borrow().set_text("Connecting...");
                connect_to_device(entry_ref.clone(), borrow.path.clone());
            }
        });
        entry.add_controller(gesture);
        entry
    }
}

fn connect_to_device(entry: Arc<BluetoothEntry>, path: Path<'static>) {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSet.Daemon",
            "/org/Xetibo/ReSet/Daemon",
            Duration::from_millis(1000),
        );
        let res: Result<(bool,), Error> = proxy.method_call(
            "org.Xetibo.ReSet.Bluetooth",
            "ConnectToBluetoothDevice",
            (path,),
        );
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                if res.is_err() {
                    entry.set_sensitive(true);
                    entry
                        .imp()
                        .connecting_label
                        .borrow()
                        .set_text("Error on connecting");
                } else {
                    entry.set_sensitive(true);
                    entry.imp().connecting_label.borrow().set_text("");
                }
            });
        });
    });
}

// fn pair_with_device(path: Path<'static>) {
//     gio::spawn_blocking(move || {
//         let conn = Connection::new_session().unwrap();
//         let proxy = conn.with_proxy(
//             "org.Xetibo.ReSet.Daemon",
//             "/org/Xetibo/ReSet/Daemon",
//             Duration::from_millis(1000),
//         );
//         let _: Result<(bool,), Error> = proxy.method_call(
//             "org.Xetibo.ReSet.Bluetooth",
//             "PairWithBluetoothDevice",
//             (path,),
//         );
//     });
// }

fn disconnect_from_device(entry: Arc<BluetoothEntry>, path: Path<'static>) {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSet.Daemon",
            "/org/Xetibo/ReSet/Daemon",
            Duration::from_millis(1000),
        );
        let res: Result<(bool,), Error> = proxy.method_call(
            "org.Xetibo.ReSet.Bluetooth",
            "DisconnectFromBluetoothDevice",
            (path,),
        );
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let imp = entry.imp();
                if res.is_err() {
                    imp.remove_device_button.borrow().set_sensitive(true);
                    imp.connecting_label
                        .borrow()
                        .set_text("Error on disconnecting");
                } else {
                    imp.remove_device_button.borrow().set_sensitive(true);
                    imp.connecting_label.borrow().set_text("");
                }
            });
        });
    });
}

fn remove_device_pairing(path: Path<'static>) {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSet.Daemon",
            "/org/Xetibo/ReSet/Daemon",
            Duration::from_millis(1000),
        );
        let _: Result<(bool,), Error> =
            proxy.method_call("org.Xetibo.ReSet.Bluetooth", "RemoveDevicePairing", (path,));
    });
}
