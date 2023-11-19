use std::cell::RefCell;
use std::collections::HashMap;

use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, DropDown, Label, StringList, TemplateChild};

use super::cardEntry;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetCardEntry.ui")]
pub struct CardEntry {
    #[template_child]
    pub resetCardName: TemplateChild<Label>,
    #[template_child]
    pub resetCardDropdown: TemplateChild<DropDown>,
    #[template_child]
    pub resetCardList: TemplateChild<StringList>,
    // first string is the alias name, the first return string is the index of the adapter and the
    // second the name of the profile
    pub resetCardMap: RefCell<HashMap<String, (u32, String)>>,
}

#[glib::object_subclass]
impl ObjectSubclass for CardEntry {
    const NAME: &'static str = "resetCardEntry";
    type Type = cardEntry::CardEntry;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl BoxImpl for CardEntry {}

impl ObjectImpl for CardEntry {
    fn constructed(&self) {}
}

impl ListBoxRowImpl for CardEntry {}

impl WidgetImpl for CardEntry {}

impl WindowImpl for CardEntry {}

impl ApplicationWindowImpl for CardEntry {}
