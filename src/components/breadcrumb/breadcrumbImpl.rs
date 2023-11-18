use std::cell::{Cell, RefCell};
use glib::{Properties, StaticType, StaticTypeExt};
use glib::subclass::Signal;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use once_cell::sync::Lazy;
use gtk::prelude::*;
use crate::components::breadcrumb::breadcrumb;
use crate::components::breadcrumb::breadcrumbItem::BreadcrumbItem;

#[allow(non_snake_case)]
#[derive(Properties, Default, CompositeTemplate)]
#[properties(wrapper_type = breadcrumb::Breadcrumb)]
#[template(resource = "/org/Xetibo/ReSet/resetBreadcrumb.ui")]
pub struct Breadcrumb {
    #[template_child]
    pub resetBox: TemplateChild<gtk::Box>,
    pub items: RefCell<i32>,
    #[property(get, set)]
    number2: Cell<i32>,
}

#[glib::object_subclass]
impl ObjectSubclass for Breadcrumb {
    const NAME: &'static str = "resetBreadcrumb";
    type Type = breadcrumb::Breadcrumb;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        BreadcrumbItem::ensure_type();
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for Breadcrumb {
    fn signals () -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("max-number-reached")
                .param_types([i32::static_type()])
                .build()]
        });
        SIGNALS.as_ref()
    }

    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl BoxImpl for Breadcrumb {}

impl WidgetImpl for Breadcrumb {}

impl WindowImpl for Breadcrumb {}

impl ApplicationWindowImpl for Breadcrumb {}
