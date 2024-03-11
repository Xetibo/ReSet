use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use adw::gio;
use adw::glib::Object;
use adw::prelude::{ActionRowExt, ComboRowExt, PreferencesGroupExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::arg::PropMap;
use dbus::{Error, Path};
use glib::{clone, PropertySet};
use gtk::prelude::{ActionableExt, ButtonExt, EditableExt, ListBoxRowExt, WidgetExt};
use re_set_lib::network::connection::{
    Connection, DNSMethod4, DNSMethod6, Enum, KeyManagement, TypeSettings,
};

use IpProtocol::{IPv4, IPv6};

use crate::components::utils::{BASE, DBUS_PATH, WIRELESS};
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
    pub fn new(connection: Connection, connection_path: Path<'static>) -> Arc<Self> {
        let wifi_option: Arc<WifiOptions> = Arc::new(Object::builder().build());
        wifi_option.imp().connection.set(connection);
        wifi_option.initialize_ui();
        setup_callbacks(&wifi_option, connection_path);
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
                .reset_wifi_auto_connect
                .set_active(conn.settings.autoconnect);
            self_imp
                .reset_wifi_metered
                .set_active(conn.settings.metered != 0);
            match &conn.device {
                TypeSettings::WIFI(wifi) => {
                    self_imp.reset_wifi_link_speed.set_visible(false);
                    self_imp.reset_wifi_ip4_addr.set_visible(false);
                    self_imp.reset_wifi_ip6_addr.set_visible(false);
                    self_imp.reset_wifi_dns.set_visible(false);
                    self_imp.reset_wifi_gateway.set_visible(false);
                    self_imp.reset_wifi_last_used.set_visible(true);
                    self_imp
                        .reset_wifi_mac
                        .set_subtitle(&wifi.cloned_mac_address);
                    self_imp
                        .reset_wifi_name
                        .set_subtitle(&String::from_utf8(wifi.ssid.clone()).unwrap_or_default());
                }
                TypeSettings::ETHERNET(ethernet) => {
                    self_imp.reset_wifi_link_speed.set_visible(true);
                    self_imp.reset_wifi_ip4_addr.set_visible(true);
                    self_imp.reset_wifi_ip6_addr.set_visible(true);
                    self_imp.reset_wifi_dns.set_visible(true);
                    self_imp.reset_wifi_gateway.set_visible(true);
                    self_imp.reset_wifi_last_used.set_visible(false);
                    self_imp
                        .reset_wifi_mac
                        .set_subtitle(&ethernet.cloned_mac_address);
                    self_imp
                        .reset_wifi_link_speed
                        .set_subtitle(&ethernet.speed.to_string());
                }
                TypeSettings::VPN(_vpn) => {}
                TypeSettings::None => {}
            };
            // IPv4
            self_imp
                .reset_ip4_method
                .set_selected(conn.ipv4.method.to_i32() as u32);
            self.set_ip4_visibility(conn.ipv4.method.to_i32() as u32);

            let ipv4_dns: Vec<Ipv4Addr> = conn
                .ipv4
                .dns
                .iter()
                .map(|addr| Ipv4Addr::from(*addr))
                .collect();

            self_imp.reset_ip4_dns.set_text(
                &ipv4_dns
                    .iter()
                    .map(|ip| ip.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            );
            self_imp.reset_ip4_gateway.set_text(&conn.ipv4.gateway);
            // IPv6
            self_imp
                .reset_ip6_method
                .set_selected(conn.ipv6.method.to_i32() as u32);
            self.set_ip6_visibility(conn.ipv6.method.to_i32() as u32);

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

            self_imp.reset_ip6_dns.set_text(&ipv6_dns.join(", "));
            self_imp.reset_ip6_gateway.set_text(&conn.ipv6.gateway);

            // Security

            match &conn.security.key_management {
                KeyManagement::NONE => {
                    self_imp.reset_wifi_security_dropdown.set_selected(0);
                    self_imp.reset_wifi_password.set_visible(false);
                    self_imp.reset_wifi_password.set_text("");
                }
                KeyManagement::WPAPSK => {
                    self_imp.reset_wifi_security_dropdown.set_selected(1);
                    self_imp.reset_wifi_password.set_visible(true);
                    self_imp.reset_wifi_password.set_text(&conn.security.psk);
                }
                _ => {}
            }
        }

        // IPv4
        for i in 0..ip4_address_length {
            let address = &WifiAddressEntry::new(Some(i), self_imp.connection.clone(), IPv4);
            self_imp.reset_ip4_address_group.add(address);
        }
        let address = &WifiAddressEntry::new(None, self_imp.connection.clone(), IPv4);
        self_imp.reset_ip4_address_group.add(address);

        for i in 0..ip4_route_length {
            let route = &WifiRouteEntry::new(Some(i), self_imp.connection.clone(), IPv4);
            self_imp.reset_ip4_routes_group.add(route)
        }
        let route = &WifiRouteEntry::new(None, self_imp.connection.clone(), IPv4);
        self_imp.reset_ip4_routes_group.add(route);

        // IPv6
        for i in 0..ip6_address_length {
            let address = &WifiAddressEntry::new(Some(i), self_imp.connection.clone(), IPv6);
            self_imp.reset_ip6_address_group.add(address);
        }
        let address = &WifiAddressEntry::new(None, self_imp.connection.clone(), IPv6);
        self_imp.reset_ip6_address_group.add(address);

        for i in 0..ip6_route_length {
            let route = &WifiRouteEntry::new(Some(i), self_imp.connection.clone(), IPv6);
            self_imp.reset_ip6_routes_group.add(route);
        }
        let route = &WifiRouteEntry::new(None, self_imp.connection.clone(), IPv6);
        self_imp.reset_ip6_routes_group.add(route);
    }

    pub fn set_ip4_visibility(&self, method: u32) {
        let self_imp = self.imp();
        match method {
            0 => {
                // auto
                self_imp.reset_ip4_address_group.set_visible(false);
                self_imp.reset_ip4_routes_group.set_visible(true);
                self_imp.reset_ip4_gateway.set_visible(false);
            }
            1 => {
                // manual
                self_imp.reset_ip4_address_group.set_visible(true);
                self_imp.reset_ip4_routes_group.set_visible(true);
                self_imp.reset_ip4_gateway.set_visible(true);
            }
            _ => {
                self_imp.reset_ip4_address_group.set_visible(false);
                self_imp.reset_ip4_routes_group.set_visible(false);
                self_imp.reset_ip4_gateway.set_visible(false);
            }
        }
    }

    pub fn set_ip6_visibility(&self, method: u32) {
        let self_imp = self.imp();
        match method {
            0 | 1 => {
                // auto, dhcp
                self_imp.reset_ip6_address_group.set_visible(false);
                self_imp.reset_ip6_routes_group.set_visible(true);
                self_imp.reset_ip6_gateway.set_visible(false);
            }
            2 => {
                // manual
                self_imp.reset_ip6_address_group.set_visible(true);
                self_imp.reset_ip6_routes_group.set_visible(true);
                self_imp.reset_ip6_gateway.set_visible(true);
            }
            _ => {
                self_imp.reset_ip6_address_group.set_visible(false);
                self_imp.reset_ip6_routes_group.set_visible(false);
                self_imp.reset_ip6_gateway.set_visible(false);
            }
        }
    }
}

