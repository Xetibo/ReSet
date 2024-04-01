use std::sync::Arc;

use crate::components::audio::generic_entry::{new_entry, TAudioEntry};
use glib::subclass::types::ObjectSubclassIsExt;
use gtk::CheckButton;
use re_set_lib::audio::audio_structures::{InputStream, Sink};

use super::input_stream_entry::InputStreamEntry;
use super::sink_box::SinkBox;
use super::sink_entry_impl;

glib::wrapper! {
    pub struct SinkEntry(ObjectSubclass<sink_entry_impl::SinkEntry>)
    @extends adw::PreferencesGroup, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for SinkEntry {}
unsafe impl Sync for SinkEntry {}

impl TAudioEntry<super::sink_entry_impl::SinkEntry> for SinkEntry {
    fn entry_imp(&self) -> &super::sink_entry_impl::SinkEntry {
        self.imp()
    }
}

impl SinkEntry {
    pub fn new(
        is_default: bool,
        check_group: Arc<CheckButton>,
        sink: Sink,
        output_box: Arc<SinkBox>,
    ) -> Arc<Self> {
        new_entry::<
            Sink,
            InputStream,
            SinkEntry,
            super::sink_entry_impl::SinkEntry,
            InputStreamEntry,
            super::input_stream_entry_impl::InputStreamEntry,
            SinkBox,
            super::sink_box_impl::SinkBox,
        >(is_default, check_group, sink, output_box)
    }
}
