use crate::components::base::list_entry_impl;
use adw::glib::{Object};
use glib::prelude::IsA;
use gtk::prelude::ListBoxRowExt;
use gtk::Widget;

glib::wrapper! {
    pub struct ListEntry(ObjectSubclass<list_entry_impl::ListEntry>)
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
