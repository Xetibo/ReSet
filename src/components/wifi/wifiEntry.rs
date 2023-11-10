use crate::components::wifi::wifiEntryImpl;
use adw::glib;
use adw::glib::{Object, PropertySet};
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::blocking::Connection;
use dbus::Error;
use glib::clone;
use gtk::prelude::WidgetExt;
use gtk::GestureClick;
use std::sync::Arc;
use std::time::Duration;
use ReSet_Lib::network::network::{AccessPoint, WifiStrength};

glib::wrapper! {
    pub struct WifiEntry(ObjectSubclass<wifiEntryImpl::WifiEntry>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget;
}

unsafe impl Send for WifiEntry {}
unsafe impl Sync for WifiEntry {}

impl WifiEntry {
    pub fn new(access_point: AccessPoint) -> Arc<Self> {
        let entry: Arc<WifiEntry> = Arc::new(Object::builder().build());
        let stored_entry = entry.clone();
        let new_entry = entry.clone();
        let entryImp = entry.imp();
        let strength = WifiStrength::from_u8(access_point.strength);
        let ssid = access_point.ssid.clone();
        let name_opt = String::from_utf8(ssid).unwrap_or_else(|_| String::from(""));
        let name = name_opt.as_str();
        let stored = access_point.stored;
        entryImp.wifiStrength.set(strength);
        entryImp.resetWifiLabel.get().set_text(name);
        entryImp.resetWifiEncrypted.set_visible(false);
        // TODO handle encryption thing
        entryImp
            .resetWifiStrength
            .get()
            .set_from_icon_name(match strength {
                WifiStrength::Excellent => Some("network-wireless-signal-excellent-symbolic"),
                WifiStrength::Ok => Some("network-wireless-signal-ok-symbolic"),
                WifiStrength::Weak => Some("network-wireless-signal-weak-symbolic"),
                WifiStrength::None => Some("network-wireless-signal-none-symbolic"),
            });
        if access_point.connected == true {
            entryImp
                .resetWifiConnected
                .get()
                .set_from_icon_name(Some("network-wireless-connected-symbolic"));
        }
        {
            let mut wifiName = entryImp.wifiName.borrow_mut();
            *wifiName = String::from(name);
        }
        entryImp.accessPoint.set(access_point);
        let gesture = GestureClick::new();
        if stored {
            entryImp
                .resetWifiStored
                .get()
                .set_from_icon_name(Some("document-save-symbolic"));
            gesture.connect_released(move |_, _, _, _| {
                click_stored_network(stored_entry.clone());
            });
        } else {
            gesture.connect_released(move |_, _, _, _| {
                click_new_network(new_entry.clone());
            });
        }
        entry.add_controller(gesture);
        entry
    }
}

pub fn click_stored_network(entry: Arc<WifiEntry>) {
    // TODO handle unknown access point -> should be done by having 2 different categories
    let entryImp = entry.imp();
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let access_point = entryImp.accessPoint.clone().into_inner();
    if access_point.connected == true {
        let res: Result<(bool,), Error> =
            proxy.method_call("org.xetibo.ReSet", "DisconnectFromCurrentAccessPoint", ());
        if res.is_err() {
            // TODO handle error
            println!("no worky");
            return;
        }
        let (res,) = res.unwrap();
        if res == false {
        } else {
            entryImp.resetWifiConnected.get().set_from_icon_name(None);
            let mut access_point = entryImp.accessPoint.borrow_mut();
            (*access_point).connected = false;
        }
        return;
    }
    dbg!(access_point.clone());
    let res: Result<(bool,), Error> = proxy.method_call(
        "org.xetibo.ReSet",
        "ConnectToKnownAccessPoint",
        (access_point,),
    );
    if res.is_err() {
        // TODO handle error
        println!("no worky");
    } else {
        let (res,) = res.unwrap();
        if res == false {
            println!("no worky but it connected");
        } else {
            println!("worky");
            entryImp
                .resetWifiConnected
                .get()
                .set_from_icon_name(Some("network-wireless-connected-symbolic"));
            let mut access_point = entryImp.accessPoint.borrow_mut();
            (*access_point).connected = true;
        }
    }
}

pub fn click_new_network(entry: Arc<WifiEntry>) {
    println!("Not implemented yet :)");
}
