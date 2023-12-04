use std::cell::RefCell;
use std::sync::Arc;

use adw::glib::StaticTypeExt;
use adw::subclass::prelude::AdwApplicationWindowImpl;
use adw::{Breakpoint, OverlaySplitView};
use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, FlowBox, ListBox, PopoverMenu, SearchEntry};

use crate::components::base::utils::Listeners;
use crate::components::wifi::wifiBox::WifiBox;
use crate::components::window::resetWindow;
use crate::components::window::sidebarEntry::SidebarEntry;

#[allow(non_snake_case)]
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/Xetibo/ReSet/resetMainWindow.ui")]
pub struct ReSetWindow {
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
    pub resetPopoverMenu: TemplateChild<PopoverMenu>,
    #[template_child]
    pub resetClose: TemplateChild<Button>,
    #[template_child]
    pub resetAboutButton: TemplateChild<Button>,
    #[template_child]
    pub resetPreferenceButton: TemplateChild<Button>,
    #[template_child]
    pub resetShortcutsButton: TemplateChild<Button>,
    pub sidebarEntries: RefCell<Vec<(SidebarEntry, Vec<SidebarEntry>)>>,
    pub listeners: Arc<Listeners>,
}

unsafe impl Send for ReSetWindow {}
unsafe impl Sync for ReSetWindow {}

#[glib::object_subclass]
impl ObjectSubclass for ReSetWindow {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetUI";
    type Type = resetWindow::ReSetWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        WifiBox::ensure_type();
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ReSetWindow {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setupCallback();
        obj.setupPopoverButtons();
        obj.handleDynamicSidebar();
        obj.setupSidebarEntries();
    }
}

impl WidgetImpl for ReSetWindow {}

impl WindowImpl for ReSetWindow {}

impl ApplicationWindowImpl for ReSetWindow {}

impl AdwApplicationWindowImpl for ReSetWindow {}
