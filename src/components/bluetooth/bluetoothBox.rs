use adw::glib;
use adw::glib::Object;
use adw::subclass::prelude::ObjectSubclassIsExt;

use crate::components::bluetooth::bluetoothBoxImpl;
use crate::components::bluetooth::bluetoothEntry::BluetoothEntry;
use crate::components::bluetooth::bluetoothEntryImpl::DeviceTypes;

glib::wrapper! {
    pub struct BluetoothBox(ObjectSubclass<bluetoothBoxImpl::BluetoothBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}


impl BluetoothBox {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn scanForDevices(&self) {
        let selfImp = self.imp();
        let mut wifiEntries = selfImp.availableDevices.borrow_mut();
        wifiEntries.push(BluetoothEntry::new(DeviceTypes::Mouse, "ina mouse"));
        wifiEntries.push(BluetoothEntry::new(DeviceTypes::Keyboard, "inaboard"));
        wifiEntries.push(BluetoothEntry::new(DeviceTypes::Controller, "ina controller"));
        wifiEntries.push(BluetoothEntry::new(DeviceTypes::Controller, "ina best waifu"));

        for wifiEntry in wifiEntries.iter() {
            selfImp.resetBluetoothAvailableDevices.append(wifiEntry);
        }
    }

    pub fn addConnectedDevices(&self) {
        let selfImp = self.imp();
        let mut wifiEntries = selfImp.connectedDevices.borrow_mut();
        wifiEntries.push(BluetoothEntry::new(DeviceTypes::Mouse, "why are we still here?"));
        wifiEntries.push(BluetoothEntry::new(DeviceTypes::Keyboard, "just to suffer?"));

        for wifiEntry in wifiEntries.iter() {
            selfImp.resetBluetoothConnectedDevices.append(wifiEntry);
        }
    }
}