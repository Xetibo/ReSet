use std::cell::RefCell;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::rc::Rc;
use std::str::FromStr;

use adw::glib;
use adw::glib::Object;
use adw::prelude::PreferencesRowExt;
use glib::clone;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{ButtonExt, EditableExt, WidgetExt};
use ReSet_Lib::network::connection::{Address, Connection};

use crate::components::wifi::utils::IpProtocol;
use crate::components::wifi::wifiAddressEntryImpl;
use crate::components::wifi::wifiAddressEntryImpl::WifiAddressEntryImpl;

glib::wrapper! {
    pub struct WifiAddressEntry(ObjectSubclass<wifiAddressEntryImpl::WifiAddressEntryImpl>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl WifiAddressEntry {
    pub fn new(
        address: Option<usize>,
        conn: Rc<RefCell<Connection>>,
        protocol: IpProtocol,
    ) -> Self {
        let entry: WifiAddressEntry = Object::builder().build();
        let entryImp = entry.imp();

        if let Some(address) = address {
            let conn = conn.borrow();
            let address = unsafe { conn.ipv4.address_data.get_unchecked(address) };

            entryImp.resetAddressAddress.set_text(&address.address);
            entryImp
                .resetAddressPrefix
                .set_text(&address.prefix_length.to_string());
            entryImp
                .resetAddressRow
                .set_title(&format!("{}/{}", &*address.address, address.prefix_length));
        }
        entryImp.protocol.set(protocol);
        entry.setupCallbacks(conn);
        entry
    }

    pub fn setupCallbacks(&self, connection: Rc<RefCell<Connection>>) {
        let selfImp = self.imp();

        let conn = connection.clone();
        selfImp.resetAddressAddress.connect_changed(clone!(@weak selfImp => move |entry| {
            let addressInput = entry.text();
            let mut conn = conn.borrow_mut();

            if addressInput.is_empty() {
                selfImp.resetAddressAddress.remove_css_class("error");
                selfImp.resetAddressRow.set_title("Add new address");
                return;
            }
            let result = match selfImp.protocol.get() {
                IpProtocol::IPv4 => Ipv4Addr::from_str(addressInput.as_str()).map(IpAddr::V4),
                IpProtocol::IPv6 => Ipv6Addr::from_str(addressInput.as_str()).map(IpAddr::V6),
            };
            match result {
                Ok(ipAddr) => {
                    selfImp.resetAddressAddress.remove_css_class("error");
                    let addressData = match selfImp.protocol.get() {
                        IpProtocol::IPv4 => &mut conn.ipv4.address_data,
                        IpProtocol::IPv6 => &mut conn.ipv6.address_data,
                    };
                    addressData.push(Address::theBetterNew(ipAddr.to_string(), selfImp.prefix.get().1 as u32));
                    *selfImp.address.borrow_mut() = (true, ipAddr.to_string());
                }
                Err(_) => {
                    selfImp.resetAddressAddress.add_css_class("error");
                    *selfImp.address.borrow_mut() = (false, String::default());
                }
            }
            setRowName(&selfImp);
        }));

        let conn = connection.clone();
        selfImp.resetAddressPrefix.connect_changed(clone!(@weak selfImp => move |entry| {
            let prefixInput = entry.text();
            let prefix = prefixInput.parse::<u8>();
            let mut conn = conn.borrow_mut();

            let handleError = || {
                if selfImp.resetAddressPrefix.text().is_empty() {
                    selfImp.resetAddressPrefix.remove_css_class("error");
                } else {
                    selfImp.resetAddressPrefix.add_css_class("error");
                }
                selfImp.prefix.set((false, 0));
                setRowName(&selfImp);
            };

            if prefixInput.is_empty() || prefix.is_err() {
                handleError();
                return;
            }

            let prefix = prefix.unwrap();
            match selfImp.protocol.get() {
                IpProtocol::IPv4 if prefix <= 32 => {
                    selfImp.prefix.set((true, prefix as u32));
                    selfImp.resetAddressPrefix.remove_css_class("error");
                    if let Ok(address2) = Ipv4Addr::from_str(selfImp.resetAddressAddress.text().as_str()) {
                        if let Some(addr) = conn.ipv4.address_data.iter_mut()
                        .find(|connAddr| *connAddr.address == address2.to_string()) {
                            addr.prefix_length = prefix as u32;
                        }
                    }
                }
                IpProtocol::IPv6 if prefix <= 128 => {
                    selfImp.prefix.set((true, prefix as u32));
                    selfImp.resetAddressPrefix.remove_css_class("error");
                    if let Ok(address2) = Ipv6Addr::from_str(selfImp.resetAddressAddress.text().as_str()) {
                        if let Some(addr) = conn.ipv6.address_data.iter_mut()
                        .find(|connAddr| *connAddr.address == address2.to_string()) {
                            addr.prefix_length = prefix as u32;
                        }
                    }
                }
                _ => handleError()
            }
            setRowName(&selfImp);
        }));

        let conn = connection.clone();
        selfImp.resetAddressRemove.connect_clicked(
            clone!(@weak selfImp, @weak self as what => move |_| {
                let address = selfImp.resetAddressAddress.text();
                let mut conn = conn.borrow_mut();
                conn.ipv4.address_data.retain(|addr| addr.address != address);
                what.unparent();
            }),
        );
    }
}

fn setRowName(selfImp: &WifiAddressEntryImpl) {
    if selfImp.resetAddressAddress.text().is_empty() {
        return;
    }
    let address = selfImp.address.borrow();
    let prefix = selfImp.prefix.get();
    let title = match (address.0, prefix.0) {
        (true, true) => {
            format!("{}/{}", address.1, prefix.1)
        }
        (true, false) => "Prefix wrong".to_string(),
        (false, true) => "Address wrong".to_string(),
        (false, false) => "Address and Prefix wrong".to_string(),
    };
    selfImp.resetAddressRow.set_title(&title);
}
