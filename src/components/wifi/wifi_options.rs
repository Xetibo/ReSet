use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use adw::glib::Object;
use adw::prelude::{ActionRowExt, ComboRowExt, PreferencesGroupExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use adw::{gio, glib};
use dbus::arg::PropMap;
use dbus::{Error, Path};
use glib::{clone, PropertySet};
use gtk::prelude::{ButtonExt, EditableExt, WidgetExt};
use IpProtocol::{IPv4, IPv6};
use ReSet_Lib::network::connection::{Connection, DNSMethod4, DNSMethod6, Enum, TypeSettings};

use crate::components::wifi::utils::IpProtocol;
use crate::components::wifi::wifi_address_entry::WifiAddressEntry;
use crate::components::wifi::wifi_options_impl;
use crate::components::wifi::wifi_route_entry::WifiRouteEntry;

glib::wrapper! {
    pub struct WifiOptions(ObjectSubclass<wifi_options_impl::WifiOptions>)
        @extends adw::NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

unsafe impl Send for WifiOptions {}
unsafe impl Sync for WifiOptions {}

impl WifiOptions {
    pub fn new(connection: Connection, access_point: Path<'static>) -> Arc<Self> {
        let wifi_option: Arc<WifiOptions> = Arc::new(Object::builder().build());
        wifi_option.imp().connection.set(connection);
        wifi_option.initialize_ui();
        setup_callbacks(&wifi_option, access_point);
        wifi_option
    }

    pub fn initialize_ui(&self) {
        let self_imp = self.imp();
        let ip4_address_length;
        let ip4_route_length;
        let ip6_address_length;
        let ip6_route_length;
        {
            let conn = self_imp.connection.borrow();
            ip4_address_length = conn.ipv4.address_data.len();
            ip4_route_length = conn.ipv4.route_data.len();
            ip6_address_length = conn.ipv4.address_data.len();
            ip6_route_length = conn.ipv4.route_data.len();

            // General
            self_imp
                .resetWifiAutoConnect
                .set_active(conn.settings.autoconnect);
            self_imp
                .resetWifiMetered
                .set_active(conn.settings.metered != -1);
            match &conn.device {
                TypeSettings::WIFI(wifi) => {
                    self_imp.resetWifiLinkSpeed.set_visible(false);
                    self_imp.resetWifiIP4Addr.set_visible(false);
                    self_imp.resetWifiIP6Addr.set_visible(false);
                    self_imp.resetWifiDNS.set_visible(false);
                    self_imp.resetWifiGateway.set_visible(false);
                    self_imp.resetWifiLastUsed.set_visible(true);
                    self_imp.resetWifiMac.set_subtitle(&wifi.cloned_mac_address);
                    self_imp
                        .resetWifiName
                        .set_subtitle(&String::from_utf8(wifi.ssid.clone()).unwrap_or_default());
                }
                TypeSettings::ETHERNET(ethernet) => {
                    self_imp.resetWifiLinkSpeed.set_visible(true);
                    self_imp.resetWifiIP4Addr.set_visible(true);
                    self_imp.resetWifiIP6Addr.set_visible(true);
                    self_imp.resetWifiDNS.set_visible(true);
                    self_imp.resetWifiGateway.set_visible(true);
                    self_imp.resetWifiLastUsed.set_visible(false);
                    self_imp
                        .resetWifiMac
                        .set_subtitle(&ethernet.cloned_mac_address);
                    self_imp
                        .resetWifiLinkSpeed
                        .set_subtitle(&ethernet.speed.to_string());
                }
                TypeSettings::VPN(_vpn) => {}
                TypeSettings::None => {}
            };
            // IPv4
            self_imp
                .resetIP4Method
                .set_selected(conn.ipv4.dns_method.to_i32() as u32);
            self.set_ip4_visibility(conn.ipv4.dns_method.to_i32() as u32);

            let ipv4_dns: Vec<String> = conn
                .ipv4
                .dns
                .iter()
                .map(|addr| {
                    addr.iter()
                        .map(|octet| octet.to_string())
                        .collect::<Vec<String>>()
                        .join(".")
                })
                .collect();
            self_imp.resetIP4DNS.set_text(&ipv4_dns.join(", "));
            self_imp.resetIP4Gateway.set_text(&conn.ipv4.gateway);
            // IPv6
            self_imp
                .resetIP6Method
                .set_selected(conn.ipv6.dns_method.to_i32() as u32);
            self.set_ip6_visibility(conn.ipv6.dns_method.to_i32() as u32);

            let ipv6_dns: Vec<String> = conn
                .ipv6
                .dns
                .iter()
                .map(|addr| {
                    addr.iter()
                        .map(|octet| octet.to_string())
                        .collect::<Vec<String>>()
                        .join(":")
                })
                .collect();
            self_imp.resetIP6DNS.set_text(&ipv6_dns.join(", "));
            self_imp.resetIP6Gateway.set_text(&conn.ipv6.gateway);

            // Security
            if let TypeSettings::WIFI(wifi) = &conn.device {
                match wifi.security_settings.key_management.as_str() {
                    "none" => {
                        self_imp.resetWifiSecurityDropdown.set_selected(0);
                        self_imp.resetWifiPassword.set_visible(false);
                        self_imp.resetWifiPassword.set_text("");
                    }
                    "wpa-psk" => {
                        self_imp.resetWifiSecurityDropdown.set_selected(1);
                        self_imp.resetWifiPassword.set_visible(true);
                        self_imp
                            .resetWifiPassword
                            .set_text(&wifi.security_settings.psk);
                    }
                    _ => {}
                }
            }
        }
        // IPv4
        for i in 0..ip4_address_length {
            let address = &WifiAddressEntry::new(Some(i), self_imp.connection.clone(), IPv4);
            self_imp.resetIP4AddressGroup.add(address);
        }
        let address = &WifiAddressEntry::new(None, self_imp.connection.clone(), IPv4);
        self_imp.resetIP4AddressGroup.add(address);

        for i in 0..ip4_route_length {
            let route = &WifiRouteEntry::new(Some(i), self_imp.connection.clone(), IPv4);
            self_imp.resetIP4RoutesGroup.add(route)
        }
        let route = &WifiRouteEntry::new(None, self_imp.connection.clone(), IPv4);
        self_imp.resetIP4RoutesGroup.add(route);

        // IPv6
        for i in 0..ip6_address_length {
            let address = &WifiAddressEntry::new(Some(i), self_imp.connection.clone(), IPv6);
            self_imp.resetIP6AddressGroup.add(address);
        }
        let address = &WifiAddressEntry::new(None, self_imp.connection.clone(), IPv6);
        self_imp.resetIP6AddressGroup.add(address);

        for i in 0..ip6_route_length {
            let route = &WifiRouteEntry::new(Some(i), self_imp.connection.clone(), IPv6);
            self_imp.resetIP6RoutesGroup.add(route);
        }
        let route = &WifiRouteEntry::new(None, self_imp.connection.clone(), IPv6);
        self_imp.resetIP6RoutesGroup.add(route);
        // Security
    }

    pub fn set_ip4_visibility(&self, method: u32) {
        let self_imp = self.imp();
        match method {
            0 => {
                // auto
                self_imp.resetIP4AddressGroup.set_visible(false);
                self_imp.resetIP4RoutesGroup.set_visible(true);
                self_imp.resetIP4Gateway.set_visible(false);
            }
            1 => {
                // manual
                self_imp.resetIP4AddressGroup.set_visible(true);
                self_imp.resetIP4RoutesGroup.set_visible(true);
                self_imp.resetIP4Gateway.set_visible(true);
            }
            _ => {
                self_imp.resetIP4AddressGroup.set_visible(false);
                self_imp.resetIP4RoutesGroup.set_visible(false);
                self_imp.resetIP4Gateway.set_visible(false);
            }
        }
    }

    pub fn set_ip6_visibility(&self, method: u32) {
        let self_imp = self.imp();
        match method {
            0 | 1 => {
                // auto, dhcp
                self_imp.resetIP6AddressGroup.set_visible(false);
                self_imp.resetIP6RoutesGroup.set_visible(true);
                self_imp.resetIP6Gateway.set_visible(false);
            }
            2 => {
                // manual
                self_imp.resetIP6AddressGroup.set_visible(true);
                self_imp.resetIP6RoutesGroup.set_visible(true);
                self_imp.resetIP6Gateway.set_visible(true);
            }
            _ => {
                self_imp.resetIP6AddressGroup.set_visible(false);
                self_imp.resetIP6RoutesGroup.set_visible(false);
                self_imp.resetIP6Gateway.set_visible(false);
            }
        }
    }
}

fn setup_callbacks(wifi_options: &Arc<WifiOptions>, path: Path<'static>) {
    let imp = wifi_options.imp();

    // General
    imp.resetWifiAutoConnect
        .connect_active_notify(clone!(@weak imp => move |x| {
            imp.connection.borrow_mut().settings.autoconnect = x.is_active();
        }));
    imp.resetWifiMetered
        .connect_active_notify(clone!(@weak imp => move |x| {
            imp.connection.borrow_mut().settings.metered = if x.is_active() { 1 } else { 2 };
        }));
    imp.wifiOptionsApplyButton
        .connect_clicked(clone!(@weak imp => move |_| {
            let prop = imp.connection.borrow().convert_to_propmap();
            set_connection_settings(path.clone(), prop);
        }));
    // IPv4
    let wifi_options_ip4 = wifi_options.clone();
    imp.resetIP4Method
        .connect_selected_notify(clone!(@weak imp => move |dropdown| {
            let selected = dropdown.selected();
            let mut conn = imp.connection.borrow_mut();
            conn.ipv4.dns_method = DNSMethod4::from_i32(selected as i32);
            wifi_options_ip4.set_ip4_visibility(selected);
        }));

    imp.resetIP4DNS
        .connect_changed(clone!(@weak imp => move |entry| {
            let dns_input = entry.text();
            let mut conn = imp.connection.borrow_mut();
            conn.ipv4.dns.clear();
            if dns_input.is_empty() {
                imp.resetIP4DNS.remove_css_class("error");
                return;
            }
            for dns_entry in dns_input.as_str().split(',').map(|s| s.trim()) {
                if let Ok(addr) = Ipv4Addr::from_str(dns_entry) {
                    imp.resetIP4DNS.remove_css_class("error");
                    conn.ipv4.dns.push(addr.octets().to_vec());
                } else {
                    imp.resetIP4DNS.add_css_class("error");
                }
            }
        }));
    imp.resetIP4AddressAddButton
        .connect_clicked(clone!(@weak imp => move |_|  {
            let address = &WifiAddressEntry::new(None, imp.connection.clone(), IpProtocol::IPv4);
            imp.resetIP4AddressGroup.add(address);
        }));

    imp.resetIP4Gateway
        .connect_changed(clone!(@weak imp => move |entry| {
            let gateway_input = entry.text();
            let mut conn = imp.connection.borrow_mut();
            conn.ipv4.gateway.clear();
            if gateway_input.is_empty() {
                imp.resetIP4Gateway.remove_css_class("error");
                return;
            }
            if Ipv4Addr::from_str(gateway_input.as_str()).is_ok() {
                imp.resetIP4Gateway.remove_css_class("error");
                conn.ipv4.gateway = gateway_input.to_string();
            } else {
                imp.resetIP4Gateway.add_css_class("error");
            }
        }));
    // IPv6
    let wifi_options_ip6 = wifi_options.clone();
    imp.resetIP6Method
        .connect_selected_notify(clone!(@weak imp => move |dropdown| {
            let selected = dropdown.selected();
            let mut conn = imp.connection.borrow_mut();
            conn.ipv6.dns_method = DNSMethod6::from_i32(selected as i32);
            wifi_options_ip6.set_ip6_visibility(selected);
        }));

    imp.resetIP6DNS
        .connect_changed(clone!(@weak imp => move |entry| {
            let dns_input = entry.text();
            let mut conn = imp.connection.borrow_mut();
            conn.ipv6.dns.clear();
            if dns_input.is_empty() {
                imp.resetIP6DNS.remove_css_class("error");
                return;
            }
            for dns_entry in dns_input.as_str().split(',').map(|s| s.trim()) {
                if let Ok(addr) = Ipv6Addr::from_str(dns_entry) {
                    imp.resetIP6DNS.remove_css_class("error");
                    conn.ipv6.dns.push(addr.octets().to_vec());
                } else {
                    imp.resetIP6DNS.add_css_class("error");
                }
            }
        }));
    imp.resetIP6AddressAddButton
        .connect_clicked(clone!(@weak imp => move |_|  {
            let address = &WifiAddressEntry::new(None, imp.connection.clone(), IpProtocol::IPv4);
            imp.resetIP6AddressGroup.add(address);
        }));

    imp.resetIP6Gateway
        .connect_changed(clone!(@weak imp => move |entry| {
            let gateway_input = entry.text();
            let mut conn = imp.connection.borrow_mut();
            conn.ipv6.gateway.clear();
            if gateway_input.is_empty() {
                imp.resetIP6Gateway.remove_css_class("error");
                return;
            }
            if Ipv6Addr::from_str(gateway_input.as_str()).is_ok() {
                imp.resetIP6Gateway.remove_css_class("error");
                conn.ipv6.gateway = gateway_input.to_string();
            } else {
                imp.resetIP6Gateway.add_css_class("error");
            }
        }));

    // Security
    imp.resetWifiSecurityDropdown
        .connect_selected_notify(clone!(@weak imp => move |dropdown| {
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

    imp.resetWifiPassword
        .connect_changed(clone!(@weak imp => move |entry| {
            let password_input = entry.text();
            let mut conn = imp.connection.borrow_mut();
            if let TypeSettings::WIFI(wifi) = &mut conn.device {
                wifi.security_settings.psk = password_input.to_string();
            }
        }));
}

fn set_connection_settings(path: Path<'static>, prop: PropMap) {
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
