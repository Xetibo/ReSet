use std::cell::RefCell;
use std::sync::Arc;
use std::time::SystemTime;
use adw::{ActionRow, PreferencesGroup};
use adw::subclass::prelude::PreferencesGroupImpl;

use crate::components::output::sinkEntry;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CheckButton, CompositeTemplate, Label, ProgressBar, Scale};
use ReSet_Lib::audio::audio::Sink;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetSinkEntry.ui")]
pub struct SinkEntry {
    #[template_child]
    pub resetSinkName: TemplateChild<ActionRow>,
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
    pub volumeTimeStamp: RefCell<Option<SystemTime>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SinkEntry {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetSinkEntry";
    type Type = sinkEntry::SinkEntry;
    type ParentType = PreferencesGroup;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl PreferencesGroupImpl for SinkEntry {}

impl ObjectImpl for SinkEntry {}

impl WidgetImpl for SinkEntry {}
