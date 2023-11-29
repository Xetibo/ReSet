use dbus::Path;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, ListBox, Switch};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use adw::ActionRow;

use crate::components::base::listEntry::ListEntry;
use crate::components::bluetooth::bluetoothBox;
use crate::components::bluetooth::bluetoothEntry::BluetoothEntry;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetBluetooth.ui")]
pub struct BluetoothBox {
    #[template_child]
    pub resetBluetoothSwitch: TemplateChild<Switch>,
    #[template_child]
    pub resetBluetoothAvailableDevices: TemplateChild<ListBox>,
    #[template_child]
    pub resetBluetoothConnectedDevices: TemplateChild<ListBox>,
    #[template_child]
    pub resetVisibility: TemplateChild<ActionRow>,
    #[template_child]
    pub resetBluetoothMainTab: TemplateChild<ListEntry>,
    pub availableDevices: RefCell<HashMap<Path<'static>, (Arc<BluetoothEntry>, Arc<ListEntry>)>>,
    pub connectedDevices: RefCell<HashMap<Path<'static>, (Arc<BluetoothEntry>, Arc<ListEntry>)>>,
}

#[glib::object_subclass]
impl ObjectSubclass for BluetoothBox {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetBluetooth";
    type Type = bluetoothBox::BluetoothBox;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        BluetoothEntry::ensure_type();
        ListEntry::ensure_type();
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for BluetoothBox {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.setupCallbacks();
    }
}

impl BoxImpl for BluetoothBox {}

impl WidgetImpl for BluetoothBox {}

impl WindowImpl for BluetoothBox {}

impl ApplicationWindowImpl for BluetoothBox {}
