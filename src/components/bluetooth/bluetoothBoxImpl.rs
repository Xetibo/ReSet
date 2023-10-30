use gtk::{CompositeTemplate, glib, ListBox, Switch};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

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
}

#[glib::object_subclass]
impl ObjectSubclass for BluetoothBox {
    const NAME: &'static str = "resetBluetooth";
    type Type = bluetoothBox::BluetoothBox;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        BluetoothEntry::ensure_type();
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
