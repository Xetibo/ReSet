use std::sync::Arc;

use crate::components::audio::audio_entry::{new_entry, TAudioEntry};
use glib::subclass::types::ObjectSubclassIsExt;
use gtk::CheckButton;
use re_set_lib::audio::audio_structures::{OutputStream, Source};

use super::output_stream_entry::OutputStreamEntry;
use super::source_box::SourceBox;
use super::source_entry_impl;

glib::wrapper! {
    pub struct SourceEntry(ObjectSubclass<source_entry_impl::SourceEntry>)
    @extends adw::PreferencesGroup, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for SourceEntry {}
unsafe impl Sync for SourceEntry {}

impl TAudioEntry<super::source_entry_impl::SourceEntry> for SourceEntry {
    fn entry_imp(&self) -> &super::source_entry_impl::SourceEntry {
        self.imp()
    }
}

impl SourceEntry {
    pub fn new(
        is_default: bool,
        check_group: Arc<CheckButton>,
        source: Source,
        input_box: Arc<SourceBox>,
    ) -> Arc<Self> {
        new_entry::<
            Source,
            OutputStream,
            SourceEntry,
            super::source_entry_impl::SourceEntry,
            OutputStreamEntry,
            super::output_stream_entry_impl::OutputStreamEntry,
            SourceBox,
            super::source_box_impl::SourceBox,
        >(is_default, check_group, source, input_box)
    }
}
