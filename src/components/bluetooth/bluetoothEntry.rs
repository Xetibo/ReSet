use adw::glib;
use adw::glib::Object;
use crate::components::bluetooth::bluetoothEntryImpl;

glib::wrapper! {
    pub struct BluetoothEntry(ObjectSubclass<bluetoothEntryImpl::BluetoothEntry>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget;
}

impl BluetoothEntry {
    pub fn new() -> Self {
        Object::builder().build()
    }
}