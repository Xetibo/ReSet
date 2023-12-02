use std::cell::{Ref, RefCell};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::rc::Rc;
use std::str::FromStr;
use adw::glib;
use adw::glib::Object;
use adw::prelude::{ExpanderRowExt, PreferencesRowExt};
use glib::clone;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{EditableExt, WidgetExt};
use ReSet_Lib::network::connection::{Address, Connection};
use crate::components::wifi::utils::IpProtocol;
use crate::components::wifi::utils::IpProtocol::IPv4;

use crate::components::wifi::wifiRouteEntryImpl;
use crate::components::wifi::wifiRouteEntryImpl::WifiRouteEntryImpl;

glib::wrapper! {
    pub struct WifiRouteEntry(ObjectSubclass<wifiRouteEntryImpl::WifiRouteEntryImpl>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl WifiRouteEntry {
    pub fn new(address: Option<usize>, conn : Rc<RefCell<Connection>>, protocol: IpProtocol) -> Self {
        let entry: WifiRouteEntry = Object::builder().build();
        let entryImp = entry.imp();

        if let Some(address) = address {
            let conn = conn.borrow();
            let address = unsafe { conn.ipv4.route_data.get_unchecked(address) };

            entryImp.resetRouteAddress.set_text(&*address.address);
            entryImp.resetRoutePrefix.set_text(&*address.prefix_length.to_string());
            if let Some(gateway) = &address.gateway {
                entryImp.resetRouteGateway.set_text(&*gateway);
            }
            if let Some(metric) = address.metric {
                entryImp.resetRouteMetric.set_text(&*metric.to_string());
            }
            entryImp.resetRouteRow.set_title(&format!("{}/{}", &*address.address, address.prefix_length));
        }
        entryImp.protocol.set(protocol);
        entry.setupCallbacks(conn);
        entry
    }

    fn setupCallbacks(&self, connection: Rc<RefCell<Connection>>) {
        let selfImp = self.imp();

        let conn = connection.clone();
        selfImp.resetRouteAddress.connect_changed(clone!(@weak selfImp => move |entry| {
            let addressInput = entry.text();
            let mut conn = conn.borrow_mut();

            if addressInput.is_empty() {
                selfImp.resetRouteAddress.remove_css_class("error");
                selfImp.resetRouteRow.set_title("Add new address");
                return;
            }
            let result = match selfImp.protocol.get() {
                IpProtocol::IPv4 => Ipv4Addr::from_str(addressInput.as_str()).map(|a| IpAddr::V4(a)),
                IpProtocol::IPv6 => Ipv6Addr::from_str(addressInput.as_str()).map(|a| IpAddr::V6(a)),
            };
            match result {
                Ok(ipAddr) => {
                    selfImp.resetRouteAddress.remove_css_class("error");
                    let addressData = match selfImp.protocol.get() {
                        IpProtocol::IPv4 => &mut conn.ipv4.route_data,
                        IpProtocol::IPv6 => &mut conn.ipv6.route_data,
                    };
                    addressData.push(Address::new(ipAddr.to_string(), selfImp.prefix.get().1 as u32, selfImp.gateway.borrow().clone(), selfImp.metric.get()));
                    *selfImp.address.borrow_mut() = (true, ipAddr.to_string());
                }
                Err(_) => {
                    selfImp.resetRouteAddress.add_css_class("error");
                    *selfImp.address.borrow_mut() = (false, String::default());
                }
            }
            setRowTitle(&selfImp);
        }));

        let conn = connection.clone();
        selfImp.resetRoutePrefix.connect_changed(clone!(@weak selfImp => move |entry| {
            let prefixInput = entry.text();
            let prefix = prefixInput.parse::<u8>();
            let mut conn = conn.borrow_mut();

            let handleError = || {
                if selfImp.resetRoutePrefix.text().is_empty() {
                    selfImp.resetRoutePrefix.remove_css_class("error");
                } else {
                    selfImp.resetRoutePrefix.add_css_class("error");
                }
                selfImp.prefix.set((false, 0));
                setRowTitle(&selfImp);
                return;
            };

            if prefixInput.is_empty() || !prefix.is_ok() {
                handleError();
                return;
            }

            let prefix = prefix.unwrap();
            match selfImp.protocol.get() {
                IpProtocol::IPv4 if prefix <= 32 => {
                    selfImp.prefix.set((true, prefix as u32));
                    selfImp.resetRoutePrefix.remove_css_class("error");
                    if let Ok(address2) = Ipv4Addr::from_str(selfImp.resetRouteAddress.text().as_str()) {
                        if let Some(addr) = conn.ipv4.route_data.iter_mut()
                        .find(|connAddr| *connAddr.address == address2.to_string()) {
                            addr.prefix_length = prefix as u32;
                        }
                    }
                }
                IpProtocol::IPv6 if prefix <= 128 => {
                    selfImp.prefix.set((true, prefix as u32));
                    selfImp.resetRoutePrefix.remove_css_class("error");
                    if let Ok(address2) = Ipv6Addr::from_str(selfImp.resetRouteAddress.text().as_str()) {
                        if let Some(addr) = conn.ipv6.route_data.iter_mut()
                        .find(|connAddr| *connAddr.address == address2.to_string()) {
                            addr.prefix_length = prefix as u32;
                        }
                    }
                }
                _ => handleError()
            }
            setRowTitle(&selfImp);
        }));

        let conn = connection.clone();
        selfImp.resetRouteGateway.connect_changed(clone!(@weak selfImp => move |entry| {
            let gatewayInput = entry.text();
            let mut conn = conn.borrow_mut();

            if gatewayInput.is_empty() {
                selfImp.resetRouteGateway.remove_css_class("error");
                *selfImp.gateway.borrow_mut() = None;
                setRowSubtitle(&selfImp);
                return;
            }
            let result = match selfImp.protocol.get() {
                IpProtocol::IPv4 => Ipv4Addr::from_str(gatewayInput.as_str()).map(|a| IpAddr::V4(a)),
                IpProtocol::IPv6 => Ipv6Addr::from_str(gatewayInput.as_str()).map(|a| IpAddr::V6(a)),
            };
            match result {
                Ok(ipAddr) => {
                    selfImp.resetRouteGateway.remove_css_class("error");
                    let addressData = match selfImp.protocol.get() {
                        IpProtocol::IPv4 => &mut conn.ipv4.route_data,
                        IpProtocol::IPv6 => &mut conn.ipv6.route_data,
                    };
                    if let Some(address) = addressData.iter_mut()
                    .find(|connAddr| *connAddr.address == selfImp.resetRouteAddress.text()) {
                        address.gateway = Some(ipAddr.to_string());
                    }
                    *selfImp.gateway.borrow_mut() = Some(ipAddr.to_string());
                }
                Err(_) => {
                    selfImp.resetRouteGateway.add_css_class("error");
                    *selfImp.gateway.borrow_mut() = None;
                }
            }
            setRowSubtitle(&selfImp);
        }));


        let conn = connection.clone();
        selfImp.resetRouteMetric.connect_changed(clone!(@weak selfImp => move |entry| {
            let metricInput = entry.text();
            let mut conn = conn.borrow_mut();

            if metricInput.is_empty() {
                selfImp.resetRouteMetric.remove_css_class("error");
                selfImp.metric.set(None);
                setRowSubtitle(&selfImp);
                return;
            }
            let result = metricInput.parse::<u32>();
            match result {
                Ok(metric) => {
                    selfImp.resetRouteMetric.remove_css_class("error");
                    let addressData = match selfImp.protocol.get() {
                        IpProtocol::IPv4 => &mut conn.ipv4.route_data,
                        IpProtocol::IPv6 => &mut conn.ipv6.route_data,
                    };
                    if let Some(address) = addressData.iter_mut()
                    .find(|connAddr| *connAddr.address == selfImp.resetRouteAddress.text()) {
                        address.metric = Some(metric);
                    }
                    selfImp.metric.set(Some(metric));
                }
                Err(_) => {
                    selfImp.resetRouteMetric.add_css_class("error");
                    selfImp.metric.set(None);
                }
            }
            setRowSubtitle(&selfImp);
        }));
    }
}

fn setRowTitle(selfImp: &WifiRouteEntryImpl) {
    if selfImp.resetRouteAddress.text().is_empty() { return; }
    let address = selfImp.address.borrow();
    let prefix = selfImp.prefix.get();
    let title = match (address.0, prefix.0) {
        (true, true) => { format!("{}/{}", address.1, prefix.1) },
        (true, false) => "Prefix wrong".to_string(),
        (false, true) => "Address wrong".to_string(),
        (false, false) => "Address and Prefix wrong".to_string(),
    };
    selfImp.resetRouteRow.set_title(&*title);
}

fn setRowSubtitle(selfImp: &WifiRouteEntryImpl) {
    let gateway = selfImp.gateway.borrow().clone();
    let metric = selfImp.metric.get();
    let title = match (gateway, metric) {
        (Some(gateway), Some(metric)) => { format!("{}, {}", gateway, metric) },
        (Some(gateway), None) => gateway,
        (None, Some(metric)) => metric.to_string(),
        (None, None) => String::default(),
    };
    selfImp.resetRouteRow.set_subtitle(&*title);
}
