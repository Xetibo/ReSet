use std::cell::RefCell;
use gtk::{Button, CompositeTemplate, glib, Image, Label};
use gtk::subclass::prelude::*;
use crate::components::temp::listEntry::ListEntry;
use crate::components::wifi::wifiEntry;

#[derive(Default, Copy, Clone)]
pub enum WifiStrength {
    Excellent,
    Ok,
    Weak,
    #[default]
    None,
}

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetWifiEntry.ui")]
pub struct WifiEntry {
    #[template_child]
    pub resetWifiStrength: TemplateChild<Image>,
    #[template_child]
    pub resetWifiEncrypted: TemplateChild<Image>,
    #[template_child]
    pub resetWifiLabel: TemplateChild<Label>,
    #[template_child]
    pub resetWifiButton: TemplateChild<Button>,
    pub wifiName: RefCell<String>,
    pub wifiStrength: RefCell<WifiStrength>,
}

#[glib::object_subclass]
impl ObjectSubclass for WifiEntry {
    const NAME: &'static str = "resetWifiEntry";
    type Type = wifiEntry::WifiEntry;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for WifiEntry {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl BoxImpl for WifiEntry {}

impl WidgetImpl for WifiEntry {}

impl WindowImpl for WifiEntry {}

impl ApplicationWindowImpl for WifiEntry {}
