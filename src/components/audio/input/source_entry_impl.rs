use adw::subclass::prelude::PreferencesGroupImpl;
use adw::{ActionRow, PreferencesGroup};
use re_set_lib::audio::audio_structures::Source;
use std::cell::RefCell;
use std::sync::Arc;
use std::time::SystemTime;

use gtk::subclass::prelude::*;
use gtk::{Button, CheckButton, CompositeTemplate, Label, Scale};

use super::source_entry;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetSourceEntry.ui")]
pub struct SourceEntry {
    #[template_child]
    pub reset_source_name: TemplateChild<ActionRow>,
    #[template_child]
    pub reset_selected_source: TemplateChild<CheckButton>,
    #[template_child]
    pub reset_source_mute: TemplateChild<Button>,
    #[template_child]
    pub reset_volume_slider: TemplateChild<Scale>,
    #[template_child]
    pub reset_volume_percentage: TemplateChild<Label>,
    pub source: Arc<RefCell<Source>>,
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
