#![allow(non_snake_case)]

mod wifiBox;
mod wifiEntry;

use adw::glib::Object;
use gtk::{glib};
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::*;

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
}

impl WifiEntry {
    fn onClick(&self) {
        self.imp().resetWifiButton.set_tooltip_text(Option::from("asd"));
    }
}