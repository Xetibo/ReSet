use adw::glib;
use adw::glib::Object;
use adw::subclass::prelude::ObjectSubclassIsExt;
use glib::PropertySet;
use ReSet_Lib::network::connection::Connection;
use crate::components::wifi::{wifiOptionsImpl};

glib::wrapper! {
    pub struct WifiOptions(ObjectSubclass<wifiOptionsImpl::WifiOptions>)
        @extends adw::NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl WifiOptions {
    pub fn new(option: Option<Connection>) -> Self {
        let wifiOption: WifiOptions = Object::builder().build();
        wifiOption.imp().options.set(option);
        wifiOption
    }

}
