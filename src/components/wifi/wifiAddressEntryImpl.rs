use adw::{EntryRow, ExpanderRow};
use crate::components::wifi::{wifiAddressEntry};
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Button};

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetWifiAddressEntry.ui")]
pub struct WifiAddressEntryImpl {
    #[template_child]
    pub resetAddressRow: TemplateChild<ExpanderRow>,
    #[template_child]
    pub resetAddressAddress: TemplateChild<EntryRow>,
    #[template_child]
    pub resetAddressNetmask: TemplateChild<EntryRow>,
    #[template_child]
    pub resetAddressRemove: TemplateChild<Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for WifiAddressEntryImpl {
    const NAME: &'static str = "resetWifiAddressEntry";
    type Type = wifiAddressEntry::WifiAddressEntry;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for WifiAddressEntryImpl {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl BoxImpl for WifiAddressEntryImpl {}

impl WidgetImpl for WifiAddressEntryImpl {}

impl WindowImpl for WifiAddressEntryImpl {}

impl ApplicationWindowImpl for WifiAddressEntryImpl {}
