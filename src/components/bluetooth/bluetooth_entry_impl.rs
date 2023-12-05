use crate::components::bluetooth::bluetooth_entry;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Image, Label};
use std::cell::RefCell;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetBluetoothEntry.ui")]
pub struct BluetoothEntry {
    #[template_child]
    pub resetBluetoothDeviceType: TemplateChild<Image>,
    #[template_child]
    pub resetBluetoothLabel: TemplateChild<Label>,
    #[template_child]
    pub resetBluetoothAddress: TemplateChild<Label>,
    #[template_child]
    pub resetBluetoothButton: TemplateChild<Button>,
    pub device_name: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for BluetoothEntry {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetBluetoothEntry";
    type Type = bluetooth_entry::BluetoothEntry;
    type ParentType = gtk::Box;

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

impl BoxImpl for BluetoothEntry {}

impl WidgetImpl for BluetoothEntry {}

impl WindowImpl for BluetoothEntry {}

impl ApplicationWindowImpl for BluetoothEntry {}
