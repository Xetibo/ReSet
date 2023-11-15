use crate::components::base::listEntry;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetListBoxRow.ui")]
pub struct ListEntry {}

#[glib::object_subclass]
impl ObjectSubclass for ListEntry {
    const NAME: &'static str = "resetListBoxRow";
    type Type = listEntry::ListEntry;
    type ParentType = gtk::ListBoxRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ListEntry {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl ListBoxRowImpl for ListEntry {}

impl WidgetImpl for ListEntry {}

impl WindowImpl for ListEntry {}

impl ApplicationWindowImpl for ListEntry {}