use std::sync::atomic::Ordering;

use std::sync::Arc;

use std::time::Duration;

use crate::components::base::utils::Listeners;
use crate::components::utils::set_combo_row_ellipsis;
use adw::glib;
use adw::glib::Object;
use adw::prelude::{ComboRowExt, ListBoxRowExt, PreferencesGroupExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::blocking::Connection;
use dbus::message::SignalArgs;
use dbus::Error;
use dbus::Path;
use glib::{clone, Cast, PropertySet};
use gtk::glib::Variant;
use gtk::prelude::{ActionableExt, WidgetExt};
use gtk::{gio, StringObject};
use ReSet_Lib::network::network::{AccessPoint, WifiDevice, WifiStrength};
use ReSet_Lib::signals::{AccessPointAdded, WifiDeviceChanged};
use ReSet_Lib::signals::{AccessPointChanged, AccessPointRemoved};

use crate::components::wifi::wifi_box_impl;
use crate::components::wifi::wifi_entry::WifiEntry;

use super::saved_wifi_entry::SavedWifiEntry;

glib::wrapper! {
    pub struct WifiBox(ObjectSubclass<wifi_box_impl::WifiBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

type ResultMap = Result<(Vec<(Path<'static>, Vec<u8>)>,), Error>;

unsafe impl Send for WifiBox {}
unsafe impl Sync for WifiBox {}

impl WifiBox {
    pub fn new(listeners: Arc<Listeners>) -> Arc<Self> {
        let obj: Arc<WifiBox> = Arc::new(Object::builder().build());
        setup_callbacks(listeners, obj)
    }

    pub fn setup_callbacks(&self) {}
}

fn setup_callbacks(listeners: Arc<Listeners>, wifi_box: Arc<WifiBox>) -> Arc<WifiBox> {
    let wifi_status = get_wifi_status();
    let imp = wifi_box.imp();
    let wifibox_ref = wifi_box.clone();
    imp.reset_saved_networks.set_activatable(true);
    imp.reset_saved_networks
        .set_action_name(Some("navigation.push"));
    imp.reset_saved_networks
        .set_action_target_value(Some(&Variant::from("saved")));

    println!("{wifi_status}");
    imp.reset_wifi_switch.set_active(wifi_status);
    imp.reset_wifi_switch.set_state(wifi_status);

    imp.reset_available_networks.set_activatable(true);
    imp.reset_available_networks
        .set_action_name(Some("navigation.pop"));
    set_combo_row_ellipsis(imp.reset_wifi_device.get());
    imp.reset_wifi_switch.connect_state_set(
        clone!(@weak imp => @default-return glib::Propagation::Proceed, move |_, value| {
            set_wifi_enabled(value);
            if !value {
                let mut map = imp.wifi_entries.lock().unwrap();
                for entry in map.iter() {
                    imp.reset_wifi_list.remove(&*(*entry.1));
                }
                map.clear();
                imp.wifi_entries_path.lock().unwrap().clear();
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
    let _wifibox_ref_listener = wifi_box.clone();
    let wifi_entries = wifi_box.imp().wifi_entries.clone();
    let wifi_entries_path = wifi_box.imp().wifi_entries_path.clone();

    gio::spawn_blocking(move || {
        let access_points = get_access_points();
        let devices = get_wifi_devices();
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
        dbus_start_network_events();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let mut wifi_entries = wifi_entries.lock().unwrap();
                let mut wifi_entries_path = wifi_entries_path.lock().unwrap();
                let imp = wifibox_ref.imp();

                let list = imp.reset_model_list.read().unwrap();
                imp.reset_wifi_device.set_model(Some(&*list));
                let map = imp.reset_wifi_devices.read().unwrap();
                {
                    let device = imp.reset_current_wifi_device.borrow();
                    if let Some(index) = map.get(&device.name) {
                        imp.reset_wifi_device.set_selected(index.1);
                    }
                }

                imp.reset_wifi_device.connect_selected_notify(
                    clone!(@weak imp => move |dropdown| {
                        let selected = dropdown.selected_item();
                        if selected.is_none() {
                            return;
                        }
                        let selected = selected.unwrap();
                        let selected = selected.downcast_ref::<StringObject>().unwrap();
                        let selected = selected.string().to_string();

                        let device = imp.reset_wifi_devices.read().unwrap();
                        let device = device.get(&selected);
                        if device.is_none() {
                            return;
                        }
                        set_wifi_device(device.unwrap().0.path.clone());
                    }),
                );
                for access_point in access_points {
                    let ssid = access_point.ssid.clone();
                    let path = access_point.dbus_path.clone();
                    let connected =
                        imp.reset_current_wifi_device.borrow().active_access_point == path;
                    let entry = WifiEntry::new(connected, access_point, imp);
                    wifi_entries.insert(ssid, entry.clone());
                    wifi_entries_path.insert(path, entry.clone());
                    imp.reset_wifi_list.add(&*entry);
                }
            });
        });
    });
}

pub fn show_stored_connections(wifi_box: Arc<WifiBox>) {
    let wifibox_ref = wifi_box.clone();
    gio::spawn_blocking(move || {
        let connections = get_stored_connections();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let self_imp = wifibox_ref.imp();
                for connection in connections {
                    // TODO include button for settings
                    let name =
                        &String::from_utf8(connection.1).unwrap_or_else(|_| String::from(""));
                    let entry = SavedWifiEntry::new(name, connection.0, self_imp);
                    self_imp.reset_stored_wifi_list.add(&entry);
                }
            });
        });
    });
}

pub fn dbus_start_network_events() {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ReSetDaemon",
        Duration::from_millis(1000),
    );
    let _: Result<(), Error> =
        proxy.method_call("org.Xetibo.ReSetWireless", "StartNetworkListener", ());
}

