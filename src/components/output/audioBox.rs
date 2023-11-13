use adw::glib;
use adw::glib::Object;
use glib::subclass::prelude::ObjectSubclassIsExt;
use glib::Variant;
use gtk::prelude::ActionableExt;
use crate::components::output::audioBoxImpl;

glib::wrapper! {
    pub struct AudioBox(ObjectSubclass<audioBoxImpl::AudioBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl AudioBox {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn setupCallbacks(&self) {
        let selfImp = self.imp();
        selfImp.resetSinksRow.set_action_name(Some("navigation.push"));
        selfImp.resetSinksRow.set_action_target_value(Some(&Variant::from("outputDevices")));

        selfImp.resetOutputStreamButton.set_action_name(Some("navigation.pop"));
    }
}
