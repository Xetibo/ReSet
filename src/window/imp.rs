use adw::glib::{StaticTypeExt};
use adw::NavigationSplitView;
use adw::subclass::prelude::AdwApplicationWindowImpl;
use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, SearchEntry, ListBox, FlowBox, Button};
use crate::wifi::WifiBox;

#[allow(non_snake_case)]
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/xetibo/reset/resetMainWindow.ui")]
pub struct Window {
    #[template_child]
    pub resetNavigationSplitView: TemplateChild<NavigationSplitView>,
    #[template_child]
    pub resetSearchEntry: TemplateChild<SearchEntry>,
    #[template_child]
    pub resetSidebarList: TemplateChild<ListBox>,
    #[template_child]
    pub resetMain: TemplateChild<FlowBox>,
    #[template_child]
    pub resetSideBarToggle: TemplateChild<Button>
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

        let wifibox = WifiBox::new();
        let wifibox2 = WifiBox::new();
        let wifibox3 = WifiBox::new();
        let wifibox4 = WifiBox::new();

        self.resetMain.insert(&wifibox, -1);
        self.resetMain.insert(&wifibox2, -1);
        self.resetMain.insert(&wifibox3, -1);
        self.resetMain.insert(&wifibox4, -1);
    }
}

impl WidgetImpl for Window {
    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        println!("new width {}, new hight {}", width, height);
        self.parent_size_allocate(width, height, baseline);
        if width < 550 {
            self.obj().hideSidebar(true);
        } else {
            self.obj().hideSidebar(false);
        }
    }
}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}

impl AdwApplicationWindowImpl for Window {}


