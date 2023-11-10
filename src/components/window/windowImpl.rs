use std::cell::RefCell;

use adw::{Breakpoint, OverlaySplitView};
use adw::glib::StaticTypeExt;
use adw::subclass::prelude::AdwApplicationWindowImpl;
use glib::subclass::InitializingObject;
use gtk::{Box, Button, CompositeTemplate, FlowBox, glib, ListBox, PopoverMenu, SearchEntry};
use gtk::subclass::prelude::*;

use crate::components::wifi::wifiBox::WifiBox;
use crate::components::window::window;
use crate::components::window::sidebarEntry::SidebarEntry;

#[allow(non_snake_case)]
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/Xetibo/ReSet/resetMainWindow.ui")]
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

unsafe impl Send for Window {}
unsafe impl Sync for Window {}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "resetUI";
    type Type = window::Window;
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

        let obj = self.obj();
        obj.setupCallback();
        obj.setupPopoverButtons();
        obj.handleDynamicSidebar();
        obj.setupSidebarEntries();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}

impl AdwApplicationWindowImpl for Window {}
