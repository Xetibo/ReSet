#![allow(non_snake_case)]

use std::thread;
use std::time::Duration;

use adw::glib::{Object, PropertySet};
use adw::glib::clone;
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::blocking::Connection;
use dbus::Error;
use gtk::glib;
use gtk::prelude::{ListBoxRowExt, TreeViewExt, WidgetExt};

use crate::components::wifi::wifiEntry::WifiStrength;

mod wifiBox;
mod wifiEntry;

glib::wrapper! {
    pub struct WifiBox(ObjectSubclass<wifiBox::WifiBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

glib::wrapper! {
    pub struct WifiEntry(ObjectSubclass<wifiEntry::WifiEntry>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget;
}

impl WifiBox {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn setupCallbacks(&self) {
        let selfImp = self.imp();

        selfImp.resetWifiDetails.connect_row_activated(clone!(@ weak selfImp as window => move |_, y| {
            // let result = y.downcast_ref()::<WifiEntry>().unwrap(); no worky smh
        }));
    }

    pub fn scanForWifi(&self) {
        let selfImp = self.imp();
        let mut wifiEntries = selfImp.wifiEntries.borrow_mut();
        wifiEntries.push(WifiEntry::new(WifiStrength::Excellent, "ina internet", true));
        wifiEntries.push(WifiEntry::new(WifiStrength::Excellent, "watch ina", true));
        wifiEntries.push(WifiEntry::new(WifiStrength::Ok, "INANET", true));
        wifiEntries.push(WifiEntry::new(WifiStrength::Weak, "ina best waifu", false));

        for wifiEntry in wifiEntries.iter() {
            selfImp.resetWifiList.append(wifiEntry);
        }
    }

    pub fn donotdisturb() {
        thread::spawn(|| {
            let conn = Connection::new_session().unwrap();
            let proxy = conn.with_proxy(
                "org.freedesktop.Notifications",
                "/org/freedesktop/Notifications",
                Duration::from_millis(1000),
            );
            let _: Result<(), Error> = proxy.method_call("org.freedesktop.Notifications", "DoNotDisturb", ());
        });
    }
}

impl WifiEntry {
    pub fn new(strength: WifiStrength, name: &str, isEncrypted: bool) -> Self {
        let entry: WifiEntry = Object::builder().build();
        let entryImp = entry.imp();
        entryImp.wifiStrength.set(strength);
        entryImp.resetWifiLabel.get().set_text(name);
        entryImp.resetWifiEncrypted.set_visible(isEncrypted);
        entryImp.resetWifiStrength.get().set_from_icon_name(match strength {
            WifiStrength::Excellent => Some("network-wireless-signal-excellent-symbolic"),
            WifiStrength::Ok => Some("network-wireless-signal-ok-symbolic"),
            WifiStrength::Weak => Some("network-wireless-signal-weak-symbolic"),
            WifiStrength::None => Some("network-wireless-signal-none-symbolic"),
        });
        {
            let mut wifiName = entryImp.wifiName.borrow_mut();
            *wifiName = String::from(name);
        }
        entry
    }
}