fn setup_callbacks(wifi_options: &Arc<WifiOptions>, path: Path<'static>) {
    let imp = wifi_options.imp();

    // General
    imp.reset_wifi_auto_connect
        .connect_active_notify(clone!(@weak imp => move |x| {
            imp.connection.borrow_mut().settings.autoconnect = x.is_active();
        }));
    imp.reset_wifi_metered
        .connect_active_notify(clone!(@weak imp => move |x| {
            imp.connection.borrow_mut().settings.metered = if x.is_active() { 1 } else { 2 };
        }));
    imp.wifi_options_apply_button
        .connect_clicked(clone!(@weak imp => move |_| {
            let prop = imp.connection.borrow().convert_to_propmap();
            set_connection_settings(path.clone(), prop);
        }));
    // IPv4
    let wifi_options_ip4 = wifi_options.clone();
    imp.reset_ip4_method
        .connect_selected_notify(clone!(@weak imp => move |dropdown| {
            let selected = dropdown.selected();
            let mut conn = imp.connection.borrow_mut();
            conn.ipv4.method = DNSMethod4::from_i32(selected as i32);
            wifi_options_ip4.set_ip4_visibility(selected);
        }));

    imp.reset_ip4_dns
        .connect_changed(clone!(@weak imp => move |entry| {
            let dns_input = entry.text();
            let mut conn = imp.connection.borrow_mut();
            conn.ipv4.dns.clear();
            if dns_input.is_empty() {
                imp.reset_ip4_dns.remove_css_class("error");
                return;
            }
            for dns_entry in dns_input.as_str().split(',').map(|s| s.trim()) {
                if let Ok(addr) = Ipv4Addr::from_str(dns_entry) {
                    imp.reset_ip4_dns.remove_css_class("error");
                    conn.ipv4.dns.push(u32::from_be_bytes(addr.octets()));
                } else {
                    imp.reset_ip4_dns.add_css_class("error");
                }
            }
        }));
    imp.reset_ip4_address_add_button
        .connect_clicked(clone!(@weak imp => move |_|  {
            let address = &WifiAddressEntry::new(None, imp.connection.clone(), IPv4);
            imp.reset_ip4_address_group.add(address);
        }));

    imp.reset_ip4_gateway
        .connect_changed(clone!(@weak imp => move |entry| {
            let gateway_input = entry.text();
            let mut conn = imp.connection.borrow_mut();
            conn.ipv4.gateway.clear();
            if gateway_input.is_empty() {
                imp.reset_ip4_gateway.remove_css_class("error");
                return;
            }
            if Ipv4Addr::from_str(gateway_input.as_str()).is_ok() {
                imp.reset_ip4_gateway.remove_css_class("error");
                conn.ipv4.gateway = gateway_input.to_string();
            } else {
                imp.reset_ip4_gateway.add_css_class("error");
            }
        }));
    // IPv6
    let wifi_options_ip6 = wifi_options.clone();
    imp.reset_ip6_method
        .connect_selected_notify(clone!(@weak imp => move |dropdown| {
            let selected = dropdown.selected();
            let mut conn = imp.connection.borrow_mut();
            conn.ipv6.method = DNSMethod6::from_i32(selected as i32);
            wifi_options_ip6.set_ip6_visibility(selected);
        }));

    imp.reset_ip6_dns
        .connect_changed(clone!(@weak imp => move |entry| {
            let dns_input = entry.text();
            let mut conn = imp.connection.borrow_mut();
            conn.ipv6.dns.clear();
            if dns_input.is_empty() {
                imp.reset_ip6_dns.remove_css_class("error");
                return;
            }
            for dns_entry in dns_input.as_str().split(',').map(|s| s.trim()) {
                if let Ok(addr) = Ipv6Addr::from_str(dns_entry) {
                    imp.reset_ip6_dns.remove_css_class("error");
                    conn.ipv6.dns.push(addr.octets().to_vec());
                } else {
                    imp.reset_ip6_dns.add_css_class("error");
                }
            }
        }));
    imp.reset_ip6_address_add_button
        .connect_clicked(clone!(@weak imp => move |_|  {
            let address = &WifiAddressEntry::new(None, imp.connection.clone(), IPv4);
            imp.reset_ip6_address_group.add(address);
        }));

    imp.reset_ip6_gateway
        .connect_changed(clone!(@weak imp => move |entry| {
            let gateway_input = entry.text();
            let mut conn = imp.connection.borrow_mut();
            conn.ipv6.gateway.clear();
            if gateway_input.is_empty() {
                imp.reset_ip6_gateway.remove_css_class("error");
                return;
            }
            if Ipv6Addr::from_str(gateway_input.as_str()).is_ok() {
                imp.reset_ip6_gateway.remove_css_class("error");
                conn.ipv6.gateway = gateway_input.to_string();
            } else {
                imp.reset_ip6_gateway.add_css_class("error");
            }
        }));

    // Security
    imp.reset_wifi_security_dropdown
        .connect_selected_notify(clone!(@weak imp => move |dropdown| {
            let selected = dropdown.selected();
            let mut conn = imp.connection.borrow_mut();

            match selected {
                0 => { // None
                    imp.reset_wifi_password.set_visible(false);
                    conn.security.key_management = KeyManagement::NONE;
                    conn.security.authentication_algorithm = String::from("none");
                },
                1 => { // WPA/WPA2 Personal
                    imp.reset_wifi_password.set_visible(true);
                    conn.security.key_management = KeyManagement::WPAPSK;
                    conn.security.authentication_algorithm = String::from("none");
                },
                _ => {}
            }
        }));

    imp.reset_wifi_password
        .connect_changed(clone!(@weak imp => move |entry| {
            let password_input = entry.text();
            if password_input.len() < 8 && !password_input.is_empty() {
                entry.add_css_class("error");
            } else {
                entry.remove_css_class("error");
            }
            let mut conn = imp.connection.borrow_mut();
            conn.security.psk = password_input.to_string();
        }));

    imp.reset_available_networks.set_activatable(true);
    imp.reset_available_networks
        .set_action_name(Some("navigation.pop"));
}

fn set_connection_settings(path: Path<'static>, prop: HashMap<String, PropMap>) {
    gio::spawn_blocking(move || {
        let conn = dbus::blocking::Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
        let _: Result<(bool,), Error> =
            proxy.method_call(WIRELESS, "SetConnectionSettings", (path, prop));
    });
}
