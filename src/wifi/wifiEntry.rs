use gtk::{Button, CompositeTemplate, glib};
use gtk::subclass::prelude::*;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/xetibo/reset/resetWifiEntry.ui")]
pub struct WifiEntry {
    #[template_child]
    pub resetWifiButton: TemplateChild<Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for WifiEntry {
    const NAME: &'static str = "resetWifiEntry";
    type Type = super::WifiEntry;
    type ParentType = gtk::ListBoxRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }
    
    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for WifiEntry {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl ListBoxRowImpl for WifiEntry {}

impl WidgetImpl for WifiEntry {}

impl WindowImpl for WifiEntry {}

impl ApplicationWindowImpl for WifiEntry {}
