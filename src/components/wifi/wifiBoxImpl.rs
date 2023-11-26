use crate::components::wifi::wifiBox;
use adw::{ActionRow, ComboRow, NavigationView, PreferencesGroup};
use dbus::Path;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Switch};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::components::base::listEntry::ListEntry;
use crate::components::wifi::wifiEntry::WifiEntry;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetWiFi.ui")]
pub struct WifiBox {
    #[template_child]
    pub resetWifiNavigation: TemplateChild<NavigationView>,
    #[template_child]
    pub resetWifiDetails: TemplateChild<PreferencesGroup>,
    #[template_child]
    pub resetWiFiDevice: TemplateChild<ComboRow>,
    #[template_child]
    pub resetSavedNetworks: TemplateChild<ActionRow>,
    #[template_child]
    pub resetWifiSwitch: TemplateChild<Switch>,
    #[template_child]
    pub resetWifiList: TemplateChild<PreferencesGroup>,
    #[template_child]
    pub resetStoredWifiList: TemplateChild<PreferencesGroup>,
    #[template_child]
    pub resetAvailableNetworks: TemplateChild<ActionRow>,
    pub wifiEntries: Arc<Mutex<HashMap<Vec<u8>, Arc<WifiEntry>>>>,
    pub wifiEntriesPath: Arc<Mutex<HashMap<Path<'static>, Arc<WifiEntry>>>>,
    pub savedWifiEntries: Arc<Mutex<Vec<ListEntry>>>,
}

unsafe impl Send for WifiBox {}
unsafe impl Sync for WifiBox {}

#[glib::object_subclass]
impl ObjectSubclass for WifiBox {
    const ABSTRACT: bool = false;
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
