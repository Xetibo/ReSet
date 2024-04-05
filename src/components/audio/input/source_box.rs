use re_set_lib::audio::audio_structures::{OutputStream, Source};
use re_set_lib::signals::{
    OutputStreamAdded, OutputStreamChanged, OutputStreamRemoved, SourceAdded, SourceChanged,
    SourceRemoved,
};
use std::sync::Arc;

use adw::glib::Object;
use dbus::blocking::Connection;
use glib::subclass::prelude::ObjectSubclassIsExt;

use crate::components::audio::audio_box_handlers::populate_audio_objects;
use crate::components::audio::audio_box_utils::{
    setup_audio_box_callbacks, start_audio_box_listener,
};
use crate::components::audio::audio_entry::TAudioBox;
use crate::components::audio::input::source_box_impl;
use crate::components::base::error::{self};
use crate::components::base::error_impl::ReSetErrorImpl;

use super::output_stream_entry::OutputStreamEntry;
use super::source_const::{
    DUMMY, GETDEFAULT, GETDEFAULTNAME, GETOBJECTS, GETSTREAMS, SETDEFAULT, SETMUTE, SETVOLUME
};
use super::source_entry::SourceEntry;

glib::wrapper! {
    pub struct SourceBox(ObjectSubclass<source_box_impl::SourceBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for SourceBox {}
unsafe impl Sync for SourceBox {}

impl ReSetErrorImpl for SourceBox {
    fn error(&self) -> &gtk::subclass::prelude::TemplateChild<error::ReSetError> {
        &self.imp().error
    }
}

impl TAudioBox<super::source_box_impl::SourceBox> for SourceBox {
    fn box_imp(&self) -> &super::source_box_impl::SourceBox {
        self.imp()
    }
}

impl SourceBox {
    pub fn new() -> Self {
        let mut obj: Self = Object::builder().build();
        setup_audio_box_callbacks::<
            Source,
            OutputStream,
            SourceEntry,
            super::source_entry_impl::SourceEntry,
            OutputStreamEntry,
            super::output_stream_entry_impl::OutputStreamEntry,
            SourceBox,
            super::source_box_impl::SourceBox,
        >(&mut obj);
        {
            let imp = obj.imp();
            let mut model_index = imp.reset_model_index.write().unwrap();
            *model_index = 0;
        }
        obj
    }
}

impl Default for SourceBox {
    fn default() -> Self {
        Self::new()
    }
}

pub fn populate_sources(source_box: Arc<SourceBox>) {
    populate_audio_objects::<
        Source,
        OutputStream,
        SourceEntry,
        super::source_entry_impl::SourceEntry,
        OutputStreamEntry,
        super::output_stream_entry_impl::OutputStreamEntry,
        SourceBox,
        super::source_box_impl::SourceBox,
    >(
        source_box,
        &GETOBJECTS,
        &GETDEFAULT,
        &SETDEFAULT,
        &GETSTREAMS,
        &SETVOLUME,
        &SETMUTE,
    );
}

pub fn start_source_box_listener(conn: Connection, source_box: Arc<SourceBox>) -> Connection {
    start_audio_box_listener::<
        Source,
        OutputStream,
        SourceEntry,
        super::source_entry_impl::SourceEntry,
        OutputStreamEntry,
        super::output_stream_entry_impl::OutputStreamEntry,
        SourceBox,
        super::source_box_impl::SourceBox,
        SourceAdded,
        SourceChanged,
        SourceRemoved,
        OutputStreamAdded,
        OutputStreamChanged,
        OutputStreamRemoved,
    >(conn, source_box, &GETDEFAULTNAME, DUMMY)
}
