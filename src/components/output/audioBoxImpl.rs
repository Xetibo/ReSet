use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use crate::components::base::listEntry::ListEntry;
use crate::components::output::audioBox;
use crate::components::output::inputStreamEntry::InputStreamEntry;
use gtk::subclass::prelude::*;
use gtk::{glib, Box, Button, CompositeTemplate, DropDown, Label, TemplateChild};
use gtk::{prelude::*, ProgressBar, Scale};
use ReSet_Lib::audio::audio::{InputStream, Sink};

use super::sinkEntry::SinkEntry;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetAudioOutput.ui")]
pub struct AudioBox {
    #[template_child]
    pub resetSinksRow: TemplateChild<ListEntry>,
    #[template_child]
    pub resetInputDevice: TemplateChild<DropDown>,
    #[template_child]
    pub resetSinkMute: TemplateChild<Button>,
    #[template_child]
    pub resetVolumeSlider: TemplateChild<Scale>,
    #[template_child]
    pub resetVolumePercentage: TemplateChild<Label>,
    #[template_child]
    pub resetVolumeMeter: TemplateChild<ProgressBar>,
    #[template_child]
    pub resetSinks: TemplateChild<Box>,
    #[template_child]
    pub resetInputStreamButton: TemplateChild<ListEntry>,
    #[template_child]
    pub resetInputStreams: TemplateChild<Box>,
    pub resetDefaultSink: RefCell<Option<Sink>>,
    pub resetSinkList: Arc<Mutex<Vec<Sink>>>,
    pub resetInputStreamList: Arc<Mutex<Vec<InputStream>>>,
}

#[glib::object_subclass]
impl ObjectSubclass for AudioBox {
    const NAME: &'static str = "resetAudioOutput";
    type Type = audioBox::AudioBox;
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

impl BoxImpl for AudioBox {}

impl ObjectImpl for AudioBox {
    fn constructed(&self) {
        let obj = self.obj();
        obj.setupCallbacks();
    }
}

impl ListBoxRowImpl for AudioBox {}

impl WidgetImpl for AudioBox {}

impl WindowImpl for AudioBox {}

impl ApplicationWindowImpl for AudioBox {}
