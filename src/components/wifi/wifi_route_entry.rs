use crate::components::wifi::utils::IpProtocol;
use adw::glib;
use adw::glib::Object;
use adw::prelude::{ExpanderRowExt, PreferencesRowExt};
use glib::clone;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{EditableExt, WidgetExt};
use re_set_lib::network::connection::{Address, Connection};
use std::cell::RefCell;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::rc::Rc;
use std::str::FromStr;

use crate::components::wifi::wifi_route_entry_impl;
use crate::components::wifi::wifi_route_entry_impl::WifiRouteEntryImpl;

glib::wrapper! {
    pub struct WifiRouteEntry(ObjectSubclass<wifi_route_entry_impl::WifiRouteEntryImpl>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl WifiRouteEntry {
    pub fn new(
        address: Option<usize>,
        conn: Rc<RefCell<Connection>>,
        protocol: IpProtocol,
    ) -> Self {
        let entry: WifiRouteEntry = Object::builder().build();
        let entry_imp = entry.imp();

        if let Some(address) = address {
            let conn = conn.borrow();
            let address = unsafe { conn.ipv4.route_data.get_unchecked(address) };

            entry_imp.reset_route_address.set_text(&address.address);
            entry_imp
                .reset_route_prefix
                .set_text(&address.prefix.to_string());
            if let Some(gateway) = &address.gateway {
                entry_imp.reset_route_gateway.set_text(gateway);
            }
            if let Some(metric) = address.metric {
                entry_imp.reset_route_metric.set_text(&metric.to_string());
            }
            entry_imp
                .reset_route_row
                .set_title(&format!("{}/{}", &*address.address, address.prefix));
        }
        entry_imp.protocol.set(protocol);
        entry.setup_callbacks(conn);
        entry
    }

    fn setup_callbacks(&self, connection: Rc<RefCell<Connection>>) {
        let self_imp = self.imp();
        let conn = connection.clone();
        dbg!(conn.borrow());
        self_imp.reset_route_address.connect_changed(clone!(@weak self_imp => move |entry| {
            let address_input = entry.text();
            let mut conn = conn.borrow_mut();

            if address_input.is_empty() {
                self_imp.reset_route_address.remove_css_class("error");
                self_imp.reset_route_row.set_title("Add new address");
                return;
            }
            let result = match self_imp.protocol.get() {
                IpProtocol::IPv4 => Ipv4Addr::from_str(address_input.as_str()).map(IpAddr::V4),
                IpProtocol::IPv6 => Ipv6Addr::from_str(address_input.as_str()).map(IpAddr::V6),
            };
            match result {
                Ok(ip_addr) => {
                    self_imp.reset_route_address.remove_css_class("error");
                    let address_data = match self_imp.protocol.get() {
                        IpProtocol::IPv4 => &mut conn.ipv4.route_data,
                        IpProtocol::IPv6 => &mut conn.ipv6.route_data,
                    };
                    address_data.push(Address::new(ip_addr.to_string(), self_imp.prefix.get().1 as u32,self_imp.gateway.borrow().clone() ,self_imp.metric.get()));
                    *self_imp.address.borrow_mut() = (true, ip_addr.to_string());
                }
                Err(_) => {
                    self_imp.reset_route_address.add_css_class("error");
                    *self_imp.address.borrow_mut() = (false, String::default());
                }
            }
            set_row_title(&self_imp);
        }));

        let conn = connection.clone();
        self_imp.reset_route_prefix.connect_changed(clone!(@weak self_imp => move |entry| {
            let prefix_input = entry.text();
            let prefix = prefix_input.parse::<u8>();
            let mut conn = conn.borrow_mut();

            let handle_error = || {
                if self_imp.reset_route_prefix.text().is_empty() {
                    self_imp.reset_route_prefix.remove_css_class("error");
                } else {
                    self_imp.reset_route_prefix.add_css_class("error");
                }
                self_imp.prefix.set((false, 0));
                set_row_title(&self_imp);
            };

            if prefix_input.is_empty() || prefix.is_err() {
                handle_error();
                return;
            }

            let prefix = prefix.unwrap();
            match self_imp.protocol.get() {
                IpProtocol::IPv4 if prefix <= 32 => {
                    self_imp.prefix.set((true, prefix as u32));
                    self_imp.reset_route_prefix.remove_css_class("error");
                    if let Ok(address2) = Ipv4Addr::from_str(self_imp.reset_route_address.text().as_str()) {
                        if let Some(addr) = conn.ipv4.route_data.iter_mut()
                        .find(|conn_addr| *conn_addr.address == address2.to_string()) {
                            addr.prefix = prefix as u32;
                        }
                    }
                }
                IpProtocol::IPv6 if prefix <= 128 => {
                    self_imp.prefix.set((true, prefix as u32));
                    self_imp.reset_route_prefix.remove_css_class("error");
                    if let Ok(address2) = Ipv6Addr::from_str(self_imp.reset_route_address.text().as_str()) {
                        if let Some(addr) = conn.ipv6.route_data.iter_mut()
                        .find(|conn_addr| *conn_addr.address == address2.to_string()) {
                            addr.prefix = prefix as u32;
                        }
                    }
                }
                _ => handle_error()
            }
            set_row_title(&self_imp);
        }));

        let conn = connection.clone();
        self_imp
            .reset_route_gateway
            .connect_changed(clone!(@weak self_imp => move |entry| {
                let gateway_input = entry.text();
                let mut conn = conn.borrow_mut();

                if gateway_input.is_empty() {
                    self_imp.reset_route_gateway.remove_css_class("error");
                    *self_imp.gateway.borrow_mut() = None;
                    set_row_subtitle(&self_imp);
                    return;
                }
                let result = match self_imp.protocol.get() {
                    IpProtocol::IPv4 => Ipv4Addr::from_str(gateway_input.as_str()).map(IpAddr::V4),
                    IpProtocol::IPv6 => Ipv6Addr::from_str(gateway_input.as_str()).map(IpAddr::V6),
                };
                match result {
                    Ok(ip_addr) => {
                        self_imp.reset_route_gateway.remove_css_class("error");
                        let address_data = match self_imp.protocol.get() {
                            IpProtocol::IPv4 => &mut conn.ipv4.route_data,
                            IpProtocol::IPv6 => &mut conn.ipv6.route_data,
                        };
                        if let Some(address) = address_data.iter_mut()
                        .find(|conn_addr| *conn_addr.address == self_imp.reset_route_address.text()) {
                            address.gateway = Some(ip_addr.to_string());
                        }
                        *self_imp.gateway.borrow_mut() = Some(ip_addr.to_string());
                    }
                    Err(_) => {
                        self_imp.reset_route_gateway.add_css_class("error");
                        *self_imp.gateway.borrow_mut() = None;
                    }
                }
                set_row_subtitle(&self_imp);
            }));

        let conn = connection.clone();
        self_imp
            .reset_route_metric
            .connect_changed(clone!(@weak self_imp => move |entry| {
                let metric_input = entry.text();
                let mut conn = conn.borrow_mut();

                if metric_input.is_empty() {
                    self_imp.reset_route_metric.remove_css_class("error");
                    self_imp.metric.set(None);
                    set_row_subtitle(&self_imp);
                    return;
                }
                let result = metric_input.parse::<u32>();
                match result {
                    Ok(metric) => {
                        self_imp.reset_route_metric.remove_css_class("error");
                        let address_data = match self_imp.protocol.get() {
                            IpProtocol::IPv4 => &mut conn.ipv4.route_data,
                            IpProtocol::IPv6 => &mut conn.ipv6.route_data,
                        };
                        if let Some(address) = address_data.iter_mut()
                        .find(|conn_addr| *conn_addr.address == self_imp.reset_route_address.text()) {
                            address.metric = Some(metric);
                        }
                        self_imp.metric.set(Some(metric));
                    }
                    Err(_) => {
                        self_imp.reset_route_metric.add_css_class("error");
                        self_imp.metric.set(None);
                    }
                }
                set_row_subtitle(&self_imp);
            }));
    }
}

fn set_row_title(self_imp: &WifiRouteEntryImpl) {
    if self_imp.reset_route_address.text().is_empty() {
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
    self_imp.reset_route_row.set_title(&title);
}

fn set_row_subtitle(self_imp: &WifiRouteEntryImpl) {
    let gateway = self_imp.gateway.borrow().clone();
    let metric = self_imp.metric.get();
    let title = match (gateway, metric) {
        (Some(gateway), Some(metric)) => {
            format!("{}, {}", gateway, metric)
        }
        (Some(gateway), None) => gateway,
        (None, Some(metric)) => metric.to_string(),
        (None, None) => String::default(),
    };
    self_imp.reset_route_row.set_subtitle(&title);
}
