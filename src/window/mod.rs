#![allow(non_snake_case)]

use adw::BreakpointCondition;
use adw::glib::clone;
use adw::subclass::prelude::ObjectSubclassIsExt;
use glib::Object;
use gtk::{Application, FlowBox, gio, glib};
use gtk::prelude::*;

use crate::window::sidebarEntry::{Categories, SidebarAction};

mod window;
mod sidebarEntry;
mod handleSidebarClick;

glib::wrapper! {
    pub struct Window(ObjectSubclass<window::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

glib::wrapper! {
    pub struct SidebarEntry(ObjectSubclass<sidebarEntry::SidebarEntry>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

#[allow(non_snake_case)]
impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setupCallback(&self) {
        let selfImp = self.imp();

        selfImp.resetSearchEntry
            .connect_search_changed(clone!(@ weak self as window => move |_| {
                window.filterList();
            }));

        selfImp.resetSideBarToggle
            .connect_clicked(clone!(@ weak self as window => move |_| {
                window.toggleSidebar();
            }));

        selfImp.resetSidebarList.connect_row_activated(clone!(@ weak selfImp as flowbox => move |x, y| {
            let mut result = y.downcast_ref::<SidebarEntry>().unwrap();
            let x1 = result.imp().onClickEvent.borrow().onClickEvent;
            (x1)(flowbox.resetMain.get());
        }));
    }

    fn handleDynamicSidebar(&self) {
        self.imp().resetSidebarBreakpoint
            .set_condition(BreakpointCondition::parse("max-width: 500sp").as_ref().ok());
        self.imp().resetSidebarBreakpoint
            .add_setter(&Object::from(self.imp().resetOverlaySplitView.get()),
                        "collapsed",
                        &true.to_value());
        self.imp().resetSidebarBreakpoint
            .add_setter(&Object::from(self.imp().resetSideBarToggle.get()),
                        "visible",
                        &true.to_value());
    }

    fn filterList(&self) {
        let text = self.imp().resetSearchEntry.text().to_string();
        self.imp().resetSidebarList.set_filter_func(move |x| {
            if text == "" {
                return true;
            }
            if let Some(child) = x.child() {
                let result = child.downcast::<gtk::Box>().unwrap();
                let label = result.last_child().unwrap().downcast::<gtk::Label>().unwrap();
                if label.text().to_lowercase().contains(&text.to_lowercase()) {
                    return true;
                }
            }
            return false;
        });
    }

    fn toggleSidebar(&self) {
        if self.imp().resetOverlaySplitView.shows_sidebar() {
            self.imp().resetOverlaySplitView
                .set_show_sidebar(false);
        } else {
            self.imp().resetOverlaySplitView
                .set_show_sidebar(true);
        }
    }
}

impl SidebarEntry {
    pub fn new(entryName: &str, iconName: &str, category: Categories, isSubcategory: bool, clickEvent: fn(FlowBox)) -> Self {
        let entry: SidebarEntry = Object::builder().build();
        let entryImp = entry.imp();
        entryImp.resetSidebarLabel.get().set_text(entryName);
        entryImp.resetSidebarImage.set_from_icon_name(Some(iconName));
        entryImp.category.set(category);
        entryImp.isSubcategory.set(isSubcategory);
        {
            let mut ref_mut = entryImp.onClickEvent.borrow_mut();
            *ref_mut = SidebarAction { onClickEvent: clickEvent };
        }
        entry
    }
}