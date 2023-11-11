use crate::components::base::settingBoxImpl;
use adw::glib;
use adw::glib::{IsA, Object};
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::FrameExt;
use gtk::Widget;

glib::wrapper! {
    pub struct SettingBox(ObjectSubclass<settingBoxImpl::SettingBox>)
    @extends gtk::Frame, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl SettingBox {
    pub fn new(child: &impl IsA<Widget>, title: &str) -> Self {
        let entry: SettingBox = Object::builder().build();
        entry.set_child(Some(child));
        entry.imp().resetSettingLabel.set_text(title);
        entry
    }
}