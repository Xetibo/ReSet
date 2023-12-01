use crate::components::wifi::wifiRouteEntry;
use adw::{EntryRow, ExpanderRow};
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate};

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetWifiRouteEntry.ui")]
pub struct WifiRouteEntryImpl {
    #[template_child]
    pub resetRouteRow: TemplateChild<ExpanderRow>,
    #[template_child]
    pub resetRouteAddress: TemplateChild<EntryRow>,
    #[template_child]
    pub resetRouteNetmask: TemplateChild<EntryRow>,
    #[template_child]
    pub resetRouteGateway: TemplateChild<EntryRow>,
    #[template_child]
    pub resetRouteMetric: TemplateChild<EntryRow>,
    #[template_child]
    pub resetRouteRemove: TemplateChild<Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for WifiRouteEntryImpl {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetWifiRouteEntry";
    type Type = wifiRouteEntry::WifiRouteEntry;
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
