use adw::subclass::prelude::PreferencesGroupImpl;
use adw::{ComboRow, PreferencesGroup};
use std::cell::RefCell;
use std::sync::Arc;
use std::time::SystemTime;

use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Label, ProgressBar, Scale};
use ReSet_Lib::audio::audio::InputStream;

use super::inputStreamEntry;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetInputStreamEntry.ui")]
pub struct InputStreamEntry {
    #[template_child]
    pub resetSinkSelection: TemplateChild<ComboRow>,
    #[template_child]
    pub resetSinkMute: TemplateChild<Button>,
    #[template_child]
    pub resetVolumeSlider: TemplateChild<Scale>,
    #[template_child]
    pub resetVolumePercentage: TemplateChild<Label>,
    #[template_child]
    pub resetVolumeMeter: TemplateChild<ProgressBar>,
    pub stream: Arc<RefCell<InputStream>>,
    pub associatedSink: Arc<RefCell<(u32, String)>>,
    pub volumeTimeStamp: RefCell<Option<SystemTime>>,
}

#[glib::object_subclass]
impl ObjectSubclass for InputStreamEntry {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetInputStreamEntry";
    type Type = inputStreamEntry::InputStreamEntry;
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
