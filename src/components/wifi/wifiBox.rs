use std::sync::atomic::Ordering;

use std::sync::Arc;

use std::time::Duration;

use crate::components::base::utils::Listeners;
use crate::components::utils::setComboRowEllipsis;
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

use crate::components::wifi::wifiBoxImpl;
use crate::components::wifi::wifiEntry::WifiEntry;

use super::savedWifiEntry::SavedWifiEntry;

glib::wrapper! {
    pub struct WifiBox(ObjectSubclass<wifiBoxImpl::WifiBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

type ResultMap = Result<(Vec<(Path<'static>, Vec<u8>)>,), Error>;

unsafe impl Send for WifiBox {}
unsafe impl Sync for WifiBox {}

impl WifiBox {
    pub fn new(listeners: Arc<Listeners>) -> Arc<Self> {
        let obj: Arc<WifiBox> = Arc::new(Object::builder().build());
        setupCallbacks(listeners, obj)
    }

    pub fn setupCallbacks(&self) {}
}

fn setupCallbacks(listeners: Arc<Listeners>, wifiBox: Arc<WifiBox>) -> Arc<WifiBox> {
    let imp = wifiBox.imp();
    let wifibox_ref = wifiBox.clone();
    imp.resetSavedNetworks.set_activatable(true);
    imp.resetSavedNetworks
        .set_action_name(Some("navigation.push"));
    imp.resetSavedNetworks
        .set_action_target_value(Some(&Variant::from("saved")));

    imp.resetAvailableNetworks.set_activatable(true);
    imp.resetAvailableNetworks
        .set_action_name(Some("navigation.pop"));
    setComboRowEllipsis(imp.resetWiFiDevice.get());
    imp.resetWifiSwitch.connect_state_set(
        clone!(@weak imp => @default-return glib::Propagation::Proceed, move |_, value| {
            set_wifi_enabled(value);
            if !value {
                let mut map = imp.wifiEntries.lock().unwrap();
                for entry in map.iter() {
                    imp.resetWifiList.remove(&*(*entry.1));
                }
                map.clear();
                imp.wifiEntriesPath.lock().unwrap().clear();
                listeners.network_listener.store(false, Ordering::SeqCst);
            } else {
                start_event_listener(listeners.clone(), wifibox_ref.clone());
                show_stored_connections(wifibox_ref.clone());
                scanForWifi(wifibox_ref.clone());
            }
            glib::Propagation::Proceed
        }),
    );
    wifiBox
}

pub fn scanForWifi(wifiBox: Arc<WifiBox>) {
    let wifibox_ref = wifiBox.clone();
    let _wifibox_ref_listener = wifiBox.clone();
    let wifiEntries = wifiBox.imp().wifiEntries.clone();
    let wifiEntriesPath = wifiBox.imp().wifiEntriesPath.clone();

    gio::spawn_blocking(move || {
        let accessPoints = get_access_points();
        let devices = get_wifi_devices();
        {
            let imp = wifibox_ref.imp();
            let list = imp.resetModelList.write().unwrap();
            let mut model_index = imp.resetModelIndex.write().unwrap();
            let mut map = imp.resetWifiDevices.write().unwrap();
            imp.resetCurrentWifiDevice
                .replace(devices.last().unwrap().clone());
            for (index, device) in devices.into_iter().enumerate() {
                list.append(&device.name);
                map.insert(device.name.clone(), (device, index as u32));
                *model_index += 1;
            }
        }
        let wifiEntries = wifiEntries.clone();
        let wifiEntriesPath = wifiEntriesPath.clone();
        dbus_start_network_events();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let mut wifiEntries = wifiEntries.lock().unwrap();
                let mut wifiEntriesPath = wifiEntriesPath.lock().unwrap();
                let imp = wifibox_ref.imp();

                let list = imp.resetModelList.read().unwrap();
                imp.resetWiFiDevice.set_model(Some(&*list));
                let map = imp.resetWifiDevices.read().unwrap();
                let device = imp.resetCurrentWifiDevice.borrow();
                if let Some(index) = map.get(&device.name) {
                    imp.resetWiFiDevice.set_selected(index.1);
                }
                imp.resetWiFiDevice
                    .connect_selected_notify(clone!(@weak imp => move |dropdown| {
                        let selected = dropdown.selected_item();
                        if selected.is_none() {
                            return;
                        }
                        let selected = selected.unwrap();
                        let selected = selected.downcast_ref::<StringObject>().unwrap();
                        let selected = selected.string().to_string();

                        let device = imp.resetWifiDevices.read().unwrap();
                        let device = device.get(&selected);
                        if device.is_none() {
                            return;
                        }
                        set_wifi_device(device.unwrap().0.path.clone());
                    }));
                for accessPoint in accessPoints {
                    let ssid = accessPoint.ssid.clone();
                    let path = accessPoint.dbus_path.clone();
                    let connected = imp.resetCurrentWifiDevice.borrow().active_access_point == path;
                    let entry = WifiEntry::new(connected, accessPoint, imp);
                    wifiEntries.insert(ssid, entry.clone());
                    wifiEntriesPath.insert(path, entry.clone());
                    imp.resetWifiList.add(&*entry);
                }
            });
        });
    });
}

