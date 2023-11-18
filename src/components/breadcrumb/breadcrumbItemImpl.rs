use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Button};
use crate::components::breadcrumb::breadcrumbItem;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetBreadcrumbItem.ui")]
pub struct BreadcrumbItem {
    #[template_child]
    pub resetBreadcrumbButtonName: TemplateChild<Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for BreadcrumbItem {
    const NAME: &'static str = "resetBreadcrumbItem";
    type Type = breadcrumbItem::BreadcrumbItem;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for BreadcrumbItem {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl BoxImpl for BreadcrumbItem {}

impl WidgetImpl for BreadcrumbItem {}

impl WindowImpl for BreadcrumbItem {}

impl ApplicationWindowImpl for BreadcrumbItem {}
