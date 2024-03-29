use adw::subclass::prelude::PreferencesGroupImpl;
use adw::{ComboRow, PreferencesGroup};
use re_set_lib::audio::audio_structures::OutputStream;
use std::cell::RefCell;
use std::sync::Arc;
use std::time::SystemTime;

use crate::components::audio::input::output_stream_entry;
use gtk::subclass::prelude::*;
use gtk::{Button, CompositeTemplate, Label, Scale};

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
