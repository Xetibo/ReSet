use std::sync::atomic::Ordering;

use std::sync::Arc;

use std::time::Duration;

use crate::components::base::error_impl::{show_error, ReSetErrorImpl};
use crate::components::base::utils::Listeners;
use crate::components::utils::{set_combo_row_ellipsis, BASE, DBUS_PATH, WIRELESS};
use adw::glib::Object;
use adw::prelude::{ComboRowExt, ListBoxRowExt, PreferencesGroupExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::blocking::Connection;
use dbus::message::SignalArgs;
use dbus::Error;
use dbus::Path;
use glib::{clone, Cast, ControlFlow, PropertySet};
use gtk::glib::Variant;
use gtk::prelude::ActionableExt;
use gtk::{gio, StringList, StringObject};
use re_set_lib::network::network_structures::{AccessPoint, WifiDevice};
use re_set_lib::signals::{AccessPointAdded, WifiDeviceChanged, WifiDeviceReset};
use re_set_lib::signals::{AccessPointChanged, AccessPointRemoved};

use crate::components::wifi::wifi_box_impl;
use crate::components::wifi::wifi_entry::WifiEntry;

use super::saved_wifi_entry::SavedWifiEntry;
use super::wifi_event_handlers::{
    access_point_added_handler, access_point_changed_handler, access_point_removed_handler,
    wifi_device_changed_handler, wifi_device_reset_handler,
};

glib::wrapper! {
    pub struct WifiBox(ObjectSubclass<wifi_box_impl::WifiBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

type ResultMap = Result<(Vec<(Path<'static>, Vec<u8>)>,), Error>;

unsafe impl Send for WifiBox {}
unsafe impl Sync for WifiBox {}

impl ReSetErrorImpl for WifiBox {
    fn error(
        &self,
    ) -> &gtk::subclass::prelude::TemplateChild<crate::components::base::error::ReSetError> {
        &self.imp().error
    }
}

impl WifiBox {
    pub fn new(listeners: Arc<Listeners>) -> Arc<Self> {
        let obj: Arc<WifiBox> = Arc::new(Object::builder().build());
        setup_callbacks(listeners, obj)
    }

    pub fn setup_callbacks(&self) {}
}

fn setup_callbacks(listeners: Arc<Listeners>, wifi_box: Arc<WifiBox>) -> Arc<WifiBox> {
    let imp = wifi_box.imp();
    let wifibox_ref = wifi_box.clone();
    let wifibox_ref_switch = wifi_box.clone();
    imp.reset_switch_initial.set(true);
    imp.reset_saved_networks.set_activatable(true);
    imp.reset_saved_networks
        .set_action_name(Some("navigation.push"));
    imp.reset_saved_networks
        .set_action_target_value(Some(&Variant::from("saved")));

    imp.reset_available_networks.set_activatable(true);
    imp.reset_available_networks
        .set_action_name(Some("navigation.pop"));
    set_combo_row_ellipsis(imp.reset_wifi_device.get());
    imp.reset_wifi_switch.connect_state_set(
        clone!(@weak imp => @default-return glib::Propagation::Proceed, move |_, value| {
            if imp.reset_switch_initial.load(Ordering::SeqCst) {
                return glib::Propagation::Proceed;
            }
            set_wifi_enabled(value, wifibox_ref_switch.clone());
            if !value {
                imp.reset_wifi_devices.write().unwrap().clear();
                *imp.reset_model_list.write().unwrap() = StringList::new(&[]);
                *imp.reset_model_index.write().unwrap() = 0;
                let mut map = imp.wifi_entries.write().unwrap();
                for entry in map.iter() {
                    imp.reset_wifi_list.remove(&*(*entry.1));
                }
                map.clear();
                imp.wifi_entries_path.write().unwrap().clear();
                listeners.wifi_listener.store(false, Ordering::SeqCst);
            } else {
                start_event_listener(listeners.clone(), wifibox_ref.clone());
                show_stored_connections(wifibox_ref.clone());
                scan_for_wifi(wifibox_ref.clone());
            }
            glib::Propagation::Proceed
        }),
    );

    wifi_box
}

pub fn scan_for_wifi(wifi_box: Arc<WifiBox>) {
    let wifibox_ref = wifi_box.clone();
    let wifi_entries = wifi_box.imp().wifi_entries.clone();
    let wifi_entries_path = wifi_box.imp().wifi_entries_path.clone();

    gio::spawn_blocking(move || {
        let wifi_status = get_wifi_status(wifibox_ref.clone());
        let devices = get_wifi_devices(wifibox_ref.clone());
        if devices.is_empty() {
            return;
        }
        let access_points = get_access_points(wifibox_ref.clone());
        {
            let imp = wifibox_ref.imp();
            let list = imp.reset_model_list.write().unwrap();
            let mut model_index = imp.reset_model_index.write().unwrap();
            let mut map = imp.reset_wifi_devices.write().unwrap();
            imp.reset_current_wifi_device
                .replace(devices.last().unwrap().clone());
            for (index, device) in devices.into_iter().enumerate() {
                list.append(&device.name);
                map.insert(device.name.clone(), (device, index as u32));
                *model_index += 1;
            }
        }
        let wifi_entries = wifi_entries.clone();
        let wifi_entries_path = wifi_entries_path.clone();
        dbus_start_network_events(wifibox_ref.clone());
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let mut wifi_entries = wifi_entries.write().unwrap();
                let mut wifi_entries_path = wifi_entries_path.write().unwrap();
                let imp = wifibox_ref.imp();

                imp.reset_wifi_switch.set_state(wifi_status);
                imp.reset_wifi_switch.set_active(wifi_status);
                imp.reset_switch_initial.set(false);

                let list = imp.reset_model_list.read().unwrap();
                imp.reset_wifi_device.set_model(Some(&*list));
                let map = imp.reset_wifi_devices.read().unwrap();
                {
                    let device = imp.reset_current_wifi_device.borrow();
                    if let Some(index) = map.get(&device.name) {
                        imp.reset_wifi_device.set_selected(index.1);
                    }
                }

                let device_changed_ref = wifibox_ref.clone();
                imp.reset_wifi_device
                    .connect_selected_notify(move |dropdown| {
                        select_wifi_device_handler(dropdown, device_changed_ref.clone());
                    });
                for access_point in access_points {
                    if access_point.ssid.is_empty() {
                        continue;
                    }
                    let ssid = access_point.ssid.clone();
                    let path = access_point.dbus_path.clone();
                    let connected =
                        imp.reset_current_wifi_device.borrow().active_access_point == ssid;
                    let entry = WifiEntry::new(connected, access_point, imp);
                    wifi_entries.insert(ssid, entry.clone());
                    wifi_entries_path.insert(path, entry.clone());
                    imp.reset_wifi_list.add(&*entry);
                }
            });
        });
    });
}

fn select_wifi_device_handler(dropdown: &adw::ComboRow, wifi_box: Arc<WifiBox>) -> ControlFlow {
    let selected = dropdown.selected_item();
    if selected.is_none() {
        return ControlFlow::Break;
    }
    let selected = selected.unwrap();
    let selected = selected.downcast_ref::<StringObject>().unwrap();
    let selected = selected.string().to_string();
    let imp = wifi_box.imp();
    let device = imp.reset_wifi_devices.read().unwrap();
    let device = device.get(&selected);
    if device.is_none() {
        return ControlFlow::Break;
    }
    set_wifi_device(device.unwrap().0.path.clone(), wifi_box.clone());

    ControlFlow::Continue
}

pub fn show_stored_connections(wifi_box: Arc<WifiBox>) {
    let wifibox_ref = wifi_box.clone();
    gio::spawn_blocking(move || {
        let connections = get_stored_connections(wifi_box.clone());
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let self_imp = wifibox_ref.imp();
                for connection in connections {
                    // TODO include button for settings
                    let name =
                        &String::from_utf8(connection.1).unwrap_or_else(|_| String::from(""));
                    let entry = SavedWifiEntry::new(name, connection.0, self_imp);
                    self_imp.reset_stored_wifi_list.add(&*entry);
                }
            });
        });
    });
}

