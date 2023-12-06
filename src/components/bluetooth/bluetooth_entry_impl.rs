use crate::components::bluetooth::bluetooth_entry;
use adw::subclass::action_row::ActionRowImpl;
use adw::subclass::preferences_row::PreferencesRowImpl;
use adw::ActionRow;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Image, Label};
use std::cell::RefCell;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetBluetoothEntry.ui")]
pub struct BluetoothEntry {
    #[template_child]
    pub reset_bluetooth_device_type: TemplateChild<Image>,
    #[template_child]
    pub reset_bluetooth_label: TemplateChild<Label>,
    #[template_child]
    pub reset_bluetooth_address: TemplateChild<Label>,
    #[template_child]
    pub reset_bluetooth_button: TemplateChild<Button>,
    pub device_name: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for BluetoothEntry {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetBluetoothEntry";
    type Type = bluetooth_entry::BluetoothEntry;
    type ParentType = ActionRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for BluetoothEntry {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl ActionRowImpl for BluetoothEntry {}

impl PreferencesRowImpl for BluetoothEntry {}

impl ListBoxRowImpl for BluetoothEntry {}

impl WidgetImpl for BluetoothEntry {}

impl WindowImpl for BluetoothEntry {}

impl ApplicationWindowImpl for BluetoothEntry {}
