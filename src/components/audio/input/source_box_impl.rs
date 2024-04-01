use adw::{ActionRow, ComboRow, PreferencesGroup};
use re_set_lib::audio::audio_structures::Source;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use crate::components::audio::generic_entry::AudioBoxImpl;
use crate::components::audio::input::source_box;
use crate::components::base::error::ReSetError;
use crate::components::base::list_entry::ListEntry;
use gtk::subclass::prelude::*;
use gtk::{prelude::*, Button, Label, Scale};
use gtk::{CheckButton, CompositeTemplate, StringList};

use super::output_stream_entry::OutputStreamEntry;
use super::source_entry::SourceEntry;

type SourceEntryMap = Arc<RwLock<HashMap<u32, (Arc<ListEntry>, Arc<SourceEntry>, String)>>>;
type OutputStreamEntryMap = Arc<RwLock<HashMap<u32, (Arc<ListEntry>, Arc<OutputStreamEntry>)>>>;
// the key is the alias, the first value u32 is the index of the source, the second is the technical name
type SourceMap = Arc<RwLock<HashMap<String, (u32, String)>>>;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetAudioInput.ui")]
pub struct SourceBox {
    #[template_child]
    pub reset_source_row: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_cards_row: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_source_dropdown: TemplateChild<ComboRow>,
    #[template_child]
    pub reset_source_mute: TemplateChild<Button>,
    #[template_child]
    pub reset_volume_slider: TemplateChild<Scale>,
    #[template_child]
    pub reset_volume_percentage: TemplateChild<Label>,
    #[template_child]
    pub reset_sources: TemplateChild<gtk::Box>,
    #[template_child]
    pub reset_output_stream_button: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_output_streams: TemplateChild<gtk::Box>,
    #[template_child]
    pub reset_input_cards_back_button: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_cards: TemplateChild<PreferencesGroup>,
    #[template_child]
    pub error: TemplateChild<ReSetError>,
    pub reset_default_check_button: Arc<CheckButton>,
    pub reset_default_source: Arc<RefCell<Source>>,
    pub reset_source_list: SourceEntryMap,
    pub reset_output_stream_list: OutputStreamEntryMap,
    pub reset_model_list: Arc<RwLock<StringList>>,
    pub reset_model_index: Arc<RwLock<u32>>,
    pub reset_source_map: SourceMap,
    pub volume_time_stamp: RefCell<Option<SystemTime>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SourceBox {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetAudioInput";
    type Type = source_box::SourceBox;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        OutputStreamEntry::ensure_type();
        SourceEntry::ensure_type();
        ListEntry::ensure_type();
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl BoxImpl for SourceBox {}

impl ObjectImpl for SourceBox {
    fn constructed(&self) {
        let obj = self.obj();
        obj.setup_callbacks();
    }
}

impl ListBoxRowImpl for SourceBox {}

impl WidgetImpl for SourceBox {}

impl WindowImpl for SourceBox {}

impl ApplicationWindowImpl for SourceBox {}

impl AudioBoxImpl<Source, SourceEntry, super::source_entry_impl::SourceEntry> for SourceBox {
    fn audio_object_row(&self) -> &TemplateChild<ActionRow> {
        &self.reset_source_row
    }

    fn cards_row(&self) -> &TemplateChild<ActionRow> {
        &self.reset_cards_row
    }

    fn audio_object_dropdown(&self) -> &TemplateChild<ComboRow> {
        &self.reset_source_dropdown
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

    fn audio_objects(&self) -> &TemplateChild<gtk::Box> {
        &self.reset_sources
    }

    fn audio_object_stream_button(&self) -> &TemplateChild<ActionRow> {
        &self.reset_output_stream_button
    }

    fn audio_object_streams(&self) -> &TemplateChild<gtk::Box> {
        &self.reset_output_streams
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

    fn default_audio_object(&self) -> Arc<RefCell<Source>> {
        self.reset_default_source.clone()
    }

    fn audio_object_list(
        &self,
    ) -> &crate::components::audio::generic_entry::AudioEntryMap<SourceEntry> {
        &self.reset_source_list
    }

    // fn audio_object_stream_list(
    //     &self,
    // ) -> &crate::components::audio::generic_entry::AudioStreamEntryMap<SourceEntry> {
    //     &
    // }

    fn model_list(&self) -> Arc<RwLock<StringList>> {
        self.reset_model_list.clone()
    }

    fn model_index(&self) -> Arc<RwLock<u32>> {
        self.reset_model_index.clone()
    }

    fn source_map(&self) -> &crate::components::audio::generic_entry::AudioMap {
        &self.reset_source_map
    }

    fn volume_time_stamp(&self) -> &RefCell<Option<SystemTime>> {
        &self.volume_time_stamp
    }
}
