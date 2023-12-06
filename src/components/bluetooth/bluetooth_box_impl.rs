use adw::{ActionRow, ComboRow, PreferencesGroup};
use dbus::Path;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Switch};
use gtk::{prelude::*, StringList};
use re_set_lib::bluetooth::bluetooth_structures::{BluetoothAdapter, BluetoothDevice};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::components::base::list_entry::ListEntry;
use crate::components::bluetooth::bluetooth_box;
use crate::components::bluetooth::bluetooth_entry::BluetoothEntry;

type BluetoothMap =
    RefCell<HashMap<Path<'static>, (Arc<BluetoothEntry>, BluetoothDevice)>>;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetBluetooth.ui")]
pub struct BluetoothBox {
    #[template_child]
    pub reset_bluetooth_switch: TemplateChild<Switch>,
    #[template_child]
    pub reset_bluetooth_available_devices: TemplateChild<PreferencesGroup>,
    #[template_child]
    pub reset_bluetooth_refresh_button: TemplateChild<Button>,
    #[template_child]
    pub reset_bluetooth_adapter: TemplateChild<ComboRow>,
    #[template_child]
    pub reset_bluetooth_connected_devices: TemplateChild<PreferencesGroup>,
    #[template_child]
    pub reset_visibility: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_bluetooth_main_tab: TemplateChild<ListEntry>,
    #[template_child]
    pub reset_bluetooth_discoverable_switch: TemplateChild<Switch>,
    #[template_child]
    pub reset_bluetooth_pairable_switch: TemplateChild<Switch>,
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
