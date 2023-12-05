use adw::subclass::prelude::PreferencesGroupImpl;
use adw::{ActionRow, PreferencesGroup};
use std::cell::RefCell;
use std::sync::Arc;
use std::time::SystemTime;

use crate::components::output::sink_entry;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CheckButton, CompositeTemplate, Label, ProgressBar, Scale};
use ReSet_Lib::audio::audio::Sink;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetSinkEntry.ui")]
pub struct SinkEntry {
    #[template_child]
    pub reset_sink_name: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_selected_sink: TemplateChild<CheckButton>,
    #[template_child]
    pub reset_sink_mute: TemplateChild<Button>,
    #[template_child]
    pub reset_volume_slider: TemplateChild<Scale>,
    #[template_child]
    pub reset_volume_percentage: TemplateChild<Label>,
    #[template_child]
    pub reset_volume_meter: TemplateChild<ProgressBar>,
    pub stream: Arc<RefCell<Sink>>,
    pub volume_time_stamp: RefCell<Option<SystemTime>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SinkEntry {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetSinkEntry";
    type Type = sink_entry::SinkEntry;
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
