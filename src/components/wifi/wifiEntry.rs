use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

use adw::glib;
use adw::glib::{Object, PropertySet};
use adw::prelude::{ButtonExt, EditableExt, PopoverExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::blocking::Connection;
use dbus::Error;
use glib::{clone, Cast};
use gtk::prelude::WidgetExt;
use gtk::{AlertDialog, GestureClick};
use ReSet_Lib::network::network::{AccessPoint, WifiStrength};

use crate::components::wifi::wifiBox::getConnectionSettings;
use crate::components::wifi::wifiEntryImpl;

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
            gesture.connect_released(move |_, _, _, _| {
                click_stored_network(stored_entry.clone());
            });
        } else {
            entryImp.resetWifiEditButton.set_sensitive(false);
            gesture.connect_released(move |_, _, _, _| {
                click_new_network(new_entry.clone());
            });
        }
        entry.add_controller(gesture);
        entry
    }

    pub fn setupCallbacks(&self) {
        let selfImp = self.imp();
        selfImp.resetWifiEditButton.connect_clicked(clone!(@ weak selfImp => move |_| {
            // TODO open navigationpage
            let _option = getConnectionSettings(selfImp.accessPoint.borrow().associated_connection.clone());
        }));
    }
}

pub fn click_stored_network(entry: Arc<WifiEntry>) {
    let alert = AlertDialog::builder().build();
    let root = &entry.root().unwrap();
    let root = root.downcast_ref::<gtk::Window>();
    if root.is_none() {
        return;
    }
    let root = root.unwrap();
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
            alert.set_message("Error on connecting to dbus.");
            alert.show(Some(root));
            return;
        }
        let (res,) = res.unwrap();
        if res == false {
            alert.set_message("Could not disconnect from access point.");
            alert.show(Some(root));
        } else {
            entryImp.resetWifiConnected.get().set_from_icon_name(None);
            let mut access_point = entryImp.accessPoint.borrow_mut();
            (*access_point).connected = false;
        }
        return;
    }
    let res: Result<(bool,), Error> = proxy.method_call(
        "org.xetibo.ReSet",
        "ConnectToKnownAccessPoint",
        (access_point,),
    );
    if res.is_err() {
        alert.set_message("Error on connecting to dbus.");
        alert.show(Some(root));
    } else {
        let (res,) = res.unwrap();
        if res == false {
            alert.set_message("Could not connect to access point.");
            alert.show(Some(root));
        } else {
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
    let connect_new_network = |result: Arc<AtomicBool>,
                               entry: Arc<WifiEntry>,
                               access_point: AccessPoint,
                               password: String| {
        let entry_ref = entry.clone();
        let popup = entry.imp().resetWifiPopup.imp();
        popup.resetPopupLabel.set_text("Connecting...");
        popup.resetPopupLabel.set_visible(true);
        popup.resetPopupEntry.set_sensitive(false);
        popup.resetPopupButton.set_sensitive(false);

        glib::spawn_future_local(async move {
            let conn = Connection::new_session().unwrap();
            let proxy = conn.with_proxy(
                "org.xetibo.ReSet",
                "/org/xetibo/ReSet",
                Duration::from_millis(10000),
            );
            let res: Result<(bool,), Error> = proxy.method_call(
                "org.xetibo.ReSet",
                "ConnectToNewAccessPoint",
                (access_point, password),
            );
            glib::MainContext::default().spawn_local(async move {
                glib::idle_add_once(move || {
                    if res.is_err() {
                        entry_ref
                            .imp()
                            .resetWifiPopup
                            .imp()
                            .resetPopupLabel
                            .set_text("Could not connect to dbus.");
                        result.store(false, std::sync::atomic::Ordering::SeqCst);
                        return;
                    }
                    if res.unwrap() == (false,) {
                        entry_ref
                            .imp()
                            .resetWifiPopup
                            .imp()
                            .resetPopupLabel
                            .set_text("Could not connect to access point.");
                        result.store(false, std::sync::atomic::Ordering::SeqCst);
                        return;
                    }
                    entry_ref.imp().resetWifiPopup.popdown();
                    result.store(true, std::sync::atomic::Ordering::SeqCst);
                });
            });
        });
        // TODO crate spinner animation and block UI
    };

    let result = Arc::new(AtomicBool::new(false));
    let result_ref = result.clone();
    let result_ref_button = result.clone();
    let entryImp = entry.imp();
    let popupImp = entryImp.resetWifiPopup.imp();
    popupImp
        .resetPopupEntry
        .connect_activate(clone!(@weak entry as origEntry, @weak entryImp => move |entry| {
                connect_new_network(result_ref.clone(), origEntry, entryImp.accessPoint.clone().take(), entry.text().to_string());
        }));
    popupImp.resetPopupButton.connect_clicked(
        clone!(@weak entry as origEntry,@weak entryImp, @weak popupImp => move |_| {
            let entry = entryImp.resetWifiPopup.imp().resetPopupEntry.text().to_string();
                connect_new_network(result_ref_button.clone(), origEntry, entryImp.accessPoint.clone().take(), entry);
        }),
    );
    entryImp.resetWifiPopup.popup();
}
