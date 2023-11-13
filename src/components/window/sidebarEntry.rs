use std::sync::Arc;

use adw::subclass::prelude::ObjectSubclassIsExt;
use glib::Object;
use gtk::{FlowBox, glib};
use gtk::prelude::*;
use crate::components::base::utils::Listeners;
use crate::components::window::sidebarEntryImpl;
use crate::components::window::sidebarEntryImpl::{Categories, SidebarAction};

glib::wrapper! {
    pub struct SidebarEntry(ObjectSubclass<sidebarEntryImpl::SidebarEntry>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl SidebarEntry {
    pub fn new(
        entryName: &str,
        iconName: &str,
        category: Categories,
        isSubcategory: bool,
        clickEvent: fn(Arc<Listeners>, FlowBox),
    ) -> Self {
        let entry: SidebarEntry = Object::builder().build();
        let entryImp = entry.imp();
        entryImp.resetSidebarLabel.get().set_text(entryName);
        entryImp
            .resetSidebarImage
            .set_from_icon_name(Some(iconName));
        entryImp.category.set(category);
        entryImp.isSubcategory.set(isSubcategory);
        {
            let mut name = entryImp.name.borrow_mut();
            *name = String::from(entryName);
            let mut action = entryImp.onClickEvent.borrow_mut();
            *action = SidebarAction {
                onClickEvent: clickEvent,
            };
        }
        Self::setMargin(&entry);
        entry
    }

    fn setMargin(entry: &SidebarEntry) {
        if entry.imp().isSubcategory.get() {
            let option = entry.child().unwrap();
            option.set_margin_start(30);
        }
    }
}
