use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use gtk::FlowBox;
use re_set_lib::utils::plugin::SidebarInfo;

use crate::components::base::utils::{Listeners, Position};

extern "C" {
    pub fn startup() -> SidebarInfo;
    pub fn shutdown();
    pub fn run_test();
}

type RegularClickEvent = fn(Arc<Listeners>, FlowBox, Rc<RefCell<Position>>);
type PluginClickEvent = Rc<dyn Fn(FlowBox, Rc<RefCell<Position>>, Vec<gtk::Box>)>;

pub trait TSideBarInfo {
    fn name(&self) -> &'static str;
    fn icon_name(&self) -> &'static str;
    fn parent(&self) -> Option<&'static str>;
    fn regular_click_event(&self) -> Option<RegularClickEvent>;
    fn plugin_click_event(&self) -> PluginClickEvent;
    fn plugin_boxes(&self) -> Option<Vec<gtk::Box>>;
}

pub struct ReSetSidebarInfo {
    pub name: &'static str,
    pub icon_name: &'static str,
    pub parent: Option<&'static str>,
    pub click_event: RegularClickEvent,
}

impl TSideBarInfo for ReSetSidebarInfo {
    fn name(&self) -> &'static str {
        self.name
    }

    fn icon_name(&self) -> &'static str {
        self.icon_name
    }

    fn parent(&self) -> Option<&'static str> {
        self.parent
    }

    fn regular_click_event(&self) -> Option<RegularClickEvent> {
        Some(self.click_event)
    }

    fn plugin_click_event(&self) -> PluginClickEvent {
       Rc::new(|_,_,_| {}) 
    }

    fn plugin_boxes(&self) -> Option<Vec<gtk::Box>> {
        None
    }
}

#[repr(C)]
pub struct PluginSidebarInfo {
    pub name: &'static str,
    pub icon_name: &'static str,
    pub parent: Option<&'static str>,
    pub click_event: PluginClickEvent,
    pub plugin_boxes: Vec<gtk::Box>,
}


impl TSideBarInfo for PluginSidebarInfo {
    fn name(&self) -> &'static str {
        self.name
    }

    fn icon_name(&self) -> &'static str {
        self.icon_name
    }

    fn parent(&self) -> Option<&'static str> {
        self.parent
    }

    fn regular_click_event(&self) -> Option<RegularClickEvent> {
        None
    }

    fn plugin_click_event(&self) -> PluginClickEvent {
        self.click_event.clone()
    }

    fn plugin_boxes(&self) -> Option<Vec<gtk::Box>> {
        Some(self.plugin_boxes.clone())
    }
}

