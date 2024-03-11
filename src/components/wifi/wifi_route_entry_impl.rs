use crate::components::wifi::utils::IpProtocol;
use crate::components::wifi::wifi_route_entry;
use adw::{EntryRow, ExpanderRow};
use gtk::subclass::prelude::*;
use gtk::{Button, CompositeTemplate};
use std::cell::{Cell, RefCell};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetWifiRouteEntry.ui")]
pub struct WifiRouteEntryImpl {
    #[template_child]
    pub reset_route_row: TemplateChild<ExpanderRow>,
    #[template_child]
    pub reset_route_address: TemplateChild<EntryRow>,
    #[template_child]
    pub reset_route_prefix: TemplateChild<EntryRow>,
    #[template_child]
    pub reset_route_gateway: TemplateChild<EntryRow>,
    #[template_child]
    pub reset_route_metric: TemplateChild<EntryRow>,
    #[template_child]
    pub reset_route_remove: TemplateChild<Button>,
    pub address: RefCell<(bool, String)>,
    pub prefix: Cell<(bool, u32)>,
    pub gateway: RefCell<Option<String>>,
    pub metric: Cell<Option<u32>>,
    pub protocol: Cell<IpProtocol>,
}

#[glib::object_subclass]
impl ObjectSubclass for WifiRouteEntryImpl {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetWifiRouteEntry";
    type Type = wifi_route_entry::WifiRouteEntry;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for WifiRouteEntryImpl {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl BoxImpl for WifiRouteEntryImpl {}

impl WidgetImpl for WifiRouteEntryImpl {}

impl WindowImpl for WifiRouteEntryImpl {}

impl ApplicationWindowImpl for WifiRouteEntryImpl {}
