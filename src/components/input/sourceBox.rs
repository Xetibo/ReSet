use crate::components::input::sourceBoxImpl;
use adw::glib;
use adw::glib::Object;
use glib::subclass::prelude::ObjectSubclassIsExt;
use glib::Variant;
use gtk::prelude::ActionableExt;

glib::wrapper! {
    pub struct SourceBox(ObjectSubclass<sourceBoxImpl::SourceBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl SourceBox {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn setupCallbacks(&self) {
        let selfImp = self.imp();
        selfImp.resetSourceRow.set_action_name(Some("navigation.push"));
        selfImp.resetSourceRow.set_action_target_value(Some(&Variant::from("sources")));

        selfImp.resetOutputStreamButton.set_action_name(Some("navigation.pop"));
    }
}
