use std::cell::RefCell;
use adw::NavigationPage;
use adw::subclass::prelude::NavigationPageImpl;
use crate::components::wifi::{wifiOptions};
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use ReSet_Lib::network::connection::Connection;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetWifiOptions.ui")]
pub struct WifiOptions {
    pub options: RefCell<Option<Connection>> // Option<Rc<RefCell<Connection>>>
}

#[glib::object_subclass]
impl ObjectSubclass for WifiOptions {
    const NAME: &'static str = "resetWifiOptions";
    type Type = wifiOptions::WifiOptions;
    type ParentType = NavigationPage;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl NavigationPageImpl for WifiOptions {}

impl ObjectImpl for WifiOptions {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl BoxImpl for WifiOptions {}

impl WidgetImpl for WifiOptions {}

impl WindowImpl for WifiOptions {}

impl ApplicationWindowImpl for WifiOptions {}
