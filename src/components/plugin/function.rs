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

pub struct ReSetSidebarInfo {
    pub name: &'static str,
    pub icon_name: &'static str,
    pub parent: Option<&'static str>,
    pub click_event: fn(Arc<Listeners>, FlowBox, Rc<RefCell<Position>>),
}

#[repr(C)]
pub struct PluginSidebarInfo {
    pub name: &'static str,
    pub icon_name: &'static str,
    pub parent: Option<&'static str>,
    pub click_event: Rc<dyn Fn(FlowBox, Rc<RefCell<Position>>, Vec<gtk::Box>)>,
    pub plugin_boxes: Vec<gtk::Box>,
}