use std::{sync::Arc, time::Duration};

use adw::prelude::{ComboRowExt, PreferencesGroupExt};
use dbus::{blocking::Connection, Error};
use glib::subclass::types::ObjectSubclassIsExt;
use gtk::{
    gio,
    prelude::{BoxExt, ButtonExt, CheckButtonExt, ListBoxRowExt, RangeExt},
};
use re_set_lib::audio::audio_structures::{Card, InputStream, Sink};

use crate::components::{
    base::{card_entry::CardEntry, error_impl::show_error, list_entry::ListEntry},
    utils::{AUDIO, BASE, DBUS_PATH},
};

use super::{
    input_stream_entry::InputStreamEntry,
    sink_box::SinkBox,
    sink_box_handlers::{drop_down_handler, mute_handler, volume_slider_handler},
    sink_entry::SinkEntry,
};

pub fn populate_sink_information(sink_box: Arc<SinkBox>, sinks: Vec<Sink>) {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let sink_box_ref_select = sink_box.clone();
            let sink_box_ref_slider = sink_box.clone();
            let sink_box_ref_mute = sink_box.clone();
            let sink_box_ref = sink_box.clone();
            {
                let sink_box_imp = sink_box_ref.imp();
                let default_sink = sink_box_imp.reset_default_sink.clone();
                let sink = default_sink.borrow();

                if sink.muted {
                    sink_box_imp
                        .reset_sink_mute
                        .set_icon_name("audio-volume-muted-symbolic");
                } else {
                    sink_box_imp
                        .reset_sink_mute
                        .set_icon_name("audio-volume-high-symbolic");
                }

                let volume = sink.volume.first().unwrap_or(&0);
                let fraction = (*volume as f64 / 655.36).round();
                let percentage = (fraction).to_string() + "%";
                sink_box_imp.reset_volume_percentage.set_text(&percentage);
                sink_box_imp.reset_volume_slider.set_value(*volume as f64);
                let mut list = sink_box_imp.reset_sink_list.write().unwrap();
                for sink in sinks {
                    let index = sink.index;
                    let alias = sink.alias.clone();
                    let mut is_default = false;
                    if sink_box_imp.reset_default_sink.borrow().name == sink.name {
                        is_default = true;
                    }
                    let sink_entry = SinkEntry::new(
                        is_default,
                        sink_box_imp.reset_default_check_button.clone(),
                        sink,
                        sink_box.clone(),
                    );
                    let sink_clone = sink_entry.clone();
                    let entry = Arc::new(ListEntry::new(&*sink_entry));
                    entry.set_activatable(false);
                    list.insert(index, (entry.clone(), sink_clone, alias));
                    sink_box_imp.reset_sinks.append(&*entry);
                }
                let list = sink_box_imp.reset_model_list.read().unwrap();
                sink_box_imp.reset_sink_dropdown.set_model(Some(&*list));
                let name = sink_box_imp.reset_default_sink.borrow();

                let index = sink_box_imp.reset_model_index.read().unwrap();
                let model_list = sink_box_imp.reset_model_list.read().unwrap();
                for entry in 0..*index {
                    if model_list.string(entry) == Some(name.alias.clone().into()) {
                        sink_box_imp.reset_sink_dropdown.set_selected(entry);
                        break;
                    }
                }
                sink_box_imp
                    .reset_sink_dropdown
                    .connect_selected_notify(move |dropdown| {
                        drop_down_handler(sink_box_ref_select.clone(), dropdown);
                    });
            }
            sink_box_ref
                .imp()
                .reset_volume_slider
                .connect_change_value(move |_, _, value| {
                    volume_slider_handler(sink_box_ref_slider.clone(), value)
                });
            sink_box_ref
                .imp()
                .reset_sink_mute
                .connect_clicked(move |_| {
                    mute_handler(sink_box_ref_mute.clone());
                });
        });
    });
}

