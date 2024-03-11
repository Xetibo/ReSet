use crate::components::base::popup::Popup;
use crate::components::wifi::wifi_entry;
use adw::subclass::preferences_row::PreferencesRowImpl;
use adw::subclass::prelude::ActionRowImpl;
use adw::ActionRow;
use gtk::subclass::prelude::*;
use gtk::{Button, CompositeTemplate, Image, Label};
use re_set_lib::network::network_structures::{AccessPoint, WifiStrength};
use std::cell::RefCell;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetWifiEntry.ui")]
pub struct WifiEntry {
    #[template_child]
    pub reset_wifi_popup: TemplateChild<Popup>,
    pub reset_wifi_strength: RefCell<Image>,
    pub reset_wifi_encrypted: RefCell<Image>,
    pub reset_wifi_connected: RefCell<Label>,
    pub reset_wifi_edit_button: RefCell<Button>,
    pub wifi_name: RefCell<String>,
    pub wifi_strength: RefCell<WifiStrength>,
    pub access_point: RefCell<AccessPoint>,
    pub connected: RefCell<bool>,
}

unsafe impl Send for WifiEntry {}
unsafe impl Sync for WifiEntry {}

#[glib::object_subclass]
impl ObjectSubclass for WifiEntry {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetWifiEntry";
    type Type = wifi_entry::WifiEntry;
    type ParentType = ActionRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for WifiEntry {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl PreferencesRowImpl for WifiEntry {}

impl ListBoxRowImpl for WifiEntry {}

impl ActionRowImpl for WifiEntry {}

impl WidgetImpl for WifiEntry {}

impl WindowImpl for WifiEntry {}

impl ApplicationWindowImpl for WifiEntry {}
