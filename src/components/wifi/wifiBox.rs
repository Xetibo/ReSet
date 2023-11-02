use std::thread;
use std::time::Duration;

use adw::glib;
use adw::glib::clone;
use adw::glib::Object;
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::blocking::Connection;
use dbus::Error;
use gtk::prelude::ButtonExt;

use crate::components::wifi::wifiBoxImpl;
use crate::components::wifi::wifiEntry::WifiEntry;
use crate::components::wifi::wifiEntryImpl::WifiStrength;

glib::wrapper! {
    pub struct WifiBox(ObjectSubclass<wifiBoxImpl::WifiBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl WifiBox {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn setupCallbacks(&self) {
        let selfImp = self.imp();

        selfImp.resetWifiDetails.connect_row_activated(clone!(@ weak selfImp as window => move |_, _y| {
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
