use adw::subclass::preferences_row::PreferencesRowImpl;
use adw::subclass::prelude::ActionRowImpl;
use adw::ActionRow;
use re_set_lib::network::network_structures::AccessPoint;
use std::cell::RefCell;

use dbus::Path;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;

use super::saved_wifi_entry;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetSavedWifiEntry.ui")]
pub struct SavedWifiEntry {
    pub reset_connection_path: RefCell<Path<'static>>,
    pub access_point: RefCell<AccessPoint>,
}

unsafe impl Send for SavedWifiEntry {}
unsafe impl Sync for SavedWifiEntry {}

#[glib::object_subclass]
impl ObjectSubclass for SavedWifiEntry {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetSavedWifiEntry";
    type Type = saved_wifi_entry::SavedWifiEntry;
    type ParentType = ActionRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for SavedWifiEntry {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl PreferencesRowImpl for SavedWifiEntry {}

impl ListBoxRowImpl for SavedWifiEntry {}

impl ActionRowImpl for SavedWifiEntry {}

impl WidgetImpl for SavedWifiEntry {}

impl WindowImpl for SavedWifiEntry {}

impl ApplicationWindowImpl for SavedWifiEntry {}
