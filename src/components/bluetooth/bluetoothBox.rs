use adw::glib;
use adw::glib::Object;
use crate::components::bluetooth::bluetoothBoxImpl;

glib::wrapper! {
    pub struct BluetoothBox(ObjectSubclass<bluetoothBoxImpl::BluetoothBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}


impl BluetoothBox {
    pub fn new() -> Self {
        Object::builder().build()
    }
}