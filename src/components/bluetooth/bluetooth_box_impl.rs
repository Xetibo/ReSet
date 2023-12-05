use adw::{ActionRow, ComboRow};
use dbus::Path;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Switch};
use gtk::{prelude::*, StringList};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use ReSet_Lib::bluetooth::bluetooth::{BluetoothAdapter, BluetoothDevice};

use crate::components::base::list_entry::ListEntry;
use crate::components::bluetooth::bluetooth_box;
use crate::components::bluetooth::bluetooth_entry::BluetoothEntry;

type BluetoothMap =
    RefCell<HashMap<Path<'static>, (Arc<BluetoothEntry>, Arc<ListEntry>, BluetoothDevice)>>;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetBluetooth.ui")]
pub struct BluetoothBox {
    #[template_child]
    pub resetBluetoothSwitch: TemplateChild<Switch>,
    #[template_child]
    pub resetBluetoothAvailableDevices: TemplateChild<gtk::Box>,
    #[template_child]
    pub resetBluetoothRefreshButton: TemplateChild<Button>,
    #[template_child]
    pub resetBluetoothAdapter: TemplateChild<ComboRow>,
    #[template_child]
    pub resetBluetoothConnectedDevices: TemplateChild<gtk::Box>,
    #[template_child]
    pub resetVisibility: TemplateChild<ActionRow>,
    #[template_child]
    pub resetBluetoothMainTab: TemplateChild<ListEntry>,
    pub available_devices: BluetoothMap,
    pub connected_devices: BluetoothMap,
    pub reset_bluetooth_adapters: Arc<RwLock<HashMap<String, (BluetoothAdapter, u32)>>>,
    pub reset_current_bluetooth_adapter: Arc<RefCell<BluetoothAdapter>>,
    pub reset_model_list: Arc<RwLock<StringList>>,
    pub reset_model_index: Arc<RwLock<u32>>,
}

#[glib::object_subclass]
impl ObjectSubclass for BluetoothBox {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetBluetooth";
    type Type = bluetooth_box::BluetoothBox;
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
    }
}

impl BoxImpl for BluetoothBox {}

impl WidgetImpl for BluetoothBox {}

impl WindowImpl for BluetoothBox {}

impl ApplicationWindowImpl for BluetoothBox {}