pub fn show_stored_connections(wifiBox: Arc<WifiBox>) {
    let wifibox_ref = wifiBox.clone();
    gio::spawn_blocking(move || {
        let connections = get_stored_connections();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let selfImp = wifibox_ref.imp();
                for connection in connections {
                    // TODO include button for settings
                    let name =
                        &String::from_utf8(connection.1).unwrap_or_else(|_| String::from(""));
                    let entry = SavedWifiEntry::new(name, connection.0, selfImp);
                    selfImp.resetStoredWifiList.add(&entry);
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
    let (accessPoints,) = res.unwrap();
    accessPoints
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
        if listeners.network_listener.load(Ordering::SeqCst) {
            return;
        }
        listeners.network_listener.store(true, Ordering::SeqCst);

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
                    let mut wifiEntries = imp.wifiEntries.lock().unwrap();
                    let mut wifiEntriesPath = imp.wifiEntriesPath.lock().unwrap();
                    let ssid = ir.access_point.ssid.clone();
                    let path = ir.access_point.dbus_path.clone();
                    if wifiEntries.get(&ssid).is_some() {
                        return;
                    }
                    let connected = imp.resetCurrentWifiDevice.borrow().active_access_point
                        == ir.access_point.dbus_path;
                    let entry = WifiEntry::new(connected, ir.access_point, imp);
                    wifiEntries.insert(ssid, entry.clone());
                    wifiEntriesPath.insert(path, entry.clone());
                    imp.resetWifiList.add(&*entry);
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
                    let mut wifiEntries = imp.wifiEntries.lock().unwrap();
                    let mut wifiEntriesPath = imp.wifiEntriesPath.lock().unwrap();
                    let entry = wifiEntriesPath.remove(&ir.access_point);
                    if entry.is_none() {
                        return;
                    }
                    let entry = entry.unwrap();
                    let ssid = entry.imp().accessPoint.borrow().ssid.clone();
                    wifiEntries.remove(&ssid);
                    imp.resetWifiList.remove(&*entry);
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
                    let wifiEntries = imp.wifiEntries.lock().unwrap();
                    let entry = wifiEntries.get(&ir.access_point.ssid);
                    if entry.is_none() {
                        return;
                    }
                    let entry = entry.unwrap();
                    let entryImp = entry.imp();
                    let strength = WifiStrength::from_u8(ir.access_point.strength);
                    let ssid = ir.access_point.ssid.clone();
                    let name_opt = String::from_utf8(ssid).unwrap_or_else(|_| String::from(""));
                    let name = name_opt.as_str();
                    entryImp.wifiStrength.set(strength);
                    entryImp.resetWifiLabel.get().set_text(name);
                    entryImp.resetWifiEncrypted.set_visible(false);
                    // TODO handle encryption thing
                    entryImp
                        .resetWifiStrength
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
                        entryImp.resetWifiEditButton.set_sensitive(false);
                    }
                    if ir.access_point.dbus_path
                        == imp.resetCurrentWifiDevice.borrow().active_access_point
                    {
                        entryImp
                            .resetWifiConnected
                            .get()
                            .set_from_icon_name(Some("network-wireless-connected-symbolic"));
                    } else {
                        entryImp.resetWifiConnected.get().set_from_icon_name(None);
                    }
                    {
                        let mut wifiName = entryImp.wifiName.borrow_mut();
                        *wifiName = String::from(name);
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
                    let mut current_device = imp.resetCurrentWifiDevice.borrow_mut();
                    if current_device.path == ir.wifi_device.path {
                        current_device.active_access_point = ir.wifi_device.active_access_point;
                    } else {
                        *current_device = ir.wifi_device;
                    }
                    let mut wifiEntries = imp.wifiEntries.lock().unwrap();
                    for entry in wifiEntries.iter_mut() {
                        let imp = entry.1.imp();
                        let mut connected = imp.connected.borrow_mut();
                        *connected = imp.accessPoint.borrow().dbus_path == current_device.path;
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
            if !listeners.network_listener.load(Ordering::SeqCst) {
                println!("stopping thread listener");
                break;
            }
        }
    });
}
