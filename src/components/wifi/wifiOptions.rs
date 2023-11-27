use std::sync::Arc;

use adw::glib;
use adw::glib::Object;
use adw::prelude::{ActionRowExt, ComboRowExt, PreferencesGroupExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::arg::PropMap;
use glib::{clone, PropertySet};
use gtk::prelude::{EditableExt, WidgetExt};
use regex::Regex;
use ReSet_Lib::network::connection::{Connection, Enum, TypeSettings};

use crate::components::wifi::wifiAddressEntry::WifiAddressEntry;
use crate::components::wifi::wifiOptionsImpl;
use crate::components::wifi::wifiRouteEntry::WifiRouteEntry;

glib::wrapper! {
    pub struct WifiOptions(ObjectSubclass<wifiOptionsImpl::WifiOptions>)
        @extends adw::NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl WifiOptions {
    pub fn new(connection: Connection) -> Arc<Self> {
        let wifiOption: Arc<WifiOptions> = Arc::new(Object::builder().build());
        wifiOption.imp().connection.set(connection);
        wifiOption.initializeUI();
        setupCallbacks(&wifiOption);
        wifiOption
    }

    pub fn initializeUI(&self) {
        let selfImp = self.imp();
        let conn = selfImp.connection.borrow();
        // General
        selfImp.resetWifiName.set_subtitle(&conn.settings.name);
        selfImp.resetWifiAutoConnect.set_active(conn.settings.autoconnect);
        selfImp.resetWifiMetered.set_active(conn.settings.metered != -1);
        match &conn.device {
            TypeSettings::WIFI(wifi) => {
                selfImp.resetWifiLinkSpeed.set_visible(false);
                selfImp.resetWifiIP4Addr.set_visible(false);
                selfImp.resetWifiIP6Addr.set_visible(false);
                selfImp.resetWifiDNS.set_visible(false);
                selfImp.resetWifiGateway.set_visible(false);
                selfImp.resetWifiLastUsed.set_visible(true);
                selfImp.resetWifiMac.set_subtitle(&*wifi.cloned_mac_address);
            }
            TypeSettings::ETHERNET(ethernet) => {
                selfImp.resetWifiLinkSpeed.set_visible(true);
                selfImp.resetWifiIP4Addr.set_visible(true);
                selfImp.resetWifiIP6Addr.set_visible(true);
                selfImp.resetWifiDNS.set_visible(true);
                selfImp.resetWifiGateway.set_visible(true);
                selfImp.resetWifiLastUsed.set_visible(false);
                selfImp.resetWifiMac.set_subtitle(&*ethernet.cloned_mac_address);
                selfImp.resetWifiLinkSpeed.set_subtitle(&*ethernet.speed.to_string());
            }
            TypeSettings::VPN(_vpn) => {}
            TypeSettings::None => {}
        };
        // IPv4
        selfImp.resetIP4Method.set_selected(conn.ipv4.dns_method.to_i32() as u32);
        self.setIP4Visibility(conn.ipv4.dns_method.to_i32() as u32);

        let ipv4Dns: Vec<String> = conn.ipv4.dns.iter()
            .map(|addr| {
                addr.iter()
                    .map(|octet| octet.to_string())
                    .collect::<Vec<String>>()
                    .join(".")
            })
            .collect();
        selfImp.resetIP4DNS.set_text(&ipv4Dns.join(", "));
        selfImp.resetIP4Gateway.set_text(&conn.ipv4.gateway);

        if conn.ipv4.address_data.is_empty() {
            selfImp.resetIP4AddressGroup.add(&WifiAddressEntry::new(None))
        } else {
            for address in conn.ipv4.address_data.iter() {
                selfImp.resetIP4AddressGroup.add(&WifiAddressEntry::new(Some(address)))
            }
        }

        if conn.ipv4.route_data.is_empty() {
            selfImp.resetIP4RoutesGroup.add(&WifiRouteEntry::new(None))
        } else {
            for address in conn.ipv4.route_data.iter() {
                selfImp.resetIP4RoutesGroup.add(&WifiRouteEntry::new(Some(address)))
            }
        }
        // IPv6
        selfImp.resetIP6Method.set_selected(conn.ipv6.dns_method.to_i32() as u32);
        self.setIP6Visibility(conn.ipv6.dns_method.to_i32() as u32);

        let ipv6Dns: Vec<String> = conn.ipv6.dns.iter()
            .map(|addr| {
                addr.iter()
                    .map(|octet| octet.to_string())
                    .collect::<Vec<String>>()
                    .join(":")
            })
            .collect();
        selfImp.resetIP6DNS.set_text(&ipv6Dns.join(", "));
        selfImp.resetIP6Gateway.set_text(&conn.ipv6.gateway);

        if conn.ipv6.address_data.is_empty() {
            selfImp.resetIP6AddressGroup.add(&WifiAddressEntry::new(None))
        } else {
            for address in conn.ipv6.address_data.iter() {
                selfImp.resetIP6AddressGroup.add(&WifiAddressEntry::new(Some(address)))
            }
        }

        if conn.ipv6.route_data.is_empty() {
            selfImp.resetIP6RoutesGroup.add(&WifiRouteEntry::new(None))
        } else {
            for address in conn.ipv6.route_data.iter() {
                selfImp.resetIP6RoutesGroup.add(&WifiRouteEntry::new(Some(address)))
            }
        }
        // Security
        dbg!(conn);
    }

    pub fn setIP4Visibility(&self, method: u32) {
        let selfImp = self.imp();
        match method {
            0 => { // auto
                selfImp.resetIP4AddressGroup.set_visible(false);
                selfImp.resetIP4RoutesGroup.set_visible(true);
                selfImp.resetIP4Gateway.set_visible(false);
            }
            1 => { // manual
                selfImp.resetIP4AddressGroup.set_visible(true);
                selfImp.resetIP4RoutesGroup.set_visible(true);
                selfImp.resetIP4Gateway.set_visible(true);
            }
            _ => {
                selfImp.resetIP4AddressGroup.set_visible(false);
                selfImp.resetIP4RoutesGroup.set_visible(false);
                selfImp.resetIP4Gateway.set_visible(false);
            }
        }
    }

    pub fn setIP6Visibility(&self, method: u32) {
        let selfImp = self.imp();
        match method {
            0 | 1 => { // auto, dhcp
                selfImp.resetIP6AddressGroup.set_visible(false);
                selfImp.resetIP6RoutesGroup.set_visible(true);
                selfImp.resetIP6Gateway.set_visible(false);
            }
            2 => { // manual
                selfImp.resetIP6AddressGroup.set_visible(true);
                selfImp.resetIP6RoutesGroup.set_visible(true);
                selfImp.resetIP6Gateway.set_visible(true);
            }
            _ => {
                selfImp.resetIP6AddressGroup.set_visible(false);
                selfImp.resetIP6RoutesGroup.set_visible(false);
                selfImp.resetIP6Gateway.set_visible(false);
            }
        }
    }
}

fn setupCallbacks(wifiOptions: &Arc<WifiOptions>) {
    let imp = wifiOptions.imp();

    // General
    imp.resetWifiAutoConnect.connect_active_notify(clone!(@weak imp => move |x| {
        imp.connection.borrow_mut().settings.autoconnect = x.is_active();
    }));
    imp.resetWifiMetered.connect_active_notify(clone!(@weak imp => move |x| {
        imp.connection.borrow_mut().settings.metered = if x.is_active() { 1 } else { 2 };
    }));
    // IPv4
    let wifiOptionsIP4 = wifiOptions.clone();
    imp.resetIP4Method.connect_selected_notify(clone!(@weak imp => move |dropdown| {
        let selected = dropdown.selected();
        wifiOptionsIP4.setIP4Visibility(selected);
    }));

    let dnsRegex = Regex::new(r"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$").unwrap();
    imp.resetIP4DNS.connect_changed(clone!(@weak imp => move |entry| {
        let dnsInput = entry.text();
        let mut conn = imp.connection.borrow_mut();
        conn.ipv4.dns.clear();
        if dnsInput.as_str().is_empty() {
            imp.resetIP4DNS.remove_css_class("error");
            return;
        }
        for dnsEntry in dnsInput.as_str().split(',').collect::<Vec<&str>>() {
            if dnsRegex.is_match(dnsEntry) {
                imp.resetIP4DNS.remove_css_class("error");
                let dnsParts = dnsEntry.split('.')
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>();
                conn.ipv4.dns.push(dnsParts);
            } else {
                imp.resetIP4DNS.add_css_class("error");
            }
        }
    }));
    // IPv6
    let wifiOptionsIP6 = wifiOptions.clone();
    imp.resetIP6Method.connect_selected_notify(clone!(@weak imp => move |dropdown| {
        let selected = dropdown.selected();
        wifiOptionsIP6.setIP6Visibility(selected);
    }));
    // Security
}

pub fn getValueFromKey(map: &PropMap, key: &str) -> String {
    map.get(key)
        .map_or_else(|| "".to_string(),
                     |value| value.0
                         .as_str()
                         .unwrap_or_default()
                         .trim()
                         .to_string())
}
