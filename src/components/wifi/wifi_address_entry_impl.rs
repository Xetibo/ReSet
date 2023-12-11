use crate::components::wifi::utils::IpProtocol;
use crate::components::wifi::wifi_address_entry;
use adw::{EntryRow, ExpanderRow};
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate};
use std::cell::{Cell, RefCell};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetWifiAddressEntry.ui")]
pub struct WifiAddressEntryImpl {
    #[template_child]
    pub reset_address_row: TemplateChild<ExpanderRow>,
    #[template_child]
    pub reset_address_address: TemplateChild<EntryRow>,
    #[template_child]
    pub reset_address_prefix: TemplateChild<EntryRow>,
    #[template_child]
    pub reset_address_remove: TemplateChild<Button>,
    pub address: RefCell<(bool, String)>,
    pub prefix: Cell<(bool, u32)>,
    pub protocol: Cell<IpProtocol>,
}

#[glib::object_subclass]
impl ObjectSubclass for WifiAddressEntryImpl {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetWifiAddressEntry";
    type Type = wifi_address_entry::WifiAddressEntry;
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
