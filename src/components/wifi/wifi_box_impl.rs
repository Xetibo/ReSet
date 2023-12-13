use crate::components::wifi::wifi_box;
use adw::{ActionRow, ComboRow, NavigationView, PreferencesGroup};
use dbus::Path;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Switch};
use gtk::{prelude::*, StringList};
use re_set_lib::network::network_structures::WifiDevice;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use crate::components::base::list_entry::ListEntry;
use crate::components::wifi::wifi_entry::WifiEntry;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetWiFi.ui")]
pub struct WifiBox {
    #[template_child]
    pub reset_wifi_navigation: TemplateChild<NavigationView>,
    #[template_child]
    pub reset_wifi_details: TemplateChild<PreferencesGroup>,
    #[template_child]
    pub reset_wifi_device: TemplateChild<ComboRow>,
    #[template_child]
    pub reset_saved_networks: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_wifi_switch: TemplateChild<Switch>,
    #[template_child]
    pub reset_wifi_list: TemplateChild<PreferencesGroup>,
    #[template_child]
    pub reset_stored_wifi_list: TemplateChild<PreferencesGroup>,
    #[template_child]
    pub reset_available_networks: TemplateChild<ActionRow>,
    pub wifi_entries: Arc<Mutex<HashMap<Vec<u8>, Arc<WifiEntry>>>>,
    pub wifi_entries_path: Arc<Mutex<HashMap<Path<'static>, Arc<WifiEntry>>>>,
    pub reset_wifi_devices: Arc<RwLock<HashMap<String, (WifiDevice, u32)>>>,
    pub reset_current_wifi_device: Arc<RefCell<WifiDevice>>,
    pub reset_model_list: Arc<RwLock<StringList>>,
    pub reset_model_index: Arc<RwLock<u32>>,
}

unsafe impl Send for WifiBox {}
unsafe impl Sync for WifiBox {}

#[glib::object_subclass]
impl ObjectSubclass for WifiBox {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetWifi";
    type Type = wifi_box::WifiBox;
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
        obj.setup_callbacks();
    }
}

impl BoxImpl for WifiBox {}

impl WidgetImpl for WifiBox {}

impl WindowImpl for WifiBox {}

impl ApplicationWindowImpl for WifiBox {}
