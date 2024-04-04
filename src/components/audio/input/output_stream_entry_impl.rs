use adw::subclass::prelude::PreferencesGroupImpl;
use adw::{ComboRow, PreferencesGroup};
use re_set_lib::audio::audio_structures::{OutputStream, Source};
use std::cell::RefCell;
use std::sync::Arc;
use std::time::SystemTime;

use crate::components::audio::audio_entry::{AudioIcons, TAudioStreamImpl};
use crate::components::audio::input::output_stream_entry;
use gtk::subclass::prelude::*;
use gtk::{Button, CompositeTemplate, Label, Scale};

use super::source_const::{ICONS, SETSTREAMMUTE, SETSTREAMOBJECT, SETSTREAMVOLUME};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetOutputStreamEntry.ui")]
pub struct OutputStreamEntry {
    #[template_child]
    pub reset_source_selection: TemplateChild<ComboRow>,
    #[template_child]
    pub reset_source_mute: TemplateChild<Button>,
    #[template_child]
    pub reset_volume_slider: TemplateChild<Scale>,
    #[template_child]
    pub reset_volume_percentage: TemplateChild<Label>,
    pub stream: Arc<RefCell<OutputStream>>,
    pub associated_source: Arc<RefCell<(u32, String)>>,
    pub volume_time_stamp: RefCell<Option<SystemTime>>,
}

#[glib::object_subclass]
impl ObjectSubclass for OutputStreamEntry {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetOutputStreamEntry";
    type Type = output_stream_entry::OutputStreamEntry;
    type ParentType = PreferencesGroup;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl PreferencesGroupImpl for OutputStreamEntry {}

impl ObjectImpl for OutputStreamEntry {}

impl WidgetImpl for OutputStreamEntry {}

impl TAudioStreamImpl<Source, OutputStream> for OutputStreamEntry {
    fn audio_object_selection(&self) -> &TemplateChild<ComboRow> {
        &self.reset_source_selection
    }

    fn audio_object_mute(&self) -> &TemplateChild<Button> {
        &self.reset_source_mute
    }

    fn volume_slider(&self) -> &TemplateChild<Scale> {
        &self.reset_volume_slider
    }

    fn volume_percentage(&self) -> &TemplateChild<Label> {
        &self.reset_volume_percentage
    }

    fn stream_object(&self) -> Arc<RefCell<OutputStream>> {
        self.stream.clone()
    }

    fn associated_audio_object(&self) -> Arc<RefCell<(u32, String)>> {
        self.associated_source.clone()
    }

    fn volume_time_stamp(&self) -> &RefCell<Option<SystemTime>> {
        &self.volume_time_stamp
    }

    fn set_volume_fn(&self) -> &'static crate::components::audio::audio_entry::DBusFunction {
        &SETSTREAMVOLUME
    }

    fn set_audio_object_fn(&self) -> &'static crate::components::audio::audio_entry::DBusFunction {
        &SETSTREAMOBJECT
    }

    fn set_mute_fn(&self) -> &'static crate::components::audio::audio_entry::DBusFunction {
        &SETSTREAMMUTE
    }

    fn icons(&self) -> &AudioIcons {
        &ICONS
    }
}
