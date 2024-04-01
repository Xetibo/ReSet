use adw::subclass::prelude::PreferencesGroupImpl;
use adw::{ActionRow, PreferencesGroup};
use re_set_lib::audio::audio_structures::Source;
use std::cell::RefCell;
use std::sync::Arc;
use std::time::SystemTime;

use gtk::subclass::prelude::*;
use gtk::{Button, CheckButton, CompositeTemplate, Label, Scale};

use crate::components::audio::generic_entry::{AudioIcons, TAudioEntryImpl, DBusFunction};

use super::source_const::{ICONS, SETDEFAULT, SETMUTE, SETVOLUME};
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

impl TAudioEntryImpl<Source> for SourceEntry {
    fn name(&self) -> &TemplateChild<ActionRow> {
        &self.reset_source_name
    }

    fn selected_audio_object(&self) -> &TemplateChild<CheckButton> {
        &self.reset_selected_source
    }

    fn mute(&self) -> &TemplateChild<Button> {
        &self.reset_source_mute
    }

    fn volume_slider(&self) -> &TemplateChild<Scale> {
        &self.reset_volume_slider
    }

    fn volume_percentage(&self) -> &TemplateChild<Label> {
        &self.reset_volume_percentage
    }

    fn audio_object(&self) -> Arc<RefCell<Source>> {
        self.source.clone()
    }

    fn volume_time_stamp(&self) -> &RefCell<Option<SystemTime>> {
        &self.volume_time_stamp
    }

    fn set_volume_fn(&self) -> &'static DBusFunction {
        &SETVOLUME
    }

    fn set_audio_object_fn(&self) -> &'static DBusFunction {
        &SETDEFAULT
    }

    fn set_mute_fn(&self) -> &'static DBusFunction {
        &SETMUTE
    }

    fn icons(&self) -> &AudioIcons {
        &ICONS
    }
}
