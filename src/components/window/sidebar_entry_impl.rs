use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;

use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, FlowBox, Image, Label, ListBoxRow};

use crate::components::base::utils::{Listeners, Position};
use crate::components::window::handle_sidebar_click::HANDLE_HOME;
use crate::components::window::sidebar_entry;

#[derive(Default)]
pub enum Categories {
    Connectivity,
    Audio,
    Peripherals,
    #[default]
    Misc,
}

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/Xetibo/ReSet/resetSidebarEntry.ui")]
pub struct SidebarEntry {
    #[template_child]
    pub reset_sidebar_label: TemplateChild<Label>,
    #[template_child]
    pub reset_sidebar_image: TemplateChild<Image>,
    pub category: Cell<Categories>,
    pub is_subcategory: Cell<bool>,
    pub on_click_event: RefCell<SidebarAction>,
    pub name: RefCell<String>,
}

pub struct SidebarAction {
    pub on_click_event: fn(Arc<Listeners>, FlowBox, Rc<RefCell<Position>>),
}

impl Default for SidebarAction {
    fn default() -> Self {
        Self {
            on_click_event: HANDLE_HOME,
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for SidebarEntry {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetSidebarEntry";
    type Type = sidebar_entry::SidebarEntry;
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
