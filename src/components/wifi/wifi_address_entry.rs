use re_set_lib::network::connection::Address;
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
use re_set_lib::network::connection::{Connection};

use crate::components::wifi::utils::IpProtocol;
use crate::components::wifi::wifi_address_entry_impl;
use crate::components::wifi::wifi_address_entry_impl::WifiAddressEntryImpl;

glib::wrapper! {
    pub struct WifiAddressEntry(ObjectSubclass<wifi_address_entry_impl::WifiAddressEntryImpl>)
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
        let entry_imp = entry.imp();

        if let Some(address) = address {
            let conn = conn.borrow();
            let address = unsafe { conn.ipv4.address_data.get_unchecked(address) };

            entry_imp.reset_address_address.set_text(&address.address);
            entry_imp
                .reset_address_prefix
                .set_text(&address.prefix.to_string());
            entry_imp
                .reset_address_row
                .set_title(&format!("{}/{}", &*address.address, address.prefix));
        }
        entry_imp.protocol.set(protocol);
        entry.setup_callbacks(conn);
        entry
    }

    pub fn setup_callbacks(&self, connection: Rc<RefCell<Connection>>) {
        let self_imp = self.imp();

        let conn = connection.clone();
        self_imp.reset_address_address.connect_changed(clone!(@weak self_imp => move |entry| {
            let address_input = entry.text();
            let mut conn = conn.borrow_mut();

            if address_input.is_empty() {
                self_imp.reset_address_address.remove_css_class("error");
                self_imp.reset_address_row.set_title("Add new address");
                return;
            }
            let result = match self_imp.protocol.get() {
                IpProtocol::IPv4 => Ipv4Addr::from_str(address_input.as_str()).map(IpAddr::V4),
                IpProtocol::IPv6 => Ipv6Addr::from_str(address_input.as_str()).map(IpAddr::V6),
            };
            match result {
                Ok(ip_addr) => {
                    self_imp.reset_address_address.remove_css_class("error");
                    let address_data = match self_imp.protocol.get() {
                        IpProtocol::IPv4 => &mut conn.ipv4.address_data,
                        IpProtocol::IPv6 => &mut conn.ipv6.address_data,
                    };
                    address_data.push(Address::new_no_options(ip_addr.to_string(), self_imp.prefix.get().1));
                    *self_imp.address.borrow_mut() = (true, ip_addr.to_string());
                }
                Err(_) => {
                    self_imp.reset_address_address.add_css_class("error");
                    *self_imp.address.borrow_mut() = (false, String::default());
                }
            }
            set_row_name(&self_imp);
        }));

        let conn = connection.clone();
        self_imp.reset_address_prefix.connect_changed(clone!(@weak self_imp => move |entry| {
            let prefix_input = entry.text();
            let prefix = prefix_input.parse::<u8>();
            let mut conn = conn.borrow_mut();

            let handle_error = || {
                if self_imp.reset_address_prefix.text().is_empty() {
                    self_imp.reset_address_prefix.remove_css_class("error");
                } else {
                    self_imp.reset_address_prefix.add_css_class("error");
                }
                self_imp.prefix.set((false, 0));
                set_row_name(&self_imp);
            };

            if prefix_input.is_empty() || prefix.is_err() {
                handle_error();
                return;
            }

            let prefix = prefix.unwrap();
            match self_imp.protocol.get() {
                IpProtocol::IPv4 if prefix <= 32 => {
                    self_imp.prefix.set((true, prefix as u32));
                    self_imp.reset_address_prefix.remove_css_class("error");
                    if let Ok(address2) = Ipv4Addr::from_str(self_imp.reset_address_address.text().as_str()) {
                        if let Some(addr) = conn.ipv4.address_data.iter_mut()
                        .find(|conn_addr| *conn_addr.address == address2.to_string()) {
                            addr.prefix = prefix as u32;
                        }
                    }
                }
                IpProtocol::IPv6 if prefix <= 128 => {
                    self_imp.prefix.set((true, prefix as u32));
                    self_imp.reset_address_prefix.remove_css_class("error");
                    if let Ok(address2) = Ipv6Addr::from_str(self_imp.reset_address_address.text().as_str()) {
                        if let Some(addr) = conn.ipv6.address_data.iter_mut()
                        .find(|conn_addr| *conn_addr.address == address2.to_string()) {
                            addr.prefix = prefix as u32;
                        }
                    }
                }
                _ => handle_error()
            }
            set_row_name(&self_imp);
        }));

        let conn = connection.clone();
        self_imp.reset_address_remove.connect_clicked(
            clone!(@weak self_imp, @weak self as what => move |_| {
                let address = self_imp.reset_address_address.text();
                let mut conn = conn.borrow_mut();
                conn.ipv4.address_data.retain(|addr| addr.address != address);
                what.unparent();
            }),
        );
    }
}

fn set_row_name(self_imp: &WifiAddressEntryImpl) {
    if self_imp.reset_address_address.text().is_empty() {
        return;
    }
    let address = self_imp.address.borrow();
    let prefix = self_imp.prefix.get();
    let title = match (address.0, prefix.0) {
        (true, true) => {
            format!("{}/{}", address.1, prefix.1)
        }
        (true, false) => "Prefix wrong".to_string(),
        (false, true) => "Address wrong".to_string(),
        (false, false) => "Address and Prefix wrong".to_string(),
    };
    self_imp.reset_address_row.set_title(&title);
}
