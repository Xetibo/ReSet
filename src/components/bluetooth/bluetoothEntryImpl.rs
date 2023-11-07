use std::cell::RefCell;
use gtk::{Button, CompositeTemplate, glib, Image, Label};
use gtk::subclass::prelude::*;
use crate::components::bluetooth::bluetoothEntry;

#[derive(Default, Copy, Clone)]
pub enum DeviceTypes {
    Mouse,
    Keyboard,
    Headset,
    Controller,
    #[default]
    None,
}
#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetBluetoothEntry.ui")]
pub struct BluetoothEntry {
    #[template_child]
    pub resetBluetoothDeviceType: TemplateChild<Image>,
    #[template_child]
    pub resetBluetoothLabel: TemplateChild<Label>,
    #[template_child]
    pub resetBluetoothButton: TemplateChild<Button>,
    pub deviceName: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for BluetoothEntry {
    const NAME: &'static str = "resetBluetoothEntry";
    type Type = bluetoothEntry::BluetoothEntry;
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
