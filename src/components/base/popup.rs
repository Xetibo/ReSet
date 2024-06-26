use adw::glib::Object;
use gtk::{gdk, Editable, Popover};

use super::popup_impl;

glib::wrapper! {
    pub struct Popup(ObjectSubclass<popup_impl::Popup>)
    @extends Popover, gtk::Widget,
    @implements Editable,gdk::Popup, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Popup {
    pub fn new() -> Self {
        let popup: Popup = Object::builder().build();
        popup
    }
}

impl Default for Popup {
    fn default() -> Self {
        Self::new()
    }
}
