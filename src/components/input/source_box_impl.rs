use adw::{ActionRow, ComboRow, PreferencesGroup};
use re_set_lib::audio::audio_structures::Source;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use crate::components::base::list_entry::ListEntry;
use crate::components::input::source_box;
use gtk::subclass::prelude::*;
use gtk::{glib, CheckButton, CompositeTemplate, StringList, TemplateChild};
use gtk::{prelude::*, Button, Label, ProgressBar, Scale};

use super::output_stream_entry::OutputStreamEntry;
use super::source_entry::SourceEntry;

type SourceEntryMap = Arc<RwLock<HashMap<u32, (Arc<ListEntry>, Arc<SourceEntry>, String)>>>;
type OutputStreamEntryMap = Arc<RwLock<HashMap<u32, (Arc<ListEntry>, Arc<OutputStreamEntry>)>>>;
type SourceMap = Arc<RwLock<HashMap<String, (u32, u32, String)>>>;

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
    pub reset_volume_meter: TemplateChild<ProgressBar>,
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
    pub reset_default_check_button: Arc<CheckButton>,
    pub reset_default_source: Arc<RefCell<Source>>,
    pub reset_source_list: SourceEntryMap,
    pub reset_output_stream_list: OutputStreamEntryMap,
    pub reset_model_list: Arc<RwLock<StringList>>,
    pub reset_model_index: Arc<RwLock<u32>>,
    // first u32 is the index of the source, the second the index in the model list and the third is
    // the full name
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
