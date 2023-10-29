use gtk::{Button, CompositeTemplate, DropDown, TemplateChild, glib};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use crate::components::audio::AudioSourceEntry;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetAudio.ui")]
pub struct AudioBox {
    #[template_child]
    pub resetOutputDevice: TemplateChild<DropDown>,
    #[template_child]
    pub resetAllOutputDevices: TemplateChild<Button>,
}


#[glib::object_subclass]
impl ObjectSubclass for AudioBox {
    const NAME: &'static str = "resetAudio";
    type Type = super::AudioBox;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        AudioSourceEntry::ensure_type();
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl BoxImpl for AudioBox {}

impl ObjectImpl for AudioBox {}

impl ListBoxRowImpl for AudioBox {}

impl WidgetImpl for AudioBox {}

impl WindowImpl for AudioBox {}

impl ApplicationWindowImpl for AudioBox {}
