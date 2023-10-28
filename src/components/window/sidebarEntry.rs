use std::cell::{Cell, RefCell};

use glib::subclass::InitializingObject;
use gtk::{CompositeTemplate, FlowBox, glib, Image, Label, ListBoxRow};
use gtk::subclass::prelude::*;

use crate::components::window::handleSidebarClick::HANDLE_HOME;

#[derive(Default)]
pub enum Categories {
    Connectivity,
    Audio,
    #[default]
    Misc,
}

#[allow(non_snake_case)]
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/xetibo/reset/resetSidebarEntry.ui")]
pub struct SidebarEntry {
    #[template_child]
    pub resetSidebarLabel: TemplateChild<Label>,
    #[template_child]
    pub resetSidebarImage: TemplateChild<Image>,
    pub category: Cell<Categories>,
    pub isSubcategory: Cell<bool>,
    pub onClickEvent: RefCell<SidebarAction>,
    pub name : RefCell<String>,
}

#[allow(non_snake_case)]
pub struct SidebarAction {
    pub onClickEvent: fn(FlowBox),
}

impl Default for SidebarAction {
    fn default() -> Self {
        Self {
            onClickEvent: HANDLE_HOME
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for SidebarEntry {
    const NAME: &'static str = "resetSidebarEntry";
    type Type = super::SidebarEntry;
    type ParentType = ListBoxRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for SidebarEntry {}

impl ListBoxRowImpl for SidebarEntry {}

impl WidgetImpl for SidebarEntry {}
