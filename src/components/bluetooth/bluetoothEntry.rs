use adw::glib;
use adw::glib::Object;
use adw::subclass::prelude::ObjectSubclassIsExt;
use crate::components::bluetooth::bluetoothEntryImpl;
use crate::components::bluetooth::bluetoothEntryImpl::DeviceTypes;

glib::wrapper! {
    pub struct BluetoothEntry(ObjectSubclass<bluetoothEntryImpl::BluetoothEntry>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget;
}

impl BluetoothEntry {
    pub fn new(deviceType: DeviceTypes, name: &str) -> Self {
        let entry: BluetoothEntry = Object::builder().build();
        let entryImp = entry.imp();
        entryImp.resetBluetoothLabel.get().set_text(name);
        entryImp.resetBluetoothDeviceType.get().set_from_icon_name(match deviceType {
            DeviceTypes::Mouse => Some("input-mouse-symbolic"),
            DeviceTypes::Keyboard => Some("input-keyboard-symbolic"),
            DeviceTypes::Headset => Some("output-headset-symbolic"),
            DeviceTypes::Controller => Some("input-gaming-symbolic"),
            DeviceTypes::None => Some("text-x-generic-symbolic") // no generic bluetooth device icon found
        });
        {
            let mut wifiName = entryImp.deviceName.borrow_mut();
            *wifiName = String::from(name);
        }
        entry
    }
}