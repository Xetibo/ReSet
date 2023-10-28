#![allow(non_snake_case)]
mod bluetoothBox;
mod bluetoothEntry;

use adw::glib::Object;
use gtk::{glib};

glib::wrapper! {
    pub struct BluetoothBox(ObjectSubclass<bluetoothBox::BluetoothBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

glib::wrapper! {
    pub struct BluetoothEntry(ObjectSubclass<bluetoothEntry::BluetoothEntry>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget;
}

impl BluetoothBox {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl BluetoothEntry {
    pub fn new() -> Self {
        Object::builder().build()
    }
}