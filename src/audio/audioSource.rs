use gtk::{Button, CompositeTemplate, glib, Image, Label, ProgressBar, Scale};
use gtk::prelude::*;
use gtk::subclass::prelude::*;


#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/xetibo/reset/resetAudioSourceEntry.ui")]
pub struct AudioSourceEntry {
    #[template_child]
    pub resetSourceIcon: TemplateChild<Image>,
    #[template_child]
    pub resetSourceName: TemplateChild<Label>,
    #[template_child]
    pub resetSourceMute: TemplateChild<Button>,
    #[template_child]
    pub resetVolumeSlider: TemplateChild<Scale>,
    #[template_child]
    pub resetVolumeMeter: TemplateChild<ProgressBar>,
}

#[glib::object_subclass]
impl ObjectSubclass for AudioSourceEntry {
    const NAME: &'static str = "resetWifiEntry";
    type Type = super::AudioSourceEntry;
    // type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }
    
    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for AudioSourceEntry {}

impl ListBoxRowImpl for AudioSourceEntry {}

impl WidgetImpl for AudioSourceEntry {}

impl WindowImpl for AudioSourceEntry {}

impl ApplicationWindowImpl for AudioSourceEntry {}
