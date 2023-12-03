use std::sync::atomic::Ordering;

use std::sync::Arc;

use std::time::Duration;

use crate::components::base::utils::Listeners;
use crate::components::utils::setComboRowEllipsis;
use adw::glib;
use adw::glib::Object;
use adw::prelude::{ListBoxRowExt, PreferencesGroupExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::blocking::Connection;
use dbus::message::SignalArgs;
use dbus::Error;
use dbus::Path;
use glib::PropertySet;
use gtk::gio;
use gtk::glib::Variant;
use gtk::prelude::{ActionableExt, BoxExt, WidgetExt};
use ReSet_Lib::network::network::{AccessPoint, WifiStrength};
use ReSet_Lib::signals::AccessPointAdded;
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
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn setupCallbacks(&self) {
        let selfImp = self.imp();
        selfImp.resetSavedNetworks.set_activatable(true);
        selfImp
            .resetSavedNetworks
            .set_action_name(Some("navigation.push"));
        selfImp
            .resetSavedNetworks
            .set_action_target_value(Some(&Variant::from("saved")));

        selfImp.resetAvailableNetworks.set_activatable(true);
        selfImp
            .resetAvailableNetworks
            .set_action_name(Some("navigation.pop"));
        setComboRowEllipsis(selfImp.resetWiFiDevice.get());
    }
}

impl Default for WifiBox {
    fn default() -> Self {
        Self::new()
    }
}

pub fn scanForWifi(_listeners: Arc<Listeners>, wifiBox: Arc<WifiBox>) {
    let wifibox_ref = wifiBox.clone();
    let _wifibox_ref_listener = wifiBox.clone();
    let wifiEntries = wifiBox.imp().wifiEntries.clone();
    let wifiEntriesPath = wifiBox.imp().wifiEntriesPath.clone();

    gio::spawn_blocking(move || {
        let accessPoints = get_access_points();
        let wifiEntries = wifiEntries.clone();
        let wifiEntriesPath = wifiEntriesPath.clone();
        dbus_start_network_events();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let mut wifiEntries = wifiEntries.lock().unwrap();
                let mut wifiEntriesPath = wifiEntriesPath.lock().unwrap();
                let selfImp = wifibox_ref.imp();
                for accessPoint in accessPoints {
                    let ssid = accessPoint.ssid.clone();
                    let path = accessPoint.dbus_path.clone();
                    let entry = WifiEntry::new(accessPoint, selfImp);
                    wifiEntries.insert(ssid, entry.clone());
                    wifiEntriesPath.insert(path, entry.clone());
                    selfImp.resetWifiList.add(&*entry);
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
                    let entry = WifiEntry::new(ir.access_point, imp);
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
                    if ir.access_point.connected {
                        entryImp.resetWifiConnected.set_text("Connected")
                    } else {
                        entryImp.resetWifiConnected.set_text("");
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
