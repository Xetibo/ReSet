use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use adw::{gio, glib};
use adw::glib::Object;
use adw::prelude::{ActionRowExt, ComboRowExt, PreferencesGroupExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::{Error, Path};
use dbus::arg::PropMap;
use glib::{clone, PropertySet};
use gtk::prelude::{ButtonExt, EditableExt, WidgetExt};
use ReSet_Lib::network::connection::{Connection, DNSMethod4, DNSMethod6, Enum, TypeSettings};
use IpProtocol::{IPv4, IPv6};

use crate::components::wifi::utils::IpProtocol;
use crate::components::wifi::wifiAddressEntry::WifiAddressEntry;
use crate::components::wifi::wifiOptionsImpl;
use crate::components::wifi::wifiRouteEntry::WifiRouteEntry;

glib::wrapper! {
    pub struct WifiOptions(ObjectSubclass<wifiOptionsImpl::WifiOptions>)
        @extends adw::NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

unsafe impl Send for WifiOptions {}
unsafe impl Sync for WifiOptions {}

impl WifiOptions {
    pub fn new(connection: Connection, accessPoint: Path<'static>) -> Arc<Self> {
        let wifiOption: Arc<WifiOptions> = Arc::new(Object::builder().build());
        wifiOption.imp().connection.set(connection);
        wifiOption.initializeUI();
        setupCallbacks(&wifiOption, accessPoint);
        wifiOption
    }

    pub fn initializeUI(&self) {
        let selfImp = self.imp();
        let ip4AddressLength;
        let ip4RouteLength;
        let ip6AddressLength;
        let ip6RouteLength;
        {
            let conn = selfImp.connection.borrow();
            ip4AddressLength = conn.ipv4.address_data.len();
            ip4RouteLength = conn.ipv4.route_data.len();
            ip6AddressLength = conn.ipv4.address_data.len();
            ip6RouteLength = conn.ipv4.route_data.len();

            // General
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
                    selfImp.resetWifiName.set_subtitle(&*String::from_utf8(wifi.ssid.clone())
                        .unwrap_or(String::default()));
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

            let ipv4Dns: Vec<String> = conn.ipv4.dns.iter().map(|addr| {
                addr.iter().map(|octet| octet.to_string()).collect::<Vec<String>>().join(".")
            }).collect();
            selfImp.resetIP4DNS.set_text(&ipv4Dns.join(", "));
            selfImp.resetIP4Gateway.set_text(&conn.ipv4.gateway);
            // IPv6
            selfImp.resetIP6Method.set_selected(conn.ipv6.dns_method.to_i32() as u32);
            self.setIP6Visibility(conn.ipv6.dns_method.to_i32() as u32);

            let ipv6Dns: Vec<String> = conn.ipv6.dns.iter().map(|addr| {
                addr.iter().map(|octet| octet.to_string()).collect::<Vec<String>>().join(":")
            }).collect();
            selfImp.resetIP6DNS.set_text(&ipv6Dns.join(", "));
            selfImp.resetIP6Gateway.set_text(&conn.ipv6.gateway);

            // Security
            if let TypeSettings::WIFI(wifi) = &conn.device {
                match wifi.security_settings.key_management.as_str() {
                    "none" => {
                        selfImp.resetWifiSecurityDropdown.set_selected(0);
                        selfImp.resetWifiPassword.set_visible(false);
                        selfImp.resetWifiPassword.set_text("");
                    }
                    "wpa-psk" => {
                        selfImp.resetWifiSecurityDropdown.set_selected(1);
                        selfImp.resetWifiPassword.set_visible(true);
                        selfImp.resetWifiPassword.set_text(&wifi.security_settings.psk);
                    }
                    _ => {}
                }
            }
        }
        // IPv4
        for i in 0..ip4AddressLength {
            let address = &WifiAddressEntry::new(Some(i), selfImp.connection.clone(), IPv4);
            selfImp.resetIP4AddressGroup.add(address);
        }
        let address = &WifiAddressEntry::new(None, selfImp.connection.clone(), IPv4);
        selfImp.resetIP4AddressGroup.add(address);

        for i in 0..ip4RouteLength {
            let route = &WifiRouteEntry::new(Some(i), selfImp.connection.clone(), IPv4);
            selfImp.resetIP4RoutesGroup.add(route)
        }
        let route = &WifiRouteEntry::new(None, selfImp.connection.clone(), IPv4);
        selfImp.resetIP4RoutesGroup.add(route);

        // IPv6
        for i in 0..ip6AddressLength {
            let address = &WifiAddressEntry::new(Some(i), selfImp.connection.clone(), IPv6);
            selfImp.resetIP6AddressGroup.add(address);
        }
        let address = &WifiAddressEntry::new(None, selfImp.connection.clone(), IPv6);
        selfImp.resetIP6AddressGroup.add(address);

        for i in 0..ip6RouteLength {
            let route = &WifiRouteEntry::new(Some(i), selfImp.connection.clone(), IPv6);
            selfImp.resetIP6RoutesGroup.add(route);
        }
        let route = &WifiRouteEntry::new(None, selfImp.connection.clone(), IPv6);
        selfImp.resetIP6RoutesGroup.add(route);
        // Security
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

fn setupCallbacks(wifiOptions: &Arc<WifiOptions>, path: Path<'static>) {
    let imp = wifiOptions.imp();

    // General
    imp.resetWifiAutoConnect.connect_active_notify(clone!(@weak imp => move |x| {
        imp.connection.borrow_mut().settings.autoconnect = x.is_active();
    }));
    imp.resetWifiMetered.connect_active_notify(clone!(@weak imp => move |x| {
        imp.connection.borrow_mut().settings.metered = if x.is_active() { 1 } else { 2 };
    }));
    imp.wifiOptionsApplyButton.connect_clicked(clone!(@weak imp => move |_| {
        let prop = imp.connection.borrow().convert_to_propmap();
        setConnectionSettings(path.clone(), prop);
    }));
    // IPv4
    let wifiOptionsIP4 = wifiOptions.clone();
    imp.resetIP4Method.connect_selected_notify(clone!(@weak imp => move |dropdown| {
        let selected = dropdown.selected();
        let mut conn = imp.connection.borrow_mut();
        conn.ipv4.dns_method = DNSMethod4::from_i32(selected as i32);
        wifiOptionsIP4.setIP4Visibility(selected);
    }));

    imp.resetIP4DNS.connect_changed(clone!(@weak imp => move |entry| {
        let dnsInput = entry.text();
        let mut conn = imp.connection.borrow_mut();
        conn.ipv4.dns.clear();
        if dnsInput.is_empty() {
            imp.resetIP4DNS.remove_css_class("error");
            return;
        }
        for dnsEntry in dnsInput.as_str().split(',').map(|s| s.trim()) {
            if let Ok(addr) = Ipv4Addr::from_str(dnsEntry) {
                imp.resetIP4DNS.remove_css_class("error");
                conn.ipv4.dns.push(addr.octets().to_vec());
            } else {
                imp.resetIP4DNS.add_css_class("error");
            }
        }
    }));
    imp.resetIP4AddressAddButton.connect_clicked(clone!(@weak imp => move |_|  {
        let address = &WifiAddressEntry::new(None, imp.connection.clone(), IpProtocol::IPv4);
        imp.resetIP4AddressGroup.add(address);
    }));

    imp.resetIP4Gateway.connect_changed(clone!(@weak imp => move |entry| {
        let gatewayInput = entry.text();
        let mut conn = imp.connection.borrow_mut();
        conn.ipv4.gateway.clear();
        if gatewayInput.is_empty() {
            imp.resetIP4Gateway.remove_css_class("error");
            return;
        }
        if let Ok(_) = Ipv4Addr::from_str(gatewayInput.as_str()) {
            imp.resetIP4Gateway.remove_css_class("error");
            conn.ipv4.gateway = gatewayInput.to_string();
        } else {
            imp.resetIP4Gateway.add_css_class("error");
        }
    }));
    // IPv6
    let wifiOptionsIP6 = wifiOptions.clone();
    imp.resetIP6Method.connect_selected_notify(clone!(@weak imp => move |dropdown| {
        let selected = dropdown.selected();
        let mut conn = imp.connection.borrow_mut();
        conn.ipv6.dns_method = DNSMethod6::from_i32(selected as i32);
        wifiOptionsIP6.setIP6Visibility(selected);
    }));

    imp.resetIP6DNS.connect_changed(clone!(@weak imp => move |entry| {
        let dnsInput = entry.text();
        let mut conn = imp.connection.borrow_mut();
        conn.ipv6.dns.clear();
        if dnsInput.is_empty() {
            imp.resetIP6DNS.remove_css_class("error");
            return;
        }
        for dnsEntry in dnsInput.as_str().split(',').map(|s| s.trim()) {
            if let Ok(addr) = Ipv6Addr::from_str(dnsEntry) {
                imp.resetIP6DNS.remove_css_class("error");
                conn.ipv6.dns.push(addr.octets().to_vec());
            } else {
                imp.resetIP6DNS.add_css_class("error");
            }
        }
    }));
    imp.resetIP6AddressAddButton.connect_clicked(clone!(@weak imp => move |_|  {
        let address = &WifiAddressEntry::new(None, imp.connection.clone(), IpProtocol::IPv4);
        imp.resetIP6AddressGroup.add(address);
    }));

    imp.resetIP6Gateway.connect_changed(clone!(@weak imp => move |entry| {
        let gatewayInput = entry.text();
        let mut conn = imp.connection.borrow_mut();
        conn.ipv6.gateway.clear();
        if gatewayInput.is_empty() {
            imp.resetIP6Gateway.remove_css_class("error");
            return;
        }
        if let Ok(_) = Ipv6Addr::from_str(gatewayInput.as_str()) {
            imp.resetIP6Gateway.remove_css_class("error");
            conn.ipv6.gateway = gatewayInput.to_string();
        } else {
            imp.resetIP6Gateway.add_css_class("error");
        }
    }));

    // Security
    imp.resetWifiSecurityDropdown.connect_selected_notify(clone!(@weak imp => move |dropdown| {
        let selected = dropdown.selected();
        let mut conn = imp.connection.borrow_mut();

        match (selected, &mut conn.device) {
            (0 , TypeSettings::WIFI(wifi)) => { // None
                imp.resetWifiPassword.set_visible(false);
                wifi.security_settings.key_management = String::from("none");
                wifi.security_settings.authentication_algorithm = String::from("open");
            },
            (1 , TypeSettings::WIFI(wifi)) => { // WPA/WPA2 Personal
                imp.resetWifiPassword.set_visible(true);
                wifi.security_settings.key_management = String::from("wpa-psk");
                wifi.security_settings.authentication_algorithm = String::from("");
            },
            (_, _) => {}
        }
    }));

    imp.resetWifiPassword.connect_changed(clone!(@weak imp => move |entry| {
        let passwordInput = entry.text();
        let mut conn = imp.connection.borrow_mut();
        if let TypeSettings::WIFI(wifi) = &mut conn.device {
            wifi.security_settings.psk = passwordInput.to_string();
        }
    }));
}

fn setConnectionSettings(path: Path<'static>, prop: PropMap) {
    gio::spawn_blocking(move || {
        let conn = dbus::blocking::Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSetDaemon",
            "/org/Xetibo/ReSetDaemon",
            Duration::from_millis(1000),
        );
        let _: Result<(bool,), Error> = proxy.method_call(
            "org.Xetibo.ReSetWireless",
            "SetConnectionSettings",
            (path, prop),
        );
    });
}