use adw::{ActionRow, ComboRow, PreferencesGroup};
use re_set_lib::audio::audio_structures::Sink;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use crate::components::base::list_entry::ListEntry;
use crate::components::output::input_stream_entry::InputStreamEntry;
use gtk::subclass::prelude::*;
use gtk::{glib, Box, Button, CheckButton, CompositeTemplate, Label, StringList, TemplateChild};
use gtk::{prelude::*, ProgressBar, Scale};

use super::sink_box;
use super::sink_entry::SinkEntry;

type SinkEntryMap = Arc<RwLock<HashMap<u32, (Arc<ListEntry>, Arc<SinkEntry>, String)>>>;
type InputStreamEntryMap = Arc<RwLock<HashMap<u32, (Arc<ListEntry>, Arc<InputStreamEntry>)>>>;
// key is model name -> alias, first u32 is the index of the sink, the second the index in the model list and the third is
// the detailed name
type SinkMap = Arc<RwLock<HashMap<String, (u32, u32, String)>>>;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetAudioOutput.ui")]
pub struct SinkBox {
    #[template_child]
    pub reset_sinks_row: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_cards_row: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_sink_dropdown: TemplateChild<ComboRow>,
    #[template_child]
    pub reset_sink_mute: TemplateChild<Button>,
    #[template_child]
    pub reset_volume_slider: TemplateChild<Scale>,
    #[template_child]
    pub reset_volume_percentage: TemplateChild<Label>,
    #[template_child]
    pub reset_sinks: TemplateChild<Box>,
    #[template_child]
    pub reset_input_stream_button: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_input_streams: TemplateChild<Box>,
    #[template_child]
    pub reset_input_cards_back_button: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_cards: TemplateChild<PreferencesGroup>,
    pub reset_default_check_button: Arc<CheckButton>,
    pub reset_default_sink: Arc<RefCell<Sink>>,
    pub reset_sink_list: SinkEntryMap,
    pub reset_input_stream_list: InputStreamEntryMap,
    pub reset_model_list: Arc<RwLock<StringList>>,
    pub reset_model_index: Arc<RwLock<u32>>,
    pub reset_sink_map: SinkMap,
    pub volume_time_stamp: RefCell<Option<SystemTime>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SinkBox {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetAudioOutput";
    type Type = sink_box::SinkBox;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        InputStreamEntry::ensure_type();
        SinkEntry::ensure_type();
        ListEntry::ensure_type();
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl BoxImpl for SinkBox {}

impl ObjectImpl for SinkBox {
    fn constructed(&self) {
        let obj = self.obj();
        obj.setup_callbacks();
    }
}

impl ListBoxRowImpl for SinkBox {}

impl WidgetImpl for SinkBox {}

impl WindowImpl for SinkBox {}

impl ApplicationWindowImpl for SinkBox {}
