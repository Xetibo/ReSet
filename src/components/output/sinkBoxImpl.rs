use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use crate::components::base::listEntry::ListEntry;
use crate::components::output::inputStreamEntry::InputStreamEntry;
use gtk::subclass::prelude::*;
use gtk::{glib, Box, Button, CompositeTemplate, DropDown, Label, TemplateChild};
use gtk::{prelude::*, ProgressBar, Scale};
use ReSet_Lib::audio::audio::{InputStream, Sink};

use super::sinkBox;
use super::sinkEntry::SinkEntry;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetAudioOutput.ui")]
pub struct SinkBox {
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
    pub resetDefaultSink: Arc<RefCell<Sink>>,
    pub resetSinkList: Arc<Mutex<Vec<Sink>>>,
    pub resetInputStreamList: Arc<Mutex<Vec<InputStream>>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SinkBox {
    const NAME: &'static str = "resetAudioOutput";
    type Type = sinkBox::SinkBox;
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
        obj.setupCallbacks();
    }
}

impl ListBoxRowImpl for SinkBox {}

impl WidgetImpl for SinkBox {}

impl WindowImpl for SinkBox {}

impl ApplicationWindowImpl for SinkBox {}
