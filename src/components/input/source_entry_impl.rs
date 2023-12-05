use adw::subclass::prelude::PreferencesGroupImpl;
use adw::{ActionRow, PreferencesGroup};
use std::cell::RefCell;
use std::sync::Arc;
use std::time::SystemTime;

use gtk::subclass::prelude::*;
use gtk::{glib, Button, CheckButton, CompositeTemplate, Label, ProgressBar, Scale};
use ReSet_Lib::audio::audio::Source;

use super::source_entry;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetSourceEntry.ui")]
pub struct SourceEntry {
    #[template_child]
    pub resetSourceName: TemplateChild<ActionRow>,
    #[template_child]
    pub resetSelectedSource: TemplateChild<CheckButton>,
    #[template_child]
    pub resetSourceMute: TemplateChild<Button>,
    #[template_child]
    pub resetVolumeSlider: TemplateChild<Scale>,
    #[template_child]
    pub resetVolumePercentage: TemplateChild<Label>,
    #[template_child]
    pub resetVolumeMeter: TemplateChild<ProgressBar>,
    pub stream: Arc<RefCell<Source>>,
    pub volume_time_stamp: RefCell<Option<SystemTime>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SourceEntry {
    const ABSTRACT: bool = false;
    const NAME: &'static str = "resetSourceEntry";
    type Type = source_entry::SourceEntry;
    type ParentType = PreferencesGroup;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl PreferencesGroupImpl for SourceEntry {}

impl ObjectImpl for SourceEntry {}

impl WidgetImpl for SourceEntry {}
