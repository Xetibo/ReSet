use adw::glib;
use adw::glib::Object;

use super::outputStreamEntryImpl;

glib::wrapper! {
    pub struct OutputStreamEntry(ObjectSubclass<outputStreamEntryImpl::OutputStreamEntry>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl OutputStreamEntry {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
