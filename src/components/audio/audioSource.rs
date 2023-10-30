use adw::glib;
use adw::glib::Object;
use crate::components::audio::audioSourceImpl;

glib::wrapper! {
    pub struct AudioSourceEntry(ObjectSubclass<audioSourceImpl::AudioSourceEntry>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl AudioSourceEntry {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
