#![allow(non_snake_case)]

mod audioSource;

use adw::glib::Object;
use gtk::{glib};

glib::wrapper! {
    pub struct AudioSourceEntry(ObjectSubclass<audioSource::AudioSourceEntry>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}


impl AudioSourceEntry {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
