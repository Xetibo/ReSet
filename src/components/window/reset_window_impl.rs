use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use adw::subclass::prelude::AdwApplicationWindowImpl;
use adw::{Breakpoint, OverlaySplitView};
use glib::prelude::StaticTypeExt;
use glib::subclass::InitializingObject;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::*;
use gtk::{Button, CompositeTemplate, FlowBox, ListBox, SearchEntry};

use crate::components::base::error::ReSetError;
use crate::components::base::utils::{Listeners, Position};
use crate::components::utils::Capabilities;
use crate::components::wifi::wifi_box::WifiBox;
use crate::components::window::reset_window;
use crate::components::window::sidebar_entry::SidebarEntry;

type SidebarEntries = RefCell<Vec<(Rc<SidebarEntry>, Vec<Rc<SidebarEntry>>)>>;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/Xetibo/ReSet/resetMainWindow.ui")]
pub struct ReSetWindow {
    #[template_child]
    pub reset_main: TemplateChild<FlowBox>,
    #[template_child]
    pub reset_banner: TemplateChild<adw::Banner>,
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
    pub reset_close: TemplateChild<Button>,
    // #[template_child]
    // pub reset_banner: TemplateChild<adw::Banner>,
    pub sidebar_entries: SidebarEntries,
    pub default_entry: RefCell<Option<Rc<SidebarEntry>>>,
    pub listeners: Arc<Listeners>,
    pub position: Rc<RefCell<Position>>,
    pub error_popup: ReSetError,
    pub capabilities: Capabilities, 
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
        obj.setup_shortcuts();
        obj.handle_dynamic_sidebar();
        obj.setup_sidebar_entries();
    }
}

impl WidgetImpl for ReSetWindow {
    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        self.parent_size_allocate(width, height, baseline);
        if width > 658 {
            self.reset_main.set_margin_start(60);
            self.reset_main.set_margin_end(60);
                self.reset_banner.set_margin_start(60);
                self.reset_banner.set_margin_end(60);
        } else {
            let div = (width - 540) / 2;
            if div > 1 {
                self.reset_main.set_margin_start(div);
                self.reset_main.set_margin_end(div);
                self.reset_banner.set_margin_start(div);
                self.reset_banner.set_margin_end(div);
            } else {
                self.reset_main.set_margin_start(0);
                self.reset_main.set_margin_end(0);
                self.reset_banner.set_margin_start(0);
                self.reset_banner.set_margin_end(0);
            }
        }
    }
}

impl WindowImpl for ReSetWindow {}

impl ApplicationWindowImpl for ReSetWindow {}

impl AdwApplicationWindowImpl for ReSetWindow {}
