use adw::subclass::preferences_row::PreferencesRowImpl;
use adw::subclass::prelude::ActionRowImpl;
use adw::ActionRow;
use std::cell::RefCell;

use dbus::Path;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Label};
use ReSet_Lib::network::network::AccessPoint;

use super::savedWifiEntry;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetSavedWifiEntry.ui")]
pub struct SavedWifiEntry {
    #[template_child]
    pub resetDeleteSavedWifiButton: TemplateChild<Button>,
    #[template_child]
    pub resetEditSavedWifiButton: TemplateChild<Button>,
    #[template_child]
    pub resetSavedWifiLabel: TemplateChild<Label>,
    pub resetConnectionPath: RefCell<Path<'static>>,
    pub accessPoint: RefCell<AccessPoint>,
}

unsafe impl Send for SavedWifiEntry {}
unsafe impl Sync for SavedWifiEntry {}

#[glib::object_subclass]
impl ObjectSubclass for SavedWifiEntry {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetSavedWifiEntry";
    type Type = savedWifiEntry::SavedWifiEntry;
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
