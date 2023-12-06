use std::cell::RefCell;
use std::sync::Arc;

use adw::glib::StaticTypeExt;
use adw::subclass::prelude::AdwApplicationWindowImpl;
use adw::{Breakpoint, OverlaySplitView};
use glib::subclass::InitializingObject;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, FlowBox, ListBox, PopoverMenu, SearchEntry};

use crate::components::base::utils::Listeners;
use crate::components::wifi::wifi_box::WifiBox;
use crate::components::window::reset_window;
use crate::components::window::sidebar_entry::SidebarEntry;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/Xetibo/ReSet/resetMainWindow.ui")]
pub struct ReSetWindow {
    #[template_child]
    pub reset_main: TemplateChild<FlowBox>,
    #[template_child]
    pub reset_sidebar_breakpoint: TemplateChild<Breakpoint>,
    #[template_child]
    pub reset_overlay_split_view: TemplateChild<OverlaySplitView>,
    #[template_child]
    pub reset_search_entry: TemplateChild<SearchEntry>,
    #[template_child]
    pub reset_sidebar_list: TemplateChild<ListBox>,
    #[template_child]
    pub reset_sidebar_toggle: TemplateChild<Button>,
    #[template_child]
    pub reset_popover_menu: TemplateChild<PopoverMenu>,
    #[template_child]
    pub reset_close: TemplateChild<Button>,
    #[template_child]
    pub reset_about_button: TemplateChild<Button>,
    #[template_child]
    pub reset_preference_button: TemplateChild<Button>,
    #[template_child]
    pub reset_shortcuts_button: TemplateChild<Button>,
    pub sidebar_entries: RefCell<Vec<(SidebarEntry, Vec<SidebarEntry>)>>,
    pub listeners: Arc<Listeners>,
}

unsafe impl Send for ReSetWindow {}
unsafe impl Sync for ReSetWindow {}

#[glib::object_subclass]
impl ObjectSubclass for ReSetWindow {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetUI";
    type Type = reset_window::ReSetWindow;
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
        obj.setup_callback();
        obj.setup_popover_buttons();
        obj.handle_dynamic_sidebar();
        obj.setup_sidebar_entries();
    }
}

impl WidgetImpl for ReSetWindow {
    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        // felixusan, use "the force" for debugging
        // aka this variable
        // println!("{width}");
        self.parent_size_allocate(width, height, baseline);
        if width > 738 {
            self.reset_main.set_margin_start(100);
            self.reset_main.set_margin_end(100);
        } else if width > 600 {
            self.reset_main.set_margin_start((width - 600) / 2);
            self.reset_main.set_margin_end((width - 600) / 2);
        } else {
            self.reset_main.set_margin_start(0);
            self.reset_main.set_margin_end(0);
        }
    }
}

impl WindowImpl for ReSetWindow {}

impl ApplicationWindowImpl for ReSetWindow {}

impl AdwApplicationWindowImpl for ReSetWindow {}
