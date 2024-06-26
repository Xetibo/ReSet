use adw::{ActionRow, ComboRow, PreferencesGroup};
use re_set_lib::audio::audio_structures::Sink;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use crate::components::audio::audio_entry::{AudioIcons, TAudioBoxImpl};
use crate::components::audio::output::input_stream_entry::InputStreamEntry;
use crate::components::base::error::ReSetError;
use crate::components::base::list_entry::ListEntry;
use gtk::subclass::prelude::*;
use gtk::{prelude::*, Scale};
use gtk::{Box, Button, CheckButton, CompositeTemplate, Label, StringList};

use super::sink_box;
use super::sink_const::ICONS;
use super::sink_entry::SinkEntry;

type SinkEntryMap = Arc<RwLock<HashMap<u32, (Arc<ListEntry>, Arc<SinkEntry>, String)>>>;
type InputStreamEntryMap = Arc<RwLock<HashMap<u32, (Arc<ListEntry>, Arc<InputStreamEntry>)>>>;
// key is model name -> alias, first u32 is the index of the sink, the second the index in the model list and the third is
// the detailed name
type SinkMap = Arc<RwLock<HashMap<String, (u32, String)>>>;

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
    #[template_child]
    pub error: TemplateChild<ReSetError>,
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

impl ObjectImpl for SinkBox {}

impl ListBoxRowImpl for SinkBox {}

impl WidgetImpl for SinkBox {}

impl WindowImpl for SinkBox {}

impl ApplicationWindowImpl for SinkBox {}

impl TAudioBoxImpl<Sink, SinkEntry, InputStreamEntry> for SinkBox {
    fn audio_object_row(&self) -> &TemplateChild<ActionRow> {
        &self.reset_sinks_row
    }

    fn cards_row(&self) -> &TemplateChild<ActionRow> {
        &self.reset_cards_row
    }

    fn audio_object_dropdown(&self) -> &TemplateChild<ComboRow> {
        &self.reset_sink_dropdown
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

    fn audio_objects(&self) -> &TemplateChild<gtk::Box> {
        &self.reset_sinks
    }

    fn audio_object_stream_button(&self) -> &TemplateChild<ActionRow> {
        &self.reset_input_stream_button
    }

    fn audio_object_streams(&self) -> &TemplateChild<gtk::Box> {
        &self.reset_input_streams
    }

    fn cards_button(&self) -> &TemplateChild<ActionRow> {
        &self.reset_input_cards_back_button
    }

    fn cards(&self) -> &TemplateChild<PreferencesGroup> {
        &self.reset_cards
    }

    fn error(&self) -> &TemplateChild<ReSetError> {
        &self.error
    }

    fn default_check_button(&self) -> Arc<CheckButton> {
        self.reset_default_check_button.clone()
    }

    fn default_audio_object(&self) -> Arc<RefCell<Sink>> {
        self.reset_default_sink.clone()
    }

    fn audio_object_list(
        &self,
    ) -> &crate::components::audio::audio_entry::AudioEntryMap<SinkEntry> {
        &self.reset_sink_list
    }

    fn audio_object_stream_list(
        &self,
    ) -> &crate::components::audio::audio_entry::AudioStreamEntryMap<InputStreamEntry> {
        &self.reset_input_stream_list
    }

    fn model_list(&self) -> Arc<RwLock<StringList>> {
        self.reset_model_list.clone()
    }

    fn model_index(&self) -> Arc<RwLock<u32>> {
        self.reset_model_index.clone()
    }

    fn source_map(&self) -> &crate::components::audio::audio_entry::AudioMap {
        &self.reset_sink_map
    }

    fn volume_time_stamp(&self) -> &RefCell<Option<SystemTime>> {
        &self.volume_time_stamp
    }

    fn icons(&self) -> &AudioIcons {
        &ICONS
    }
}
