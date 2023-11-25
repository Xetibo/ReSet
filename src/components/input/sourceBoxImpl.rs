use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use adw::{ActionRow, ComboRow, PreferencesGroup};

use crate::components::base::listEntry::ListEntry;
use crate::components::input::sourceBox;
use gtk::subclass::prelude::*;
use gtk::{glib, CheckButton, CompositeTemplate, StringList, TemplateChild};
use gtk::{prelude::*, Button, Label, ProgressBar, Scale};
use ReSet_Lib::audio::audio::Source;

use super::outputStreamEntry::OutputStreamEntry;
use super::sourceEntry::SourceEntry;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetAudioInput.ui")]
pub struct SourceBox {
    #[template_child]
    pub resetSourceRow: TemplateChild<ActionRow>,
    #[template_child]
    pub resetCardsRow: TemplateChild<ActionRow>,
    #[template_child]
    pub resetSourceDropdown: TemplateChild<ComboRow>,
    #[template_child]
    pub resetSourceMute: TemplateChild<Button>,
    #[template_child]
    pub resetVolumeSlider: TemplateChild<Scale>,
    #[template_child]
    pub resetVolumePercentage: TemplateChild<Label>,
    #[template_child]
    pub resetVolumeMeter: TemplateChild<ProgressBar>,
    #[template_child]
    pub resetSources: TemplateChild<gtk::Box>,
    #[template_child]
    pub resetOutputStreamButton: TemplateChild<ListEntry>,
    #[template_child]
    pub resetOutputStreams: TemplateChild<gtk::Box>,
    #[template_child]
    pub resetInputCardsBackButton: TemplateChild<ListEntry>,
    #[template_child]
    pub resetCards: TemplateChild<PreferencesGroup>,
    pub resetDefaultCheckButton: Arc<CheckButton>,
    pub resetDefaultSource: Arc<RefCell<Source>>,
    pub resetSourceList: Arc<RwLock<HashMap<u32, (Arc<ListEntry>, Arc<SourceEntry>, String)>>>,
    pub resetOutputStreamList: Arc<RwLock<HashMap<u32, (Arc<ListEntry>, Arc<OutputStreamEntry>)>>>,
    pub resetModelList: Arc<RwLock<StringList>>,
    pub resetModelIndex: Arc<RwLock<u32>>,
    // first u32 is the index of the source, the second the index in the model list and the third is
    // the full name
    pub resetSourceMap: Arc<RwLock<HashMap<String, (u32, u32, String)>>>,
    pub volumeTimeStamp: RefCell<Option<SystemTime>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SourceBox {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetAudioInput";
    type Type = sourceBox::SourceBox;
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
        obj.setupCallbacks();
    }
}

impl ListBoxRowImpl for SourceBox {}

impl WidgetImpl for SourceBox {}

impl WindowImpl for SourceBox {}

impl ApplicationWindowImpl for SourceBox {}
