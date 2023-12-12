use crate::components::wifi::wifi_options;
use adw::subclass::prelude::NavigationPageImpl;
use adw::{
    ActionRow, ComboRow, EntryRow, NavigationPage, PasswordEntryRow, PreferencesGroup, SwitchRow,
};
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Label};
use re_set_lib::network::connection::Connection;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetWifiOptions.ui")]
pub struct WifiOptions {
    // General
    #[template_child]
    pub reset_wifi_name: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_wifi_mac: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_wifi_link_speed: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_wifi_ip4_addr: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_wifi_ip6_addr: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_wifi_gateway: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_wifi_dns: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_wifi_last_used: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_wifi_auto_connect: TemplateChild<SwitchRow>,
    #[template_child]
    pub reset_wifi_metered: TemplateChild<SwitchRow>,
    // IPv4
    #[template_child]
    pub reset_ip4_method: TemplateChild<ComboRow>,
    #[template_child]
    pub reset_ip4_dns: TemplateChild<EntryRow>,
    #[template_child]
    pub reset_ip4_gateway: TemplateChild<EntryRow>,
    #[template_child]
    pub reset_ip4_address_group: TemplateChild<PreferencesGroup>,
    #[template_child]
    pub reset_ip4_address_add_button: TemplateChild<Button>,
    #[template_child]
    pub reset_ip4_routes_group: TemplateChild<PreferencesGroup>,
    #[template_child]
    pub reset_ip4_route_add_button: TemplateChild<Button>,
    // IPv6
    #[template_child]
    pub reset_ip6_method: TemplateChild<ComboRow>,
    #[template_child]
    pub reset_ip6_dns: TemplateChild<EntryRow>,
    #[template_child]
    pub reset_ip6_gateway: TemplateChild<EntryRow>,
    #[template_child]
    pub reset_ip6_address_group: TemplateChild<PreferencesGroup>,
    #[template_child]
    pub reset_ip6_address_add_button: TemplateChild<Button>,
    #[template_child]
    pub reset_ip6_routes_group: TemplateChild<PreferencesGroup>,
    #[template_child]
    pub reset_ip6_route_add_button: TemplateChild<Button>,
    // Security
    #[template_child]
    pub reset_wifi_security_dropdown: TemplateChild<ComboRow>,
    #[template_child]
    pub reset_wifi_password: TemplateChild<PasswordEntryRow>,
    // Misc
    #[template_child]
    pub reset_available_networks: TemplateChild<ActionRow>,
    #[template_child]
    pub wifi_options_apply_button: TemplateChild<Button>,
    #[template_child]
    pub wifi_options_error_msg: TemplateChild<Label>,
    pub connection: Rc<RefCell<Connection>>,
}

#[glib::object_subclass]
impl ObjectSubclass for WifiOptions {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetWifiOptions";
    type Type = wifi_options::WifiOptions;
    type ParentType = NavigationPage;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl NavigationPageImpl for WifiOptions {}

impl ObjectImpl for WifiOptions {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl BoxImpl for WifiOptions {}

impl WidgetImpl for WifiOptions {}

impl WindowImpl for WifiOptions {}

impl ApplicationWindowImpl for WifiOptions {}