pub fn refresh_default_sink(new_sink: Sink, sink_box: Arc<SinkBox>, entry: bool) {
    let volume = *new_sink.volume.first().unwrap_or(&0_u32);
    let fraction = (volume as f64 / 655.36).round();
    let percentage = (fraction).to_string() + "%";
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let imp = sink_box.imp();
            if !entry {
                let list = imp.reset_sink_list.read().unwrap();
                let entry = list.get(&new_sink.index);
                if entry.is_none() {
                    return;
                }
                let entry_imp = entry.unwrap().1.imp();
                entry_imp.reset_selected_sink.set_active(true);
            } else {
                let index = imp.reset_model_index.read().unwrap();
                let model_list = imp.reset_model_list.read().unwrap();
                for entry in 0..*index {
                    if model_list.string(entry) == Some(new_sink.alias.clone().into()) {
                        imp.reset_sink_dropdown.set_selected(entry);
                        break;
                    }
                }
            }
            imp.reset_volume_percentage.set_text(&percentage);
            imp.reset_volume_slider.set_value(volume as f64);
            if new_sink.muted {
                imp.reset_sink_mute
                    .set_icon_name("audio-volume-muted-symbolic");
            } else {
                imp.reset_sink_mute
                    .set_icon_name("audio-volume-high-symbolic");
            }
            imp.reset_default_sink.replace(new_sink);
        });
    });
}

pub fn populate_inputstreams(sink_box: Arc<SinkBox>) {
    gio::spawn_blocking(move || {
        let streams = get_input_streams(sink_box.clone());
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let sink_box_imp = sink_box.imp();
                let mut list = sink_box_imp.reset_input_stream_list.write().unwrap();
                for stream in streams {
                    let index = stream.index;
                    let input_stream = InputStreamEntry::new(sink_box.clone(), stream);
                    let entry = Arc::new(ListEntry::new(&*input_stream));
                    entry.set_activatable(false);
                    list.insert(index, (entry.clone(), input_stream.clone()));
                    sink_box_imp.reset_input_streams.append(&*entry);
                }
            });
        });
    });
}

pub fn populate_cards(sink_box: Arc<SinkBox>) {
    gio::spawn_blocking(move || {
        let sink_box_ref = sink_box.clone();
        let cards = get_cards(sink_box.clone());
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let imp = sink_box_ref.imp();
                for card in cards {
                    imp.reset_cards.add(&CardEntry::new(card));
                }
            });
        });
    });
}

pub fn get_input_streams(sink_box: Arc<SinkBox>) -> Vec<InputStream> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Vec<InputStream>,), Error> = proxy.method_call(AUDIO, "ListInputStreams", ());
    if res.is_err() {
        show_error::<SinkBox>(sink_box.clone(), "Failed to list input streams");
        return Vec::new();
    }
    res.unwrap().0
}

pub fn get_sinks(sink_box: Arc<SinkBox>) -> Vec<Sink> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Vec<Sink>,), Error> = proxy.method_call(AUDIO, "ListSinks", ());
    if res.is_err() {
        show_error::<SinkBox>(sink_box.clone(), "Failed to list sinks");
        return Vec::new();
    }
    res.unwrap().0
}

pub fn get_cards(sink_box: Arc<SinkBox>) -> Vec<Card> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Vec<Card>,), Error> = proxy.method_call(AUDIO, "ListCards", ());
    if res.is_err() {
        show_error::<SinkBox>(sink_box.clone(), "Failed to list profiles");
        return Vec::new();
    }
    res.unwrap().0
}

pub fn get_default_sink_name(sink_box: Arc<SinkBox>) -> String {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(String,), Error> = proxy.method_call(AUDIO, "GetDefaultSinkName", ());
    if res.is_err() {
        show_error::<SinkBox>(sink_box.clone(), "Failed to get default sink name");
        return String::from("");
    }
    res.unwrap().0
}

pub fn get_default_sink(sink_box: Arc<SinkBox>) -> Sink {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Sink,), Error> = proxy.method_call(AUDIO, "GetDefaultSink", ());
    if res.is_err() {
        show_error::<SinkBox>(sink_box.clone(), "Failed to get default sink");
        return Sink::default();
    }
    res.unwrap().0
}