pub fn dbus_start_network_events(wifi_box: Arc<WifiBox>) {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(), Error> = proxy.method_call(WIRELESS, "StartNetworkListener", ());
    if res.is_err() {
        show_error::<WifiBox>(wifi_box.clone(), "Failed to start Network listener");
    }
}

pub fn get_access_points(wifi_box: Arc<WifiBox>) -> Vec<AccessPoint> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Vec<AccessPoint>,), Error> =
        proxy.method_call(WIRELESS, "ListAccessPoints", ());
    if res.is_err() {
        show_error::<WifiBox>(wifi_box.clone(), "Failed to list access points");
        return Vec::new();
    }
    let (access_points,) = res.unwrap();
    access_points
}

pub fn set_wifi_device(path: Path<'static>, wifi_box: Arc<WifiBox>) {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(bool,), Error> = proxy.method_call(WIRELESS, "SetWifiDevice", (path,));
    if res.is_err() {
        show_error::<WifiBox>(wifi_box.clone(), "Failed to set WiFi devices");
    }
}

pub fn get_wifi_devices(wifi_box: Arc<WifiBox>) -> Vec<WifiDevice> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Vec<WifiDevice>,), Error> =
        proxy.method_call(WIRELESS, "GetAllWifiDevices", ());
    if res.is_err() {
        show_error::<WifiBox>(wifi_box.clone(), "Failed to get WiFi devices");
        return Vec::new();
    }
    let (devices,) = res.unwrap();
    devices
}

