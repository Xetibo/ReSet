use std::cell::RefCell;
use std::rc::Rc;
use adw::{ActionRow, ComboRow, EntryRow, NavigationPage, PreferencesGroup, SwitchRow};
use adw::subclass::prelude::NavigationPageImpl;
use crate::components::wifi::{wifiOptions};
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Button};
use ReSet_Lib::network::connection::Connection;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetWifiOptions.ui")]
pub struct WifiOptions {
    // General
    #[template_child]
    pub resetWifiName: TemplateChild<ActionRow>,
    #[template_child]
    pub resetWifiMac: TemplateChild<ActionRow>,
    #[template_child]
    pub resetWifiLinkSpeed: TemplateChild<ActionRow>,
    #[template_child]
    pub resetWifiIP4Addr: TemplateChild<ActionRow>,
    #[template_child]
    pub resetWifiIP6Addr: TemplateChild<ActionRow>,
    #[template_child]
    pub resetWifiGateway: TemplateChild<ActionRow>,
    #[template_child]
    pub resetWifiDNS: TemplateChild<ActionRow>,
    #[template_child]
    pub resetWifiLastUsed: TemplateChild<ActionRow>,
    #[template_child]
    pub resetWifiAutoConnect: TemplateChild<SwitchRow>,
    #[template_child]
    pub resetWifiMetered: TemplateChild<SwitchRow>,
    // IPv4
    #[template_child]
    pub resetIP4Method: TemplateChild<ComboRow>,
    #[template_child]
    pub resetIP4DNS: TemplateChild<EntryRow>,
    #[template_child]
    pub resetIP4Gateway: TemplateChild<EntryRow>,
    #[template_child]
    pub resetIP4AddressGroup: TemplateChild<PreferencesGroup>,
    #[template_child]
    pub resetIP4RoutesGroup: TemplateChild<PreferencesGroup>,
    // IPv6
    #[template_child]
    pub resetIP6Method: TemplateChild<ComboRow>,
    #[template_child]
    pub resetIP6DNS: TemplateChild<EntryRow>,
    #[template_child]
    pub resetIP6Gateway: TemplateChild<EntryRow>,
    #[template_child]
    pub resetIP6AddressGroup: TemplateChild<PreferencesGroup>,
    #[template_child]
    pub resetIP6RoutesGroup: TemplateChild<PreferencesGroup>,
    // Security
    // Misc
    #[template_child]
    pub wifiOptionsApplyButton: TemplateChild<Button>,
    pub connection: Rc<RefCell<Connection>>
}

#[glib::object_subclass]
impl ObjectSubclass for WifiOptions {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetWifiOptions";
    type Type = wifiOptions::WifiOptions;
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
