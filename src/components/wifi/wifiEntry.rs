use std::sync::Arc;
use std::time::Duration;

use crate::components::wifi::utils::getConnectionSettings;
use adw::glib;
use adw::glib::{Object, PropertySet};
use adw::prelude::{ActionRowExt, ButtonExt, EditableExt, PopoverExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::blocking::Connection;
use dbus::Error;
use glib::clone;
use gtk::gio;
use gtk::prelude::{ListBoxRowExt, WidgetExt};
use ReSet_Lib::network::network::{AccessPoint, WifiStrength};

use crate::components::wifi::wifiBoxImpl::WifiBox;
use crate::components::wifi::wifiEntryImpl;
use crate::components::wifi::wifiOptions::WifiOptions;

glib::wrapper! {
    pub struct WifiEntry(ObjectSubclass<wifiEntryImpl::WifiEntry>)
        @extends adw::ActionRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget, gtk::ListBoxRow;
}

unsafe impl Send for WifiEntry {}
unsafe impl Sync for WifiEntry {}

impl WifiEntry {
    pub fn new(connected: bool, access_point: AccessPoint, wifiBox: &WifiBox) -> Arc<Self> {
        let entry: Arc<WifiEntry> = Arc::new(Object::builder().build());
        let stored_entry = entry.clone();
        let new_entry = entry.clone();
        let entryImp = entry.imp();
        let strength = WifiStrength::from_u8(access_point.strength);
        let ssid = access_point.ssid.clone();
        let name_opt = String::from_utf8(ssid).unwrap_or_else(|_| String::from(""));
        let name = name_opt.as_str();
        entryImp.wifiStrength.set(strength);
        entryImp.resetWifiLabel.get().set_text(name);
        entryImp.resetWifiEncrypted.set_visible(false);
        entryImp.connected.set(connected);
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
        if !access_point.stored {
            entryImp.resetWifiEditButton.set_sensitive(false);
        }
        if connected {
            entryImp
                .resetWifiConnected.set_text("Connected");
        }
        {
            let mut wifiName = entryImp.wifiName.borrow_mut();
            *wifiName = String::from(name);
        }
        entryImp.accessPoint.set(access_point);

        entry.set_activatable(true);
        entry.connect_activated(clone!(@weak entryImp => move |_| {
            let access_point = entryImp.accessPoint.borrow();
            if *entryImp.connected.borrow() {
                click_disconnect(stored_entry.clone());
            } else if access_point.stored {
                click_stored_network(stored_entry.clone());
            } else {
                click_new_network(new_entry.clone());
            }
        }));
        entry.setupCallbacks(wifiBox);
        entry
    }

    pub fn setupCallbacks(&self, wifiBox: &WifiBox) {
        let selfImp = self.imp();
        selfImp.resetWifiEditButton.connect_clicked(clone!(@ weak selfImp, @ weak wifiBox => move |_| {
            let _option = getConnectionSettings(selfImp.accessPoint.borrow().associated_connection.clone());
            wifiBox.resetWifiNavigation.push(&*WifiOptions::new(_option, selfImp.accessPoint.borrow().dbus_path.clone()));
        }));
    }
}

pub fn click_disconnect(entry: Arc<WifiEntry>) {
    println!("called disconnect");
    let entry_ref = entry.clone();
    entry.set_activatable(false);
    gio::spawn_blocking(move || {
        let imp = entry_ref.imp();
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSetDaemon",
            "/org/Xetibo/ReSetDaemon",
            Duration::from_millis(10000),
        );
        let res: Result<(bool,), Error> = proxy.method_call(
            "org.Xetibo.ReSetWireless",
            "DisconnectFromCurrentAccessPoint",
            (),
        );
        if res.is_err() {
            imp.connected.replace(false);
            return;
        }
        imp.resetWifiConnected.set_text("");
        imp.connected.replace(false);
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                entry.set_activatable(true);
            });
        });
    });
}

pub fn click_stored_network(entry: Arc<WifiEntry>) {
    let entryImp = entry.imp();
    let access_point = entryImp.accessPoint.borrow().clone();
    let entry_ref = entry.clone();
    entry.set_activatable(false);
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSetDaemon",
            "/org/Xetibo/ReSetDaemon",
            Duration::from_millis(10000),
        );
        let res: Result<(bool,), Error> = proxy.method_call(
            "org.Xetibo.ReSetWireless",
            "ConnectToKnownAccessPoint",
            (access_point,),
        );
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                entry.set_activatable(true);
                let imp = entry_ref.imp();
                if res.is_err() {
                    println!("wtf?");
                    imp.connected.replace(false);
                    return;
                }
                if res.unwrap() == (false,) {
                    println!("false on connecting");
                    imp.connected.replace(false);
                    return;
                }
                let imp = entry_ref.imp();
                imp.resetWifiConnected.set_text("Connected");
                imp.connected.replace(true);
            });
        });
    });
    // TODO crate spinner animation and block UI
}

pub fn click_new_network(entry: Arc<WifiEntry>) {
    let connect_new_network =
        |entry: Arc<WifiEntry>, access_point: AccessPoint, password: String| {
            let entry_ref = entry.clone();
            let popup = entry.imp().resetWifiPopup.imp();
            popup.resetPopupLabel.set_text("Connecting...");
            popup.resetPopupLabel.set_visible(true);
            popup.resetPopupEntry.set_sensitive(false);
            popup.resetPopupButton.set_sensitive(false);

            gio::spawn_blocking(move || {
                let conn = Connection::new_session().unwrap();
                let proxy = conn.with_proxy(
                    "org.Xetibo.ReSetDaemon",
                    "/org/Xetibo/ReSetDaemon",
                    Duration::from_millis(10000),
                );
                let res: Result<(bool,), Error> = proxy.method_call(
                    "org.Xetibo.ReSetWireless",
                    "ConnectToNewAccessPoint",
                    (access_point, password),
                );
                glib::spawn_future(async move {
                    glib::idle_add_once(move || {
                        if res.is_err() {
                            let imp = entry_ref.imp();
                            imp.resetWifiPopup
                                .imp()
                                .resetPopupLabel
                                .set_text("Could not connect to dbus.");
                            imp.connected.replace(false);
                            return;
                        }
                        if res.unwrap() == (false,) {
                            let imp = entry_ref.imp();
                            imp.resetWifiPopup
                                .imp()
                                .resetPopupLabel
                                .set_text("Could not connect to access point.");
                            imp.connected.replace(false);
                            return;
                        }
                        println!("worked?");
                        let imp = entry_ref.imp();
                        imp.resetWifiPopup.popdown();
                        imp.resetWifiEditButton.set_sensitive(true);
                        imp.resetWifiConnected
                            .get()
                            .set_from_icon_name(Some("network-wireless-connected-symbolic"));
                        imp.connected.replace(true);
                    });
                });
            });
            // TODO crate spinner animation and block UI
        };

    let entryImp = entry.imp();
    let popupImp = entryImp.resetWifiPopup.imp();
    popupImp
        .resetPopupEntry
        .connect_activate(clone!(@weak entry as origEntry, @weak entryImp => move |entry| {
                connect_new_network(origEntry, entryImp.accessPoint.clone().take(), entry.text().to_string());
        }));
    popupImp.resetPopupButton.connect_clicked(
        clone!(@weak entry as origEntry,@weak entryImp, @weak popupImp => move |_| {
            let entry = entryImp.resetWifiPopup.imp().resetPopupEntry.text().to_string();
                connect_new_network(origEntry, entryImp.accessPoint.clone().take(), entry);
        }),
    );
    entryImp.resetWifiPopup.popup();
}
