use adw::subclass::action_row::ActionRowImpl;
use adw::subclass::preferences_row::PreferencesRowImpl;
use adw::subclass::prelude::ComboRowImpl;
use adw::ComboRow;
use std::cell::RefCell;
use std::collections::HashMap;

use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use super::cardEntry;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetCardEntry.ui")]
pub struct CardEntry {
    // first string is the alias name, the first return string is the index of the adapter and the
    // second the name of the profile
    pub resetCardMap: RefCell<HashMap<String, (u32, String)>>,
}

#[glib::object_subclass]
impl ObjectSubclass for CardEntry {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetCardEntry";
    type Type = cardEntry::CardEntry;
    type ParentType = ComboRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ActionRowImpl for CardEntry {}

impl PreferencesRowImpl for CardEntry {}

impl ComboRowImpl for CardEntry {}

impl ObjectImpl for CardEntry {
    fn constructed(&self) {}
}

impl ListBoxRowImpl for CardEntry {}

impl WidgetImpl for CardEntry {}

impl WindowImpl for CardEntry {}

impl ApplicationWindowImpl for CardEntry {}
