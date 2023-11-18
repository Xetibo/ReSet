use crate::components::breadcrumb::breadcrumbItemImpl;

use adw::glib;
use adw::glib::Object;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::ButtonExt;

glib::wrapper! {
    pub struct BreadcrumbItem(ObjectSubclass<breadcrumbItemImpl::BreadcrumbItem>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget;
}

impl BreadcrumbItem {
    pub fn new(name : &str) -> Self {
        let entry: BreadcrumbItem = Object::builder().build();
        let entryImp = entry.imp();
        entryImp.resetBreadcrumbButtonName.set_label(name);
        entry
    }
}
