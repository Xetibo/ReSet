use std::rc::Rc;

use crate::components::window::sidebar_entry_impl;
use crate::components::window::sidebar_entry_impl::{SidebarAction};
use adw::subclass::prelude::ObjectSubclassIsExt;
use glib::Object;
use gtk::prelude::*;
use crate::components::plugin::function::{PluginSidebarInfo, ReSetSidebarInfo};

use super::handle_sidebar_click::HANDLE_HOME;

glib::wrapper! {
    pub struct SidebarEntry(ObjectSubclass<sidebar_entry_impl::SidebarEntry>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl SidebarEntry {
    // TODO: refactor new and new_plugin
    pub fn new(info: &ReSetSidebarInfo) -> Self {
        let entry: SidebarEntry = Object::builder().build();
        let entry_imp = entry.imp();
        entry_imp.reset_sidebar_label.get().set_text(info.name);
        entry_imp
            .reset_sidebar_image
            .set_from_icon_name(Some(info.icon_name));
        
        match &info.parent {
            None => {}
            Some(parent) => {
                let mut name = entry_imp.parent.borrow_mut();
                *name = parent.to_string();
                entry.child().unwrap().set_margin_start(30);
            }
        }
        
        {
            let mut name = entry_imp.name.borrow_mut();
            *name = info.name.to_string();
            let mut action = entry_imp.on_click_event.borrow_mut();
            *action = SidebarAction {
                on_click_event: Some(info.click_event),
                on_plugin_click_event: Rc::new(|_,_,_|{}),
            };
        }
        entry
    }
    
    pub fn new_plugin(info: &PluginSidebarInfo) -> Self {
        let entry: SidebarEntry = Object::builder().build();
        let entry_imp = entry.imp();
        entry_imp.reset_sidebar_label.get().set_text(info.name);
        entry_imp
            .reset_sidebar_image
            .set_from_icon_name(Some(info.icon_name));
        entry_imp.plugin_boxes.borrow_mut().extend(info.plugin_boxes.clone());
        
        match &info.parent {
            None => {}
            Some(parent) => {
                let mut name = entry_imp.parent.borrow_mut();
                *name = parent.to_string();
                entry.child().unwrap().set_margin_start(30);
            }
        }
        
        {
            let mut name = entry_imp.name.borrow_mut();
            *name = info.name.to_string();
            let mut action = entry_imp.on_click_event.borrow_mut();
            *action = SidebarAction {
                on_click_event: None,
                on_plugin_click_event: info.click_event.clone(),
            };
        }
        entry
    }
}
