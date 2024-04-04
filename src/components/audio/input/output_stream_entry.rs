use std::sync::Arc;

use crate::components::audio::audio_entry::TAudioStream;
use crate::components::audio::audio_functions::new_stream_entry;
use glib::subclass::types::ObjectSubclassIsExt;
use re_set_lib::audio::audio_structures::{OutputStream, Source};

use super::output_stream_entry_impl;
use super::source_box::SourceBox;
use super::source_entry::SourceEntry;

glib::wrapper! {
    pub struct OutputStreamEntry(ObjectSubclass<output_stream_entry_impl::OutputStreamEntry>)
    @extends adw::PreferencesGroup, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for OutputStreamEntry {}
unsafe impl Sync for OutputStreamEntry {}

impl TAudioStream<super::output_stream_entry_impl::OutputStreamEntry> for OutputStreamEntry {
    fn entry_imp(&self) -> &super::output_stream_entry_impl::OutputStreamEntry {
        self.imp()
    }
}

impl OutputStreamEntry {
    pub fn new(source_box: Arc<SourceBox>, stream: OutputStream) -> Arc<Self> {
        new_stream_entry::<
            Source,
            OutputStream,
            SourceEntry,
            super::source_entry_impl::SourceEntry,
            OutputStreamEntry,
            super::output_stream_entry_impl::OutputStreamEntry,
            SourceBox,
            super::source_box_impl::SourceBox,
        >(source_box, stream)
    }
}
