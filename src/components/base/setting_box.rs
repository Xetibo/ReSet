use crate::components::base::setting_box_impl;
use adw::glib;
use adw::glib::{IsA, Object};
use gtk::prelude::BoxExt;
use gtk::Widget;

glib::wrapper! {
    pub struct SettingBox(ObjectSubclass<setting_box_impl::SettingBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl SettingBox {
    pub fn new(child: &impl IsA<Widget>) -> Self {
        let entry: SettingBox = Object::builder().build();
        entry.append(child);
        entry
    }
}
