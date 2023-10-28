#![allow(non_snake_case)]

use std::thread;
use std::time::Duration;

use adw::glib::Object;
use dbus::blocking::Connection;
use dbus::Error;
use gtk::glib;

mod wifiBox;
mod wifiEntry;

glib::wrapper! {
    pub struct WifiBox(ObjectSubclass<wifiBox::WifiBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

glib::wrapper! {
    pub struct WifiEntry(ObjectSubclass<wifiEntry::WifiEntry>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget;
}

impl WifiBox {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn donotdisturb() {
        thread::spawn(|| {
            let conn = Connection::new_session().unwrap();
            let proxy = conn.with_proxy(
                "org.freedesktop.Notifications",
                "/org/freedesktop/Notifications",
                Duration::from_millis(1000),
            );
            let _ : Result<(), Error> = proxy.method_call("org.freedesktop.Notifications", "DoNotDisturb", ());
        });
    }
}

impl WifiEntry {
    pub fn new() -> Self {
        Object::builder().build()
    }
}