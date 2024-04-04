use crate::components::plugin::function::TSideBarInfo;
use crate::components::window::sidebar_entry_impl;
use crate::components::window::sidebar_entry_impl::SidebarAction;
use adw::subclass::prelude::ObjectSubclassIsExt;
use glib::Object;
use gtk::prelude::*;

glib::wrapper! {
    pub struct SidebarEntry(ObjectSubclass<sidebar_entry_impl::SidebarEntry>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl SidebarEntry {
    pub fn new<T: TSideBarInfo>(info: &T) -> Self {
        let entry: SidebarEntry = Object::builder().build();
        let entry_imp = entry.imp();
        entry_imp.reset_sidebar_label.get().set_text(info.name());
        entry_imp
            .reset_sidebar_image
            .set_from_icon_name(Some(info.icon_name()));
        if let Some(boxes) = info.plugin_boxes() {
            entry_imp.plugin_boxes.borrow_mut().extend(boxes);
        }

        match &info.parent() {
            None => {}
            Some(parent) => {
                let mut name = entry_imp.parent.borrow_mut();
                *name = parent.to_string();
                entry.child().unwrap().set_margin_start(30);
            }
        }

        {
            let mut name = entry_imp.name.borrow_mut();
            *name = info.name().to_string();
            let mut action = entry_imp.on_click_event.borrow_mut();
            *action = SidebarAction {
                on_click_event: info.regular_click_event(),
                on_plugin_click_event: info.plugin_click_event(),
            };
        }
        entry
    }
}
