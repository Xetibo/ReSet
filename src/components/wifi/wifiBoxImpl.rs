use crate::components::wifi::wifiBox;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, ListBox, Switch};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use adw::NavigationView;

use crate::components::base::listEntry::ListEntry;
use crate::components::wifi::wifiEntry::WifiEntry;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetWiFi.ui")]
pub struct WifiBox {
    #[template_child]
    pub resetWifiNavigation: TemplateChild<NavigationView>,
    #[template_child]
    pub resetWifiDetails: TemplateChild<ListBox>,
    #[template_child]
    pub resetSavedNetworks: TemplateChild<ListEntry>,
    #[template_child]
    pub resetWifiSwitch: TemplateChild<Switch>,
    #[template_child]
    pub resetWifiList: TemplateChild<ListBox>,
    #[template_child]
    pub resetStoredWifiList: TemplateChild<ListBox>,
    #[template_child]
    pub resetAvailableNetworks: TemplateChild<ListEntry>,
    pub wifiEntries: Arc<Mutex<HashMap<Vec<u8>, Arc<ListEntry>>>>,
    pub savedWifiEntries: Arc<Mutex<Vec<ListEntry>>>,
}

unsafe impl Send for WifiBox {}
unsafe impl Sync for WifiBox {}

#[glib::object_subclass]
impl ObjectSubclass for WifiBox {
    const NAME: &'static str = "resetWifi";
    type Type = wifiBox::WifiBox;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        WifiEntry::ensure_type();
        ListEntry::ensure_type();
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for WifiBox {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setupCallbacks();
    }
}

impl BoxImpl for WifiBox {}

impl WidgetImpl for WifiBox {}

impl WindowImpl for WifiBox {}

impl ApplicationWindowImpl for WifiBox {}
