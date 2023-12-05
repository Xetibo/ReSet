use std::sync::Arc;

use crate::components::base::utils::Listeners;
use crate::components::window::sidebar_entry_impl;
use crate::components::window::sidebar_entry_impl::{Categories, SidebarAction};
use adw::subclass::prelude::ObjectSubclassIsExt;
use glib::Object;
use gtk::prelude::*;
use gtk::{glib, FlowBox};

glib::wrapper! {
    pub struct SidebarEntry(ObjectSubclass<sidebar_entry_impl::SidebarEntry>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl SidebarEntry {
    pub fn new(
        entry_name: &str,
        icon_name: &str,
        category: Categories,
        is_subcategory: bool,
        click_event: fn(Arc<Listeners>, FlowBox),
    ) -> Self {
        let entry: SidebarEntry = Object::builder().build();
        let entry_imp = entry.imp();
        entry_imp.resetSidebarLabel.get().set_text(entry_name);
        entry_imp
            .resetSidebarImage
            .set_from_icon_name(Some(icon_name));
        entry_imp.category.set(category);
        entry_imp.is_subcategory.set(is_subcategory);
        {
            let mut name = entry_imp.name.borrow_mut();
            *name = String::from(entry_name);
            let mut action = entry_imp.on_click_event.borrow_mut();
            *action = SidebarAction {
                on_click_event: click_event,
            };
        }
        Self::set_margin(&entry);
        entry
    }

    fn set_margin(entry: &SidebarEntry) {
        if entry.imp().is_subcategory.get() {
            let option = entry.child().unwrap();
            option.set_margin_start(30);
        }
    }
}
