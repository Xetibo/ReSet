use re_set_lib::audio::audio_structures::{OutputStream, Source};
use re_set_lib::signals::{
    OutputStreamAdded, OutputStreamChanged, OutputStreamRemoved, SourceAdded, SourceChanged,
    SourceRemoved,
};
use std::sync::Arc;

use adw::glib::Object;
use dbus::blocking::Connection;
use dbus::message::SignalArgs;
use dbus::Path;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::gio;

use crate::components::audio::generic_audio_box_handlers::populate_audio_objects;
use crate::components::audio::generic_audio_box_utils::{
    populate_audio_object_information, populate_cards, populate_streams, setup_audio_box_callbacks,
};
use crate::components::audio::generic_entry::TAudioBox;
use crate::components::audio::generic_utils::audio_dbus_call;
use crate::components::audio::input::source_box_impl;
use crate::components::base::error::{self};
use crate::components::base::error_impl::ReSetErrorImpl;
use crate::components::utils::{BASE, DBUS_PATH};

use super::output_stream_entry::OutputStreamEntry;
use super::source_box_handlers::{
    output_stream_added_handler, output_stream_changed_handler, output_stream_removed_handler,
    source_added_handler, source_changed_handler, source_removed_handler,
};
use super::source_const::{GETDEFAULT, GETOBJECTS, GETSTREAMS, SETDEFAULT, SETMUTE, SETVOLUME};
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
    let source_added =
        SourceAdded::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();
    let source_removed =
        SourceRemoved::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();
    let source_changed =
        SourceChanged::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();
    let output_stream_added =
        OutputStreamAdded::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH)))
            .static_clone();
    let output_stream_removed =
        OutputStreamRemoved::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH)))
            .static_clone();
    let output_stream_changed =
        OutputStreamChanged::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH)))
            .static_clone();

    let source_added_box = source_box.clone();
    let source_removed_box = source_box.clone();
    let source_changed_box = source_box.clone();
    let output_stream_added_box = source_box.clone();
    let output_stream_removed_box = source_box.clone();
    let output_stream_changed_box = source_box.clone();

    let res = conn.add_match(source_added, move |ir: SourceAdded, _, _| {
        source_added_handler(source_added_box.clone(), ir)
    });
    if res.is_err() {
        // TODO: handle this with the log/error macro
        println!("fail on source add event");
        return conn;
    }

    let res = conn.add_match(source_removed, move |ir: SourceRemoved, _, _| {
        source_removed_handler(source_removed_box.clone(), ir)
    });
    if res.is_err() {
        println!("fail on source remove event");
        return conn;
    }

    let res = conn.add_match(source_changed, move |ir: SourceChanged, _, _| {
        source_changed_handler(source_changed_box.clone(), ir)
    });
    if res.is_err() {
        println!("fail on source change event");
        return conn;
    }

    let res = conn.add_match(output_stream_added, move |ir: OutputStreamAdded, _, _| {
        output_stream_added_handler(output_stream_added_box.clone(), ir)
    });
    if res.is_err() {
        println!("fail on output stream add event");
        return conn;
    }

    let res = conn.add_match(
        output_stream_changed,
        move |ir: OutputStreamChanged, _, _| {
            output_stream_changed_handler(output_stream_changed_box.clone(), ir)
        },
    );
    if res.is_err() {
        println!("fail on output stream change event");
        return conn;
    }

    let res = conn.add_match(
        output_stream_removed,
        move |ir: OutputStreamRemoved, _, _| {
            output_stream_removed_handler(output_stream_removed_box.clone(), ir)
        },
    );
    if res.is_err() {
        println!("fail on output stream remove event");
        return conn;
    }

    conn
}
