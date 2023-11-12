use std::sync::Arc;
use std::time::Duration;

use crate::components::wifi::savedWifiEntryImpl;
use adw::glib;
use adw::glib::Object;
use adw::prelude::{ButtonExt, WidgetExt};
use dbus::{Error, Path};
use dbus::blocking::Connection;
use glib::{clone, PropertySet};
use glib::subclass::types::ObjectSubclassIsExt;

glib::wrapper! {
    pub struct SavedWifiEntry(ObjectSubclass<savedWifiEntryImpl::SavedWifiEntry>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget;
}

impl SavedWifiEntry {
    pub fn new(name: &String, path: Path<'static>) -> Self {
        let entry: SavedWifiEntry = Object::builder().build();
        let entryImp = entry.imp();
        // TODO handle edit
        entryImp.resetSavedWifiLabel.set_text(name);
        entryImp.resetConnectionPath.set(path);
        entryImp.resetDeleteSavedWifiButton.connect_clicked(clone!(@weak entry as entry => move |_| {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.xetibo.ReSet",
            "/org/xetibo/ReSet",
            Duration::from_millis(1000),
        );
        let res: Result<(bool,), Error> = proxy.method_call("org.xetibo.ReSet", "DeleteConnection", (entry.imp().resetConnectionPath.take(),));
        if res.is_err() || res.unwrap() == (false,) { 
            // TODO handle error -> inform user
            println!("no worky");
           return; 
        }
        println!("worked, should be ded");
        let parent = entry.parent().unwrap();
        parent.set_visible(false);
        parent.unparent();
        }));
        entry
    }
}
