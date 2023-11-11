use gtk::{CompositeTemplate, glib, Label};
use gtk::subclass::prelude::*;
use crate::components::base::settingBox;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetSettingBox.ui")]
pub struct SettingBox {
    #[template_child]
    pub resetSettingLabel: TemplateChild<Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for SettingBox {
    const NAME: &'static str = "resetSettingFrame";
    type Type = settingBox::SettingBox;
    type ParentType = gtk::Frame;

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

impl FrameImpl for SettingBox {}

impl WidgetImpl for SettingBox {}

impl WindowImpl for SettingBox {}

impl ApplicationWindowImpl for SettingBox {}
