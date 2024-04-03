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
use adw::prelude::ComboRowExt;
use adw::prelude::ListBoxRowExt;
use dbus::blocking::Connection;
use dbus::message::SignalArgs;
use dbus::Path;
use glib::subclass::prelude::ObjectSubclassIsExt;
use glib::Variant;
use gtk::gio;
use gtk::prelude::ActionableExt;

use crate::components::audio::generic_audio_box_utils::setup_audio_box_callbacks;
use crate::components::audio::generic_entry::TAudioBox;
use crate::components::base::error_impl::ReSetErrorImpl;
use crate::components::utils::BASE;
use crate::components::utils::DBUS_PATH;
use crate::components::utils::{create_dropdown_label_factory, set_combo_row_ellipsis};

use super::input_stream_entry::InputStreamEntry;
use super::sink_box_handlers::input_stream_added_handler;
use super::sink_box_handlers::input_stream_changed_handler;
use super::sink_box_handlers::input_stream_removed_handler;
use super::sink_box_handlers::sink_added_handler;
use super::sink_box_handlers::sink_changed_handler;
use super::sink_box_handlers::sink_removed_handler;
use super::sink_box_impl;
use super::sink_box_utils::get_default_sink;
use super::sink_box_utils::get_sinks;
use super::sink_box_utils::populate_cards;
use super::sink_box_utils::populate_inputstreams;
use super::sink_box_utils::populate_sink_information;
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
    gio::spawn_blocking(move || {
        let sinks = get_sinks(sink_box.clone());
        {
            let sink_box_imp = sink_box.imp();
            let list = sink_box_imp.reset_model_list.write().unwrap();
            let mut map = sink_box_imp.reset_sink_map.write().unwrap();
            let mut model_index = sink_box_imp.reset_model_index.write().unwrap();
            sink_box_imp
                .reset_default_sink
                .replace(get_default_sink(sink_box.clone()));
            for sink in sinks.iter() {
                list.append(&sink.alias);
                map.insert(sink.alias.clone(), (sink.index, sink.name.clone()));
                *model_index += 1;
            }
        }
        populate_inputstreams(sink_box.clone());
        populate_cards(sink_box.clone());
        populate_sink_information(sink_box, sinks);
    });
}

pub fn start_sink_box_listener(conn: Connection, sink_box: Arc<SinkBox>) -> Connection {
    let sink_added =
        SinkAdded::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();
    let sink_removed =
        SinkRemoved::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();
    let sink_changed =
        SinkChanged::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();
    let input_stream_added =
        InputStreamAdded::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH)))
            .static_clone();
    let input_stream_removed =
        InputStreamRemoved::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH)))
            .static_clone();
    let input_stream_changed =
        InputStreamChanged::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH)))
            .static_clone();

    let sink_added_box = sink_box.clone();
    let sink_removed_box = sink_box.clone();
    let sink_changed_box = sink_box.clone();
    let input_stream_added_box = sink_box.clone();
    let input_stream_removed_box = sink_box.clone();
    let input_stream_changed_box = sink_box.clone();

    let res = conn.add_match(sink_added, move |ir: SinkAdded, _, _| {
        sink_added_handler(sink_added_box.clone(), ir)
    });
    if res.is_err() {
        // TODO: handle this with the log/error macro
        println!("fail on sink add event");
        return conn;
    }

    let res = conn.add_match(sink_removed, move |ir: SinkRemoved, _, _| {
        sink_removed_handler(sink_removed_box.clone(), ir)
    });
    if res.is_err() {
        println!("fail on sink remove event");
        return conn;
    }

    let res = conn.add_match(sink_changed, move |ir: SinkChanged, _, _| {
        sink_changed_handler(sink_changed_box.clone(), ir)
    });
    if res.is_err() {
        println!("fail on sink change event");
        return conn;
    }

    let res = conn.add_match(input_stream_added, move |ir: InputStreamAdded, _, _| {
        input_stream_added_handler(input_stream_added_box.clone(), ir)
    });
    if res.is_err() {
        println!("fail on input stream add event");
        return conn;
    }

    let res = conn.add_match(input_stream_removed, move |ir: InputStreamRemoved, _, _| {
        input_stream_removed_handler(input_stream_removed_box.clone(), ir)
    });
    if res.is_err() {
        println!("fail on input stream remove event");
        return conn;
    }

    let res = conn.add_match(input_stream_changed, move |ir: InputStreamChanged, _, _| {
        input_stream_changed_handler(input_stream_changed_box.clone(), ir)
    });
    if res.is_err() {
        println!("fail on input stream change event");
        return conn;
    }

    conn
}
