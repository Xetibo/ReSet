use adw::glib;
use adw::glib::Object;
use adw::prelude::PreferencesRowExt;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::EditableExt;
use ReSet_Lib::network::connection::Address;

use crate::components::wifi::wifiAddressEntryImpl;
use crate::components::wifi::wifiOptions::getValueFromKey;

glib::wrapper! {
    pub struct WifiAddressEntry(ObjectSubclass<wifiAddressEntryImpl::WifiAddressEntryImpl>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl WifiAddressEntry {
    pub fn new(address: Option<&Address>) -> Self {
        let entry: WifiAddressEntry = Object::builder().build();

        if let Some(address) = address {
            let entryImp = entry.imp();
            let map = address.to_map();

            let addr = getValueFromKey(&map, "address");
            let prefix = getValueFromKey(&map, "prefix-length");

            entryImp.resetAddressAddress.set_text(&*addr);
            entryImp.resetAddressNetmask.set_text(&*prefix);
            entryImp.resetAddressRow.set_title(&*format!("{}, {}", addr, prefix));
        }
        entry
    }
}
