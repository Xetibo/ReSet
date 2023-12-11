use adw::subclass::prelude::PreferencesGroupImpl;
use adw::{ComboRow, PreferencesGroup};
use re_set_lib::audio::audio_structures::InputStream;
use std::cell::RefCell;
use std::sync::Arc;
use std::time::SystemTime;

use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Label, ProgressBar, Scale};

use super::input_stream_entry;

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
    #[template_child]
    pub reset_volume_meter: TemplateChild<ProgressBar>,
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
