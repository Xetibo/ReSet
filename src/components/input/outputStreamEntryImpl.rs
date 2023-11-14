use crate::components::input::outputStreamEntry;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Label, ProgressBar, Scale, DropDown};

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetOutputStreamEntry.ui")]
pub struct OutputStreamEntry {
    #[template_child]
    pub resetSourceName: TemplateChild<Label>,
    #[template_child]
    pub resetSelectedSource: TemplateChild<DropDown>,
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
impl ObjectSubclass for OutputStreamEntry {
    const NAME: &'static str = "resetOutputStreamEntry";
    type Type = outputStreamEntry::OutputStreamEntry;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl BoxImpl for OutputStreamEntry {}

impl ObjectImpl for OutputStreamEntry {}

impl WidgetImpl for OutputStreamEntry {}

