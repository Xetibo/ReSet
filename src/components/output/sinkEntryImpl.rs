use std::cell::RefCell;
use std::sync::Arc;

use crate::components::output::sinkEntry;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, DropDown, Label, ProgressBar, Scale, CheckButton};
use ReSet_Lib::audio::audio::Sink;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetSinkEntry.ui")]
pub struct SinkEntry {
    #[template_child]
    pub resetSinkName: TemplateChild<Label>,
    #[template_child]
    pub resetSelectedSink: TemplateChild<CheckButton>,
    #[template_child]
    pub resetSinkMute: TemplateChild<Button>,
    #[template_child]
    pub resetVolumeSlider: TemplateChild<Scale>,
    #[template_child]
    pub resetVolumePercentage: TemplateChild<Label>,
    #[template_child]
    pub resetVolumeMeter: TemplateChild<ProgressBar>,
    pub stream: Arc<RefCell<Sink>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SinkEntry {
    const NAME: &'static str = "resetSinkEntry";
    type Type = sinkEntry::SinkEntry;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl BoxImpl for SinkEntry {}

impl ObjectImpl for SinkEntry {}

impl WidgetImpl for SinkEntry {}
