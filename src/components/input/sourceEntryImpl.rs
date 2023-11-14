use std::cell::RefCell;
use std::sync::Arc;

use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, DropDown, Label, ProgressBar, Scale, CheckButton};
use ReSet_Lib::audio::audio::Source;

use super::sourceEntry;

#[allow(non_snake_case)]
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/Xetibo/ReSet/resetSourceEntry.ui")]
pub struct SourceEntry {
    #[template_child]
    pub resetSourceName: TemplateChild<Label>,
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
}

#[glib::object_subclass]
impl ObjectSubclass for SourceEntry {
    const NAME: &'static str = "resetSourceEntry";
    type Type = sourceEntry::SourceEntry;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl BoxImpl for SourceEntry {}

impl ObjectImpl for SourceEntry {}

impl WidgetImpl for SourceEntry {}
