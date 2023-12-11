use std::time::Duration;

use crate::components::wifi::saved_wifi_entry_impl;
use crate::components::wifi::wifi_box_impl::WifiBox;
use adw::glib;
use adw::glib::Object;
use adw::prelude::{ButtonExt, WidgetExt};
use dbus::blocking::Connection;
use dbus::{Error, Path};
use glib::subclass::types::ObjectSubclassIsExt;
use glib::{clone, PropertySet};
use gtk::gio;
use gtk::prelude::ListBoxRowExt;

glib::wrapper! {
    pub struct SavedWifiEntry(ObjectSubclass<saved_wifi_entry_impl::SavedWifiEntry>)
        @extends adw::ActionRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget, gtk::ListBoxRow;
}

impl SavedWifiEntry {
    pub fn new(name: &str, path: Path<'static>, wifi_box: &WifiBox) -> Self {
        let entry: SavedWifiEntry = Object::builder().build();
        entry.set_activatable(false);
        let entry_imp = entry.imp();

        entry_imp.reset_edit_saved_wifi_button.connect_clicked(
            clone!(@ weak entry_imp, @ weak wifi_box => move |_| {
                // TODO accesspoint has to be saved somewhere i guess
                // let _option = getConnectionSettings(entryImp.accessPoint.borrow().associated_connection.clone());
                // wifiBox.resetWifiNavigation.push(&*WifiOptions::new(_option));
            }),
        );

        entry_imp.reset_saved_wifi_label.set_text(name);
        entry_imp.reset_connection_path.set(path);
        entry_imp.reset_delete_saved_wifi_button.connect_clicked(
            clone!(@weak entry as entry => move |_| {
            delete_connection(entry.imp().reset_connection_path.take());
            // TODO handle error
            let parent = entry.parent().unwrap();
            parent.set_visible(false);
            parent.unparent();
            }),
        );
        entry
    }
}

fn delete_connection(path: Path<'static>) {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSetDaemon",
            "/org/Xetibo/ReSetDaemon",
            Duration::from_millis(1000),
        );
        let _: Result<(), Error> =
            proxy.method_call("org.Xetibo.ReSetWireless", "DeleteConnection", (path,));
    });
}
