mod imp;

use adw::BreakpointCondition;
use adw::glib::clone;
use adw::subclass::prelude::ObjectSubclassIsExt;
use glib::Object;
use gtk::{gio, glib, Application};
use gtk::prelude::*;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

#[allow(non_snake_case)]
impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setupCallback(&self) {
        self.imp().resetSearchEntry
            .connect_search_changed(clone!(@ weak self as window => move |_| {
                window.filterList();
            }));

        self.imp().resetSideBarToggle
            .connect_clicked(clone!(@ weak self as window => move |_| {
                window.toggleSidebar();
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
        let text = self.imp().resetSearchEntry
            .text()
            .to_string();
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