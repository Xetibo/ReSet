use std::rc::Rc;
use std::time::Duration;

use crate::components::utils::{BASE, DBUS_PATH, WIRELESS};
use crate::components::wifi::saved_wifi_entry_impl;
use crate::components::wifi::utils::get_connection_settings;
use crate::components::wifi::wifi_box_impl::WifiBox;
use crate::components::wifi::wifi_options::WifiOptions;
use adw::glib::Object;
use adw::prelude::{ActionRowExt, ButtonExt, PreferencesGroupExt, PreferencesRowExt};
use dbus::blocking::Connection;
use dbus::{Error, Path};
use glib::clone;
use glib::property::PropertySet;
use glib::subclass::types::ObjectSubclassIsExt;
use gtk::prelude::{BoxExt, ListBoxRowExt};
use gtk::{gio, Align, Button, Orientation};

glib::wrapper! {
    pub struct SavedWifiEntry(ObjectSubclass<saved_wifi_entry_impl::SavedWifiEntry>)
        @extends adw::ActionRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget, adw::PreferencesRow, gtk::ListBoxRow;
}

impl SavedWifiEntry {
    pub fn new(name: &str, path: Path<'static>, wifi_box: &WifiBox) -> Rc<Self> {
        let entry: Rc<SavedWifiEntry> = Rc::new(Object::builder().build());
        entry.set_activatable(false);
        let entry_imp = entry.imp();

        entry.set_title(name);
        entry_imp.reset_connection_path.set(path);

        let edit_button = Button::builder()
            .icon_name("document-edit-symbolic")
            .valign(Align::Center)
            .build();
        let delete_button = Button::builder()
            .icon_name("user-trash-symbolic")
            .valign(Align::Center)
            .build();

        let suffix_box = gtk::Box::new(Orientation::Horizontal, 5);
        suffix_box.append(&edit_button);
        suffix_box.append(&delete_button);
        entry.add_suffix(&suffix_box);

        edit_button.connect_clicked(
            clone!(@ weak entry_imp, @ weak wifi_box => move |_| {
                let _option = get_connection_settings(entry_imp.reset_connection_path.borrow().clone());
                wifi_box.reset_wifi_navigation.push(&*WifiOptions::new(_option, entry_imp.reset_connection_path.borrow().clone()));
            }),
        );

        let entry_ref = entry.clone();
        delete_button.connect_clicked(clone!(@weak wifi_box => move |_| {
            delete_connection(entry_ref.imp().reset_connection_path.take());
            // FUTURE TODO: handle error
            wifi_box.reset_stored_wifi_list.remove(&*entry_ref);

        }));
        entry
    }
}

fn delete_connection(path: Path<'static>) {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
        let _: Result<(), Error> = proxy.method_call(WIRELESS, "DeleteConnection", (path,));
    });
}
