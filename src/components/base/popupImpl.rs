use adw::subclass::window::AdwWindowImpl;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use super::popup;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetPopup.ui")]
pub struct Popup {}

#[glib::object_subclass]
impl ObjectSubclass for Popup {
    const NAME: &'static str = "resetPopup";
    type Type = popup::Popup;
    type ParentType = adw::Window;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Popup {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl BoxImpl for Popup {}

impl WidgetImpl for Popup {}

impl AdwWindowImpl for Popup {}

impl WindowImpl for Popup {}

impl ApplicationWindowImpl for Popup {}
