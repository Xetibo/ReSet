use adw::glib::StaticTypeExt;
use adw::subclass::prelude::AdwApplicationWindowImpl;
use adw::{Breakpoint, OverlaySplitView};
use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, Box, Button, CompositeTemplate, FlowBox, ListBox, SearchEntry, PopoverMenu};
use std::cell::RefCell;

use crate::components::wifi::WifiBox;
use crate::components::window::SidebarEntry;

#[allow(non_snake_case)]
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/xetibo/reset/resetMainWindow.ui")]
pub struct Window {
    #[template_child]
    pub resetMain: TemplateChild<FlowBox>,
    #[template_child]
    pub resetSidebarBreakpoint: TemplateChild<Breakpoint>,
    #[template_child]
    pub resetOverlaySplitView: TemplateChild<OverlaySplitView>,
    #[template_child]
    pub resetSearchEntry: TemplateChild<SearchEntry>,
    #[template_child]
    pub resetSidebarList: TemplateChild<ListBox>,
    #[template_child]
    pub resetSideBarToggle: TemplateChild<Button>,
    #[template_child]
    pub resetPath: TemplateChild<Box>,
    #[template_child]
    pub resetPopoverMenu: TemplateChild<PopoverMenu>,
    #[template_child]
    pub resetClose: TemplateChild<Button>,
    #[template_child]
    pub resetAboutButton: TemplateChild<Button>,
    pub sidebarEntries: RefCell<Vec<(SidebarEntry, Vec<SidebarEntry>)>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "resetUI";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        WifiBox::ensure_type();
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let object = self.obj();
        object.setupCallback();
        object.setupPopoverButtons();
        object.handleDynamicSidebar();
        object.setupSidebarEntries();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}

impl AdwApplicationWindowImpl for Window {}
