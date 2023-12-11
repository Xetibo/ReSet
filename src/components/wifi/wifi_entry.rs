use std::sync::Arc;
use std::time::Duration;

use crate::components::wifi::utils::get_connection_settings;
use adw::glib;
use adw::glib::{Object, PropertySet};
use adw::prelude::{ActionRowExt, ButtonExt, EditableExt, PopoverExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::blocking::Connection;
use dbus::Error;
use glib::clone;
use gtk::gio;
use gtk::prelude::{ListBoxRowExt, WidgetExt};
use re_set_lib::network::network_structures::{AccessPoint, WifiStrength};

use crate::components::wifi::wifi_box_impl::WifiBox;
use crate::components::wifi::wifi_entry_impl;
use crate::components::wifi::wifi_options::WifiOptions;

glib::wrapper! {
    pub struct WifiEntry(ObjectSubclass<wifi_entry_impl::WifiEntry>)
        @extends adw::ActionRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget, gtk::ListBoxRow;
}

unsafe impl Send for WifiEntry {}
unsafe impl Sync for WifiEntry {}

impl WifiEntry {
    pub fn new(connected: bool, access_point: AccessPoint, wifi_box: &WifiBox) -> Arc<Self> {
        let entry: Arc<WifiEntry> = Arc::new(Object::builder().build());
        let stored_entry = entry.clone();
        let new_entry = entry.clone();
        let entry_imp = entry.imp();
        let strength = WifiStrength::from_u8(access_point.strength);
        let ssid = access_point.ssid.clone();
        let name_opt = String::from_utf8(ssid).unwrap_or_else(|_| String::from(""));
        let name = name_opt.as_str();
        entry_imp.wifi_strength.set(strength);
        entry_imp.reset_wifi_label.get().set_text(name);
        entry_imp.reset_wifi_encrypted.set_visible(false);
        entry_imp.connected.set(connected);
        // TODO handle encryption thing
        entry_imp
            .reset_wifi_strength
            .get()
            .set_from_icon_name(match strength {
                WifiStrength::Excellent => Some("network-wireless-signal-excellent-symbolic"),
                WifiStrength::Ok => Some("network-wireless-signal-ok-symbolic"),
                WifiStrength::Weak => Some("network-wireless-signal-weak-symbolic"),
                WifiStrength::None => Some("network-wireless-signal-none-symbolic"),
            });
        if !access_point.stored {
            entry_imp.reset_wifi_edit_button.set_sensitive(false);
        }
        if connected {
            entry_imp.reset_wifi_connected.set_text("Connected");
        }
        {
            let mut wifi_name = entry_imp.wifi_name.borrow_mut();
            *wifi_name = String::from(name);
        }
        entry_imp.access_point.set(access_point);

        entry.set_activatable(true);
        entry.connect_activated(clone!(@weak entry_imp => move |_| {
            let access_point = entry_imp.access_point.borrow();
            if *entry_imp.connected.borrow() {
                click_disconnect(stored_entry.clone());
            } else if access_point.stored {
                click_stored_network(stored_entry.clone());
            } else {
                click_new_network(new_entry.clone());
            }
        }));
        entry.setup_callbacks(wifi_box);
        entry
    }

    pub fn setup_callbacks(&self, wifi_box: &WifiBox) {
        let self_imp = self.imp();
        self_imp.reset_wifi_edit_button.connect_clicked(clone!(@ weak self_imp, @ weak wifi_box => move |_| {
            let _option = get_connection_settings(self_imp.access_point.borrow().associated_connection.clone());
            wifi_box.reset_wifi_navigation.push(&*WifiOptions::new(_option, self_imp.access_point.borrow().dbus_path.clone()));
        }));
    }
}

pub fn click_disconnect(entry: Arc<WifiEntry>) {
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
        imp.reset_wifi_connected.set_text("");
        imp.connected.replace(false);
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                entry.set_activatable(true);
            });
        });
    });
}

pub fn click_stored_network(entry: Arc<WifiEntry>) {
    let entry_imp = entry.imp();
    let access_point = entry_imp.access_point.borrow().clone();
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
                    imp.connected.replace(false);
                    return;
                }
                if res.unwrap() == (false,) {
                    imp.connected.replace(false);
                    return;
                }
                let imp = entry_ref.imp();
                imp.reset_wifi_connected.set_text("Connected");
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
            let popup = entry.imp().reset_wifi_popup.imp();
            popup.reset_popup_label.set_text("Connecting...");
            popup.reset_popup_label.set_visible(true);
            popup.reset_popup_entry.set_sensitive(false);
            popup.reset_popup_button.set_sensitive(false);

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
                            imp.reset_wifi_popup
                                .imp()
                                .reset_popup_label
                                .set_text("Could not connect to dbus.");
                            imp.connected.replace(false);
                            return;
                        }
                        if res.unwrap() == (false,) {
                            let imp = entry_ref.imp();
                            imp.reset_wifi_popup
                                .imp()
                                .reset_popup_label
                                .set_text("Could not connect to access point.");
                            imp.connected.replace(false);
                            return;
                        }
                        let imp = entry_ref.imp();
                        imp.reset_wifi_popup.popdown();
                        imp.reset_wifi_edit_button.set_sensitive(true);
                        imp.reset_wifi_connected.set_text("Connected");
                        imp.connected.replace(true);
                    });
                });
            });
            // TODO crate spinner animation and block UI
        };

    let entry_imp = entry.imp();
    let popup_imp = entry_imp.reset_wifi_popup.imp();
    popup_imp
        .reset_popup_entry
        .connect_activate(clone!(@weak entry as orig_entry, @weak entry_imp => move |entry| {
                connect_new_network(orig_entry, entry_imp.access_point.clone().take(), entry.text().to_string());
        }));
    popup_imp.reset_popup_button.connect_clicked(
        clone!(@weak entry as orig_entry,@weak entry_imp, @weak popup_imp => move |_| {
            let entry = entry_imp.reset_wifi_popup.imp().reset_popup_entry.text().to_string();
                connect_new_network(orig_entry, entry_imp.access_point.clone().take(), entry);
        }),
    );
    entry_imp.reset_wifi_popup.popup();
}
