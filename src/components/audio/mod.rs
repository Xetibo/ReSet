#![allow(non_snake_case)]

mod audioSource;
mod audioBox;

use adw::glib::Object;
use gtk::{glib};

glib::wrapper! {
    pub struct AudioBox(ObjectSubclass<audioBox::AudioBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

glib::wrapper! {
    pub struct AudioSourceEntry(ObjectSubclass<audioSource::AudioSourceEntry>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl AudioBox {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl AudioSourceEntry {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