pub fn get_access_points() -> Vec<AccessPoint> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ReSetDaemon",
        Duration::from_millis(1000),
    );
    let res: Result<(Vec<AccessPoint>,), Error> =
        proxy.method_call("org.Xetibo.ReSetWireless", "ListAccessPoints", ());
    if res.is_err() {
        return Vec::new();
    }
    let (access_points,) = res.unwrap();
    access_points
}

pub fn set_wifi_device(path: Path<'static>) {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ReSetDaemon",
        Duration::from_millis(1000),
    );
    let _: Result<(bool,), Error> =
        proxy.method_call("org.Xetibo.ReSetWireless", "SetWifiDevice", (path,));
}

pub fn get_wifi_devices() -> Vec<WifiDevice> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ReSetDaemon",
        Duration::from_millis(1000),
    );
    let res: Result<(Vec<WifiDevice>,), Error> =
        proxy.method_call("org.Xetibo.ReSetWireless", "GetAllWifiDevices", ());
    if res.is_err() {
        return Vec::new();
    }
    let (devices,) = res.unwrap();
    devices
}

pub fn get_wifi_status() -> bool {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ReSetDaemon",
        Duration::from_millis(1000),
    );
    let res: Result<(bool,), Error> =
        proxy.method_call("org.Xetibo.ReSetWireless", "GetWifiStatus", ());
    if res.is_err() {
        return false;
    }
    res.unwrap().0
}

pub fn get_stored_connections() -> Vec<(Path<'static>, Vec<u8>)> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ReSetDaemon",
        Duration::from_millis(1000),
    );
    let res: ResultMap = proxy.method_call("org.Xetibo.ReSetWireless", "ListStoredConnections", ());
    if res.is_err() {
        println!("we got error...");
        return Vec::new();
    }
    let (connections,) = res.unwrap();
    connections
}

