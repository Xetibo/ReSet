use adw::subclass::prelude::PreferencesGroupImpl;
use adw::{ComboRow, PreferencesGroup};
use re_set_lib::audio::audio_structures::{InputStream, Sink};
use std::cell::RefCell;
use std::sync::Arc;
use std::time::SystemTime;

use gtk::subclass::prelude::*;
use gtk::{Button, CompositeTemplate, Label, Scale};

use crate::components::audio::generic_entry::{AudioIcons, TAudioStreamImpl};

use super::input_stream_entry;
use super::sink_const::{ICONS, SETSTREAMMUTE, SETSTREAMOBJECT, SETSTREAMVOLUME};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetInputStreamEntry.ui")]
pub struct InputStreamEntry {
    #[template_child]
    pub reset_sink_selection: TemplateChild<ComboRow>,
    #[template_child]
    pub reset_sink_mute: TemplateChild<Button>,
    #[template_child]
    pub reset_volume_slider: TemplateChild<Scale>,
    #[template_child]
    pub reset_volume_percentage: TemplateChild<Label>,
    pub stream: Arc<RefCell<InputStream>>,
    pub associated_sink: Arc<RefCell<(u32, String)>>,
    pub volume_time_stamp: RefCell<Option<SystemTime>>,
}

#[glib::object_subclass]
impl ObjectSubclass for InputStreamEntry {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetInputStreamEntry";
    type Type = input_stream_entry::InputStreamEntry;
    type ParentType = PreferencesGroup;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl PreferencesGroupImpl for InputStreamEntry {}

impl ObjectImpl for InputStreamEntry {}

impl WidgetImpl for InputStreamEntry {}

impl TAudioStreamImpl<Sink, InputStream> for InputStreamEntry {
    fn audio_object_selection(&self) -> &TemplateChild<ComboRow> {
        &self.reset_sink_selection
    }

    fn audio_object_mute(&self) -> &TemplateChild<Button> {
        &self.reset_sink_mute
    }

    fn volume_slider(&self) -> &TemplateChild<Scale> {
        &self.reset_volume_slider
    }

    fn volume_percentage(&self) -> &TemplateChild<Label> {
        &self.reset_volume_percentage
    }

    fn stream_object(&self) -> Arc<RefCell<InputStream>> {
        self.stream.clone()
    }

    fn associated_audio_object(&self) -> Arc<RefCell<(u32, String)>> {
        self.associated_sink.clone()
    }

    fn volume_time_stamp(&self) -> &RefCell<Option<SystemTime>> {
        &self.volume_time_stamp
    }

    fn set_volume_fn(&self) -> &'static crate::components::audio::generic_entry::DBusFunction {
        &SETSTREAMVOLUME
    }

    fn set_audio_object_fn(
        &self,
    ) -> &'static crate::components::audio::generic_entry::DBusFunction {
        &SETSTREAMOBJECT
    }

    fn set_mute_fn(&self) -> &'static crate::components::audio::generic_entry::DBusFunction {
        &SETSTREAMMUTE
    }

    fn icons(&self) -> &AudioIcons {
        &ICONS
    }
}
