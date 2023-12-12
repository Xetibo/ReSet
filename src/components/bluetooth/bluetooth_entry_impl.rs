use crate::components::bluetooth::bluetooth_entry;
use adw::subclass::action_row::ActionRowImpl;
use adw::subclass::preferences_row::PreferencesRowImpl;
use adw::ActionRow;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Label};
use re_set_lib::bluetooth::bluetooth_structures::BluetoothDevice;
use std::cell::RefCell;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetBluetoothEntry.ui")]
pub struct BluetoothEntry {
    pub remove_device_button: RefCell<Button>,
    pub connecting_label: RefCell<Label>,
    pub device_name: RefCell<String>,
    pub bluetooth_device: RefCell<BluetoothDevice>,
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
