use std::cell::{Cell, RefCell};
use std::sync::Arc;

use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, FlowBox, Image, Label, ListBoxRow};

use crate::components::base::utils::Listeners;
use crate::components::window::handleSidebarClick::HANDLE_HOME;
use crate::components::window::sidebarEntry;

#[derive(Default)]
pub enum Categories {
    Connectivity,
    Audio,
    Peripherals,
    #[default]
    Misc,
}

#[allow(non_snake_case)]
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/Xetibo/ReSet/resetSidebarEntry.ui")]
pub struct SidebarEntry {
    #[template_child]
    pub resetSidebarLabel: TemplateChild<Label>,
    #[template_child]
    pub resetSidebarImage: TemplateChild<Image>,
    pub category: Cell<Categories>,
    pub isSubcategory: Cell<bool>,
    pub onClickEvent: RefCell<SidebarAction>,
    pub name: RefCell<String>,
}

#[allow(non_snake_case)]
pub struct SidebarAction {
    pub onClickEvent: fn(Arc<Listeners>, FlowBox),
}

impl Default for SidebarAction {
    fn default() -> Self {
        Self {
            onClickEvent: HANDLE_HOME,
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for SidebarEntry {
    const NAME: &'static str = "resetSidebarEntry";
    type Type = sidebarEntry::SidebarEntry;
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
