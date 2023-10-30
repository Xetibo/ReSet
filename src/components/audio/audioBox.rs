use adw::glib;
use adw::glib::Object;
use crate::components::audio::audioBoxImpl;

glib::wrapper! {
    pub struct AudioBox(ObjectSubclass<audioBoxImpl::AudioBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl AudioBox {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
