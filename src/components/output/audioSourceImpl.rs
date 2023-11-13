use gtk::{Button, CompositeTemplate, glib, Label, ProgressBar, Scale};
use gtk::subclass::prelude::*;
use crate::components::output::audioSource;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetOutputStreamEntry.ui")]
pub struct AudioSourceEntry {
    #[template_child]
    pub resetSourceName: TemplateChild<Label>,
    #[template_child]
    pub resetSourceMute: TemplateChild<Button>,
    #[template_child]
    pub resetVolumeSlider: TemplateChild<Scale>,
    #[template_child]
    pub resetVolumePercentage: TemplateChild<Label>,
    #[template_child]
    pub resetVolumeMeter: TemplateChild<ProgressBar>,
}

#[glib::object_subclass]
impl ObjectSubclass for AudioSourceEntry {
    const NAME: &'static str = "resetOutputStreamEntry";
    type Type = audioSource::AudioSourceEntry;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl BoxImpl for AudioSourceEntry {}

impl ObjectImpl for AudioSourceEntry {}

impl WidgetImpl for AudioSourceEntry {}