pub fn set_wifi_enabled(enabled: bool) {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ReSetDaemon",
        Duration::from_millis(1000),
    );
    let _: Result<(bool,), Error> =
        proxy.method_call("org.Xetibo.ReSetWireless", "SetWifiEnabled", (enabled,));
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
        let access_point_added = AccessPointAdded::match_rule(
            Some(&"org.Xetibo.ReSetDaemon".into()),
            Some(&Path::from("/org/Xetibo/ReSetDaemon")),
        )
        .static_clone();
        let access_point_removed = AccessPointRemoved::match_rule(
            Some(&"org.Xetibo.ReSetDaemon".into()),
            Some(&Path::from("/org/Xetibo/ReSetDaemon")),
        )
        .static_clone();
        let access_point_changed = AccessPointChanged::match_rule(
            Some(&"org.Xetibo.ReSetDaemon".into()),
            Some(&Path::from("/org/Xetibo/ReSetDaemon")),
        )
        .static_clone();
        let device_changed = WifiDeviceChanged::match_rule(
            Some(&"org.Xetibo.ReSetDaemon".into()),
            Some(&Path::from("/org/Xetibo/ReSetDaemon")),
        )
        .static_clone();
        let res = conn.add_match(access_point_added, move |ir: AccessPointAdded, _, _| {
            println!("received added event");
            let wifi_box = added_ref.clone();
            glib::spawn_future(async move {
                glib::idle_add_once(move || {
                    let imp = wifi_box.imp();
                    let mut wifi_entries = imp.wifi_entries.lock().unwrap();
                    let mut wifi_entries_path = imp.wifi_entries_path.lock().unwrap();
                    let ssid = ir.access_point.ssid.clone();
                    let path = ir.access_point.dbus_path.clone();
                    if wifi_entries.get(&ssid).is_some() {
                        return;
                    }
                    let connected = imp.reset_current_wifi_device.borrow().active_access_point
                        == ir.access_point.dbus_path;
                    let entry = WifiEntry::new(connected, ir.access_point, imp);
                    wifi_entries.insert(ssid, entry.clone());
                    wifi_entries_path.insert(path, entry.clone());
                    imp.reset_wifi_list.add(&*entry);
                });
            });
            true
        });
        if res.is_err() {
            println!("fail on add");
            return;
        }
        let res = conn.add_match(access_point_removed, move |ir: AccessPointRemoved, _, _| {
            println!("received removed event");
            let wifi_box = removed_ref.clone();
            glib::spawn_future(async move {
                glib::idle_add_once(move || {
                    let imp = wifi_box.imp();
                    let mut wifi_entries = imp.wifi_entries.lock().unwrap();
                    let mut wifi_entries_path = imp.wifi_entries_path.lock().unwrap();
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
        });
        if res.is_err() {
            println!("fail on remove");
            return;
        }
        let res = conn.add_match(access_point_changed, move |ir: AccessPointChanged, _, _| {
            println!("received changed event");
            dbg!(ir.access_point.clone());
            let wifi_box = changed_ref.clone();
            glib::spawn_future(async move {
                glib::idle_add_local_once(move || {
                    let imp = wifi_box.imp();
                    let wifi_entries = imp.wifi_entries.lock().unwrap();
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
                    entry_imp.reset_wifi_label.get().set_text(name);
                    entry_imp.reset_wifi_encrypted.set_visible(false);
                    // TODO handle encryption thing
                    entry_imp
                        .reset_wifi_strength
                        .get()
                        .set_from_icon_name(match strength {
                            WifiStrength::Excellent => {
                                Some("network-wireless-signal-excellent-symbolic")
                            }
                            WifiStrength::Ok => Some("network-wireless-signal-ok-symbolic"),
                            WifiStrength::Weak => Some("network-wireless-signal-weak-symbolic"),
                            WifiStrength::None => Some("network-wireless-signal-none-symbolic"),
                        });
                    if !ir.access_point.stored {
                        entry_imp.reset_wifi_edit_button.set_sensitive(false);
                    }
                    if ir.access_point.dbus_path
                        == imp.reset_current_wifi_device.borrow().active_access_point
                    {
                        entry_imp.reset_wifi_connected.set_text("Connected");
                    } else {
                        entry_imp.reset_wifi_connected.set_text("");
                    }
                    {
                        let mut wifi_name = entry_imp.wifi_name.borrow_mut();
                        *wifi_name = String::from(name);
                    }
                });
            });
            true
        });
        if res.is_err() {
            println!("fail on change");
            return;
        }
        let res = conn.add_match(device_changed, move |ir: WifiDeviceChanged, _, _| {
            println!("received wifidevice changed event");
            let wifi_box = wifi_changed_ref.clone();
            glib::spawn_future(async move {
                glib::idle_add_once(move || {
                    let imp = wifi_box.imp();
                    let mut current_device = imp.reset_current_wifi_device.borrow_mut();
                    if current_device.path == ir.wifi_device.path {
                        current_device.active_access_point = ir.wifi_device.active_access_point;
                    } else {
                        *current_device = ir.wifi_device;
                    }
                    let mut wifi_entries = imp.wifi_entries.lock().unwrap();
                    for entry in wifi_entries.iter_mut() {
                        let imp = entry.1.imp();
                        let mut connected = imp.connected.borrow_mut();
                        *connected = imp.access_point.borrow().dbus_path == current_device.path;
                    }
                });
            });
            true
        });
        if res.is_err() {
            println!("fail on add");
            return;
        }
        println!("starting thread listener");
        loop {
            let _ = conn.process(Duration::from_millis(1000));
            if !listeners.wifi_listener.load(Ordering::SeqCst) {
                println!("stopping thread listener");
                break;
            }
        }
    });
}