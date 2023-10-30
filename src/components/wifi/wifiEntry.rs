use crate::components::wifi::wifiEntryImpl;
use adw::glib;
use adw::glib::{Object, PropertySet};
use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::WidgetExt;
use crate::components::wifi::wifiEntryImpl::WifiStrength;

glib::wrapper! {
    pub struct WifiEntry(ObjectSubclass<wifiEntryImpl::WifiEntry>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget;
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