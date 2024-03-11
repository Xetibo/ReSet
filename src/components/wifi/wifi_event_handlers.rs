use std::sync::Arc;

use adw::prelude::{ComboRowExt, PreferencesGroupExt, PreferencesRowExt};
use glib::{subclass::types::ObjectSubclassIsExt, PropertySet};
use gtk::prelude::WidgetExt;
use re_set_lib::{
    network::network_structures::WifiStrength,
    signals::{
        AccessPointAdded, AccessPointChanged, AccessPointRemoved, WifiDeviceChanged,
        WifiDeviceReset,
    },
};

use super::{wifi_box::WifiBox, wifi_entry::WifiEntry};

pub fn access_point_added_handler(wifi_box: Arc<WifiBox>, ir: AccessPointAdded) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let imp = wifi_box.imp();
            let mut wifi_entries = imp.wifi_entries.write().unwrap();
            let mut wifi_entries_path = imp.wifi_entries_path.write().unwrap();
            let ssid = ir.access_point.ssid.clone();
            let path = ir.access_point.dbus_path.clone();
            if wifi_entries.get(&ssid).is_some() || ssid.is_empty() {
                return;
            }
            let connected =
                imp.reset_current_wifi_device.borrow().active_access_point == ir.access_point.ssid;
            let entry = WifiEntry::new(connected, ir.access_point, imp);
            wifi_entries.insert(ssid, entry.clone());
            wifi_entries_path.insert(path, entry.clone());
            imp.reset_wifi_list.add(&*entry);
        });
    });
    true
}

pub fn access_point_removed_handler(wifi_box: Arc<WifiBox>, ir: AccessPointRemoved) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let imp = wifi_box.imp();
            let mut wifi_entries = imp.wifi_entries.write().unwrap();
            let mut wifi_entries_path = imp.wifi_entries_path.write().unwrap();
            let entry = wifi_entries_path.remove(&ir.access_point);
            if entry.is_none() {
                return;
            }
            let entry = entry.unwrap();
            let ssid = entry.imp().access_point.borrow().ssid.clone();
            wifi_entries.remove(&ssid);
            imp.reset_wifi_list.remove(&*entry);
        });
    });
    true
}

pub fn access_point_changed_handler(wifi_box: Arc<WifiBox>, ir: AccessPointChanged) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_local_once(move || {
            let imp = wifi_box.imp();
            let wifi_entries = imp.wifi_entries.read().unwrap();
            let entry = wifi_entries.get(&ir.access_point.ssid);
            if entry.is_none() {
                return;
            }
            let entry = entry.unwrap();
            let entry_imp = entry.imp();
            let strength = WifiStrength::from_u8(ir.access_point.strength);
            let ssid = ir.access_point.ssid.clone();
            let name_opt = String::from_utf8(ssid).unwrap_or_else(|_| String::from(""));
            let name = name_opt.as_str();
            entry_imp.wifi_strength.set(strength);
            entry.set_title(name);
            // TODO handle encryption thing
            entry_imp
                .reset_wifi_strength
                .borrow()
                .set_from_icon_name(match strength {
                    WifiStrength::Excellent => Some("network-wireless-signal-excellent-symbolic"),
                    WifiStrength::Ok => Some("network-wireless-signal-ok-symbolic"),
                    WifiStrength::Weak => Some("network-wireless-signal-weak-symbolic"),
                    WifiStrength::None => Some("network-wireless-signal-none-symbolic"),
                });
            if !ir.access_point.stored {
                entry_imp
                    .reset_wifi_edit_button
                    .borrow()
                    .set_sensitive(false);
            }
            if ir.access_point.ssid == imp.reset_current_wifi_device.borrow().active_access_point {
                entry_imp
                    .reset_wifi_connected
                    .borrow()
                    .set_text("Connected");
            } else {
                entry_imp.reset_wifi_connected.borrow().set_text("");
            }
            {
                let mut wifi_name = entry_imp.wifi_name.borrow_mut();
                *wifi_name = String::from(name);
            }
        });
    });
    true
}

pub fn wifi_device_changed_handler(wifi_box: Arc<WifiBox>, ir: WifiDeviceChanged) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let imp = wifi_box.imp();
            let mut current_device = imp.reset_current_wifi_device.borrow_mut();
            if current_device.path == ir.wifi_device.path {
                current_device.active_access_point = ir.wifi_device.active_access_point;
            } else {
                *current_device = ir.wifi_device;
            }
            let mut wifi_entries = imp.wifi_entries.write().unwrap();
            for entry in wifi_entries.iter_mut() {
                let imp = entry.1.imp();
                let mut connected = imp.connected.borrow_mut();
                *connected = imp.access_point.borrow().ssid == current_device.active_access_point;
                if *connected {
                    imp.reset_wifi_connected.borrow().set_text("Connected");
                } else {
                    imp.reset_wifi_connected.borrow().set_text("");
                }
            }
        });
    });
    true
}

pub fn wifi_device_reset_handler(wifi_box: Arc<WifiBox>, ir: WifiDeviceReset) -> bool {
    if ir.devices.is_empty() {
        return true;
    }
    {
        let imp = wifi_box.imp();
        let list = imp.reset_model_list.write().unwrap();
        let mut model_index = imp.reset_model_index.write().unwrap();
        let mut map = imp.reset_wifi_devices.write().unwrap();
        imp.reset_current_wifi_device
            .replace(ir.devices.last().unwrap().clone());
        for (index, device) in ir.devices.into_iter().enumerate() {
            list.append(&device.name);
            map.insert(device.name.clone(), (device, index as u32));
            *model_index += 1;
        }
    }
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let imp = wifi_box.imp();
            let list = imp.reset_model_list.read().unwrap();
            imp.reset_wifi_device.set_model(Some(&*list));
            let map = imp.reset_wifi_devices.read().unwrap();
            {
                let device = imp.reset_current_wifi_device.borrow();
                if let Some(index) = map.get(&device.name) {
                    imp.reset_wifi_device.set_selected(index.1);
                }
            }
        });
    });
    true
}
