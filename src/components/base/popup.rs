use adw::glib;
use adw::glib::{IsA, Object};
use gtk::Widget;

use super::popupImpl;

glib::wrapper! {
    pub struct Popup(ObjectSubclass<popupImpl::Popup>)
    @extends adw::Window, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Popup {
    pub fn new(child: &impl IsA<Widget>) -> Self {
        let popup: Popup = Object::builder().build();
        // popup.set_child(child);
        popup
    }
}
