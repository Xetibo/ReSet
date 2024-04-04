use re_set_lib::audio::audio_structures::InputStream;
use re_set_lib::audio::audio_structures::Sink;
use re_set_lib::signals::InputStreamAdded;
use re_set_lib::signals::InputStreamChanged;
use re_set_lib::signals::InputStreamRemoved;
use re_set_lib::signals::SinkAdded;
use re_set_lib::signals::SinkChanged;
use re_set_lib::signals::SinkRemoved;
use std::sync::Arc;

use adw::glib::Object;
use dbus::blocking::Connection;
use glib::subclass::prelude::ObjectSubclassIsExt;

use crate::components::audio::audio_box_handlers::populate_audio_objects;
use crate::components::audio::audio_box_utils::setup_audio_box_callbacks;
use crate::components::audio::audio_box_utils::start_audio_box_listener;
use crate::components::audio::audio_entry::TAudioBox;
use crate::components::base::error_impl::ReSetErrorImpl;

use super::input_stream_entry::InputStreamEntry;
use super::sink_box_impl;
use super::sink_const::GETDEFAULT;
use super::sink_const::GETDEFAULTNAME;
use super::sink_const::GETOBJECTS;
use super::sink_const::GETSTREAMS;
use super::sink_const::SETDEFAULT;
use super::sink_const::SETMUTE;
use super::sink_const::SETVOLUME;
use super::sink_entry::SinkEntry;

glib::wrapper! {
    pub struct SinkBox(ObjectSubclass<sink_box_impl::SinkBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for SinkBox {}
unsafe impl Sync for SinkBox {}

impl ReSetErrorImpl for SinkBox {
    fn error(
        &self,
    ) -> &gtk::subclass::prelude::TemplateChild<crate::components::base::error::ReSetError> {
        &self.imp().error
    }
}

impl TAudioBox<super::sink_box_impl::SinkBox> for SinkBox {
    fn box_imp(&self) -> &super::sink_box_impl::SinkBox {
        self.imp()
    }
}

impl SinkBox {
    pub fn new() -> Self {
        let mut obj: Self = Object::builder().build();
        setup_audio_box_callbacks::<
            Sink,
            InputStream,
            SinkEntry,
            super::sink_entry_impl::SinkEntry,
            InputStreamEntry,
            super::input_stream_entry_impl::InputStreamEntry,
            SinkBox,
            super::sink_box_impl::SinkBox,
        >(&mut obj);
        {
            let imp = obj.imp();
            let mut model_index = imp.reset_model_index.write().unwrap();
            *model_index = 0;
        }
        obj
    }
}

impl Default for SinkBox {
    fn default() -> Self {
        Self::new()
    }
}

pub fn populate_sinks(sink_box: Arc<SinkBox>) {
    populate_audio_objects::<
        Sink,
        InputStream,
        SinkEntry,
        super::sink_entry_impl::SinkEntry,
        InputStreamEntry,
        super::input_stream_entry_impl::InputStreamEntry,
        SinkBox,
        super::sink_box_impl::SinkBox,
    >(
        sink_box,
        &GETOBJECTS,
        &GETDEFAULT,
        &SETDEFAULT,
        &GETSTREAMS,
        &SETVOLUME,
        &SETMUTE,
    );
}

pub fn start_sink_box_listener(conn: Connection, sink_box: Arc<SinkBox>) -> Connection {
    start_audio_box_listener::<
        Sink,
        InputStream,
        SinkEntry,
        super::sink_entry_impl::SinkEntry,
        InputStreamEntry,
        super::input_stream_entry_impl::InputStreamEntry,
        SinkBox,
        super::sink_box_impl::SinkBox,
        SinkAdded,
        SinkChanged,
        SinkRemoved,
        InputStreamAdded,
        InputStreamChanged,
        InputStreamRemoved,
    >(conn, sink_box, &GETDEFAULTNAME)
}
