use std::sync::Arc;

use adw::prelude::PreferencesGroupExt;
use glib::subclass::types::ObjectSubclassIsExt;
use gtk::prelude::WidgetExt;
use re_set_lib::signals::{BluetoothDeviceAdded, BluetoothDeviceChanged, BluetoothDeviceRemoved};

use super::{bluetooth_box::BluetoothBox, bluetooth_entry::BluetoothEntry};

pub fn device_changed_handler(
    device_changed_box: Arc<BluetoothBox>,
    ir: BluetoothDeviceChanged,
) -> bool {
    let bluetooth_box = device_changed_box.clone();
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let imp = bluetooth_box.imp();
            let mut map = imp.available_devices.borrow_mut();
            if let Some(list_entry) = map.get_mut(&ir.bluetooth_device.path) {
                let mut existing_bluetooth_device = list_entry.imp().bluetooth_device.borrow_mut();
                if existing_bluetooth_device.connected != ir.bluetooth_device.connected {
                    if ir.bluetooth_device.connected {
                        imp.reset_bluetooth_available_devices.remove(&**list_entry);
                        imp.reset_bluetooth_connected_devices.add(&**list_entry);
                    } else {
                        imp.reset_bluetooth_connected_devices.remove(&**list_entry);
                        imp.reset_bluetooth_available_devices.add(&**list_entry);
                    }
                }
                if existing_bluetooth_device.bonded != ir.bluetooth_device.bonded {
                    if ir.bluetooth_device.bonded {
                        list_entry
                            .imp()
                            .remove_device_button
                            .borrow()
                            .set_sensitive(true);
                    } else {
                        list_entry
                            .imp()
                            .remove_device_button
                            .borrow()
                            .set_sensitive(false);
                    }
                }
                *existing_bluetooth_device = ir.bluetooth_device;
            }
        });
    });
    true
}

pub fn device_removed_handler(
    device_removed_box: Arc<BluetoothBox>,
    ir: BluetoothDeviceRemoved,
) -> bool {
    let bluetooth_box = device_removed_box.clone();
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let imp = bluetooth_box.imp();
            let mut map = imp.available_devices.borrow_mut();
            if let Some(list_entry) = map.remove(&ir.bluetooth_device) {
                if list_entry.imp().bluetooth_device.borrow().connected {
                    imp.reset_bluetooth_connected_devices.remove(&*list_entry);
                } else {
                    imp.reset_bluetooth_available_devices.remove(&*list_entry);
                }
            }
        });
    });
    true
}

pub fn device_added_handler(device_added_box: Arc<BluetoothBox>, ir: BluetoothDeviceAdded) -> bool {
    let bluetooth_box = device_added_box.clone();
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let imp = bluetooth_box.imp();
            let path = ir.bluetooth_device.path.clone();
            let connected = ir.bluetooth_device.connected;
            let bluetooth_entry = BluetoothEntry::new(ir.bluetooth_device, bluetooth_box.clone());
            imp.available_devices
                .borrow_mut()
                .insert(path, bluetooth_entry.clone());
            if connected {
                imp.reset_bluetooth_connected_devices.add(&*bluetooth_entry);
            } else {
                imp.reset_bluetooth_available_devices.add(&*bluetooth_entry);
            }
        });
    });
    true
}
