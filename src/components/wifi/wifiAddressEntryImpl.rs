use std::cell::{Cell, RefCell};
use adw::{EntryRow, ExpanderRow};
use glib::once_cell::sync::Lazy;
use glib::StaticType;
use glib::subclass::Signal;
use crate::components::wifi::{wifiAddressEntry};
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Button};
use crate::components::wifi::utils::IpProtocol;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetWifiAddressEntry.ui")]
pub struct WifiAddressEntryImpl {
    #[template_child]
    pub resetAddressRow: TemplateChild<ExpanderRow>,
    #[template_child]
    pub resetAddressAddress: TemplateChild<EntryRow>,
    #[template_child]
    pub resetAddressPrefix: TemplateChild<EntryRow>,
    #[template_child]
    pub resetAddressRemove: TemplateChild<Button>,
    pub address: RefCell<(bool, String)>,
    pub prefix: Cell<(bool, u32)>,
    pub protocol: Cell<IpProtocol>,
}

#[glib::object_subclass]
impl ObjectSubclass for WifiAddressEntryImpl {
    const ABSTRACT: bool = false;
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
