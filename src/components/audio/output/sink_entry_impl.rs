use adw::subclass::prelude::PreferencesGroupImpl;
use adw::{ActionRow, PreferencesGroup};
use re_set_lib::audio::audio_structures::Sink;
use std::cell::RefCell;
use std::sync::Arc;
use std::time::SystemTime;

use crate::components::audio::generic_entry::AudioImpl;
use crate::components::audio::output::sink_entry;
use gtk::subclass::prelude::*;
use gtk::{Button, CheckButton, CompositeTemplate, Label, Scale};

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
    pub sink: Arc<RefCell<Sink>>,
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

impl AudioImpl<Sink> for SinkEntry {
    fn name(&self) -> &TemplateChild<ActionRow> {
        &self.reset_sink_name
    }

    fn selected_audio_object(&self) -> &TemplateChild<CheckButton> {
        &self.reset_selected_sink
    }

    fn mute(&self) -> &TemplateChild<Button> {
        &self.reset_sink_mute
    }

    fn volume_slider(&self) -> &TemplateChild<Scale> {
        &self.reset_volume_slider
    }

    fn volume_percentage(&self) -> &TemplateChild<Label> {
        &self.reset_volume_percentage
    }

    fn audio_object(&self) -> Arc<RefCell<Sink>> {
        self.sink.clone()
    }

    fn volume_time_stamp(&self) -> &RefCell<Option<SystemTime>> {
        &self.volume_time_stamp
    }

    fn set_volume_fn(&self) -> (&'static str, &'static str) {
        ("SetSinkVolume", "Failed to set set sink volume")
    }

    fn set_audio_object_fn(&self) -> (&'static str, &'static str) {
        ("SetDefaultSink", "Faield to set default sink")
    }

    fn set_mute_fn(&self) -> (&'static str, &'static str) {
        ("SetSinkMute", "Failed to mute sink")
    }
}