pub fn get_wifi_status(wifi_box: Arc<WifiBox>) -> bool {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(bool,), Error> = proxy.method_call(WIRELESS, "GetWifiStatus", ());
    if res.is_err() {
        show_error::<WifiBox>(wifi_box.clone(), "Failed to get WiFi status");
        return false;
    }
    res.unwrap().0
}

pub fn get_stored_connections(wifi_box: Arc<WifiBox>) -> Vec<(Path<'static>, Vec<u8>)> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: ResultMap = proxy.method_call(WIRELESS, "ListStoredConnections", ());
    if res.is_err() {
        show_error::<WifiBox>(wifi_box.clone(), "Failed to list stored connections");
        return Vec::new();
    }
    let (connections,) = res.unwrap();
    connections
}

pub fn set_wifi_enabled(enabled: bool, wifi_box: Arc<WifiBox>) {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(bool,), Error> = proxy.method_call(WIRELESS, "SetWifiEnabled", (enabled,));
    if res.is_err() {
        show_error::<WifiBox>(wifi_box.clone(), "Failed to enable WiFi");
    }
}

pub fn start_event_listener(listeners: Arc<Listeners>, wifi_box: Arc<WifiBox>) {
    gio::spawn_blocking(move || {
        if listeners.wifi_disabled.load(Ordering::SeqCst)
            || listeners.wifi_listener.load(Ordering::SeqCst)
        {
            return;
        }
        listeners.wifi_listener.store(true, Ordering::SeqCst);

        let conn = Connection::new_session().unwrap();
        let added_ref = wifi_box.clone();
        let removed_ref = wifi_box.clone();
        let changed_ref = wifi_box.clone();
        let wifi_changed_ref = wifi_box.clone();
        let wifi_reset_ref = wifi_box.clone();
        let access_point_added =
            AccessPointAdded::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH)))
                .static_clone();
        let access_point_removed =
            AccessPointRemoved::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH)))
                .static_clone();
        let access_point_changed =
            AccessPointChanged::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH)))
                .static_clone();
        let device_changed =
            WifiDeviceChanged::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH)))
                .static_clone();
        let devices_reset =
            WifiDeviceReset::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH)))
                .static_clone();
        let res = conn.add_match(access_point_added, move |ir: AccessPointAdded, _, _| {
            access_point_added_handler(added_ref.clone(), ir)
        });
        if res.is_err() {
            println!("fail on access point add event");
            return;
        }
        let res = conn.add_match(access_point_removed, move |ir: AccessPointRemoved, _, _| {
            access_point_removed_handler(removed_ref.clone(), ir)
        });
        if res.is_err() {
            println!("fail on access point remove event");
            return;
        }
        let res = conn.add_match(access_point_changed, move |ir: AccessPointChanged, _, _| {
            access_point_changed_handler(changed_ref.clone(), ir)
        });
        if res.is_err() {
            println!("fail on access point change event");
            return;
        }
        let res = conn.add_match(device_changed, move |ir: WifiDeviceChanged, _, _| {
            wifi_device_changed_handler(wifi_changed_ref.clone(), ir)
        });
        if res.is_err() {
            println!("fail on wifi device change event");
            return;
        }
        let res = conn.add_match(devices_reset, move |ir: WifiDeviceReset, _, _| {
            wifi_device_reset_handler(wifi_reset_ref.clone(), ir)
        });
        if res.is_err() {
            println!("fail on wifi device change event");
            return;
        }

        loop {
            let _ = conn.process(Duration::from_millis(1000));
            if !listeners.wifi_listener.load(Ordering::SeqCst) {
                break;
            }
        }
    });
}
