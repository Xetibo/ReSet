use crate::components::base::setting_box;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetSettingBox.ui")]
pub struct SettingBox {}

#[glib::object_subclass]
impl ObjectSubclass for SettingBox {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetSettingBox";
    type Type = setting_box::SettingBox;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for SettingBox {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl BoxImpl for SettingBox {}

impl WidgetImpl for SettingBox {}

impl WindowImpl for SettingBox {}

impl ApplicationWindowImpl for SettingBox {}
