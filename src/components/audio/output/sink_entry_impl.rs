use adw::subclass::prelude::PreferencesGroupImpl;
use adw::{ActionRow, PreferencesGroup};
use re_set_lib::audio::audio_structures::Sink;
use std::cell::RefCell;
use std::sync::Arc;
use std::time::SystemTime;

use crate::components::audio::audio_entry::{AudioIcons, DBusFunction, TAudioEntryImpl};
use crate::components::audio::output::sink_entry;
use gtk::subclass::prelude::*;
use gtk::{Button, CheckButton, CompositeTemplate, Label, Scale};

use super::sink_const::{ICONS, SETDEFAULT, SETMUTE, SETVOLUME};

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

impl TAudioEntryImpl<Sink> for SinkEntry {
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

    fn set_volume_fn(&self) -> &'static DBusFunction {
        &SETVOLUME
    }

    fn set_audio_object_fn(&self) -> &'static DBusFunction {
        &SETDEFAULT
    }

    fn set_mute_fn(&self) -> &'static DBusFunction {
        &SETMUTE
    }

    fn icons(&self) -> &AudioIcons {
        &ICONS
    }
}
