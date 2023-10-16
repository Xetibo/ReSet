mod imp;

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
        self.imp()
            .resetSearchEntry
            .connect_search_changed(clone!(@ weak self as window => move |_| {
                window.setText();
                window.filterList();
            }));
    }

    fn setText(&self) {
        let buffer = self.imp()
            .resetSearchEntry
            .text()
            .to_string();
        self.imp()
            .test
            .set_text(&buffer);
    }

    fn filterList(&self) {
        let text = self.imp()
            .resetSearchEntry
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
}