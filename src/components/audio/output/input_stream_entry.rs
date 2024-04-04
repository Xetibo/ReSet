use std::sync::Arc;

use crate::components::audio::audio_entry::TAudioStream;
use crate::components::audio::audio_functions::new_stream_entry;
use glib::subclass::types::ObjectSubclassIsExt;
use re_set_lib::audio::audio_structures::{InputStream, Sink};

use super::sink_box::SinkBox;
use super::sink_entry::SinkEntry;

glib::wrapper! {
    pub struct InputStreamEntry(ObjectSubclass<super::input_stream_entry_impl::InputStreamEntry>)
    @extends adw::PreferencesGroup, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for InputStreamEntry {}
unsafe impl Sync for InputStreamEntry {}

impl TAudioStream<super::input_stream_entry_impl::InputStreamEntry> for InputStreamEntry {
    fn entry_imp(&self) -> &super::input_stream_entry_impl::InputStreamEntry {
        self.imp()
    }
}

impl InputStreamEntry {
    pub fn new(source_box: Arc<SinkBox>, stream: InputStream) -> Arc<Self> {
        new_stream_entry::<
            Sink,
            InputStream,
            SinkEntry,
            super::sink_entry_impl::SinkEntry,
            InputStreamEntry,
            super::input_stream_entry_impl::InputStreamEntry,
            SinkBox,
            super::sink_box_impl::SinkBox,
        >(source_box, stream)
    }
}
