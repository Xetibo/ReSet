use crate::components::input::inputStreamEntryImpl;
use adw::glib;
use adw::glib::Object;

glib::wrapper! {
    pub struct InputStreamEntry(ObjectSubclass<inputStreamEntryImpl::InputStreamEntry>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl InputStreamEntry {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
