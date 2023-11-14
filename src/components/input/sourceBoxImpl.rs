use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use crate::components::base::listEntry::ListEntry;
use crate::components::input::sourceBox;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, DropDown, TemplateChild};
use gtk::{prelude::*, Button, Label, ProgressBar, Scale};
use ReSet_Lib::audio::audio::{OutputStream, Source};

use super::outputStreamEntry::OutputStreamEntry;
use super::sourceEntry::SourceEntry;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetAudioInput.ui")]
pub struct SourceBox {
    #[template_child]
    pub resetSourceRow: TemplateChild<ListEntry>,
    #[template_child]
    pub resetSourceDropdown: TemplateChild<DropDown>,
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
    pub resetDefaultSource: Arc<RefCell<Source>>,
    pub resetSourceList: Arc<Mutex<Vec<Source>>>,
    pub resetOutputStreamList: Arc<Mutex<Vec<OutputStream>>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SourceBox {
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
