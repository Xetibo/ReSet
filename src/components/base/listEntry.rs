use crate::components::base::listEntryImpl;
use adw::glib;
use adw::glib::{IsA, Object};
use gtk::prelude::ListBoxRowExt;
use gtk::Widget;

glib::wrapper! {
    pub struct ListEntry(ObjectSubclass<listEntryImpl::ListEntry>)
    @extends gtk::ListBoxRow, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

unsafe impl Send for ListEntry {}
unsafe impl Sync for ListEntry {}

impl ListEntry {
    pub fn new(child: &impl IsA<Widget>) -> Self {
        let entry: ListEntry = Object::builder().build();
        entry.set_child(Some(child));
        entry
    }
}
