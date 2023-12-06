use adw::prelude::PreferencesGroupExt;
use adw::prelude::PreferencesRowExt;
use re_set_lib::audio::audio_structures::Card;
use re_set_lib::audio::audio_structures::InputStream;
use re_set_lib::audio::audio_structures::Sink;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use adw::glib::Object;
use adw::prelude::{BoxExt, ButtonExt, CheckButtonExt, ComboRowExt, RangeExt};
use adw::{glib, prelude::ListBoxRowExt};
use dbus::blocking::Connection;
use dbus::message::SignalArgs;
use dbus::{Error, Path};
use glib::subclass::prelude::ObjectSubclassIsExt;
use glib::{clone, Cast, Propagation, Variant};
use gtk::prelude::ActionableExt;
use gtk::{gio, StringObject};

use crate::components::base::card_entry::CardEntry;
use crate::components::base::list_entry::ListEntry;
use crate::components::base::utils::{
    InputStreamAdded, InputStreamChanged, InputStreamRemoved, SinkAdded, SinkChanged, SinkRemoved,
};
use crate::components::output::sink_entry::set_sink_volume;
use crate::components::utils::{create_dropdown_label_factory, set_combo_row_ellipsis};

use super::input_stream_entry::InputStreamEntry;
use super::sink_box_impl;
use super::sink_entry::{set_default_sink, toggle_sink_mute, SinkEntry};

glib::wrapper! {
    pub struct SinkBox(ObjectSubclass<sink_box_impl::SinkBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for SinkBox {}
unsafe impl Sync for SinkBox {}

impl SinkBox {
    pub fn new() -> Self {
        let obj: Self = Object::builder().build();
        {
            let imp = obj.imp();
            let mut model_index = imp.reset_model_index.write().unwrap();
            *model_index = 0;
        }
        obj
    }

    pub fn setup_callbacks(&self) {
        let self_imp = self.imp();
        self_imp.reset_sinks_row.set_activatable(true);
        self_imp
            .reset_sinks_row
            .set_action_name(Some("navigation.push"));
        self_imp
            .reset_sinks_row
            .set_action_target_value(Some(&Variant::from("outputDevices")));
        self_imp.reset_cards_row.set_activatable(true);
        self_imp
            .reset_cards_row
            .set_action_name(Some("navigation.push"));
        self_imp
            .reset_cards_row
            .set_action_target_value(Some(&Variant::from("profileConfiguration")));
        self_imp.reset_cards_row.connect_action_name_notify(|_| {});

        self_imp.reset_input_stream_button.set_activatable(true);
        self_imp
            .reset_input_stream_button
            .set_action_name(Some("navigation.pop"));

        self_imp.reset_input_cards_back_button.set_activatable(true);
        self_imp
            .reset_input_cards_back_button
            .set_action_name(Some("navigation.pop"));

        self_imp
            .reset_sink_dropdown
            .set_factory(Some(&create_dropdown_label_factory()));
        set_combo_row_ellipsis(self_imp.reset_sink_dropdown.get());
    }
}

impl Default for SinkBox {
    fn default() -> Self {
        Self::new()
    }
}

pub fn populate_sinks(output_box: Arc<SinkBox>) {
    gio::spawn_blocking(move || {
        let output_box_ref = output_box.clone();
        let sinks = get_sinks();
        {
            let output_box_imp = output_box.imp();
            let mut map = output_box_imp.reset_sink_map.write().unwrap();
            let list = output_box_imp.reset_model_list.write().unwrap();
            let mut model_index = output_box_imp.reset_model_index.write().unwrap();
            output_box_imp
                .reset_default_sink
                .replace(get_default_sink());
            for (i, sink) in (0_u32..).zip(sinks.iter()) {
                list.append(&sink.alias);
                map.insert(sink.alias.clone(), (sink.index, i, sink.name.clone()));
                *model_index += 1;
            }
        }
        populate_inputstreams(output_box.clone());
        populate_cards(output_box.clone());
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box_ref_slider = output_box.clone();
                let output_box_ref_mute = output_box.clone();
                {
                    let output_box_imp = output_box_ref.imp();
                    let default_sink = output_box_imp.reset_default_sink.clone();
                    let sink = default_sink.borrow();

                    let volume = sink.volume.first().unwrap_or(&0);
                    let fraction = (*volume as f64 / 655.36).round();
                    let percentage = (fraction).to_string() + "%";
                    output_box_imp.reset_volume_percentage.set_text(&percentage);
                    output_box_imp.reset_volume_slider.set_value(*volume as f64);
                    let mut list = output_box_imp.reset_sink_list.write().unwrap();
                    for sink in sinks {
                        let index = sink.index;
                        let alias = sink.alias.clone();
                        let mut is_default = false;
                        if output_box_imp.reset_default_sink.borrow().name == sink.name {
                            is_default = true;
                        }
                        let sink_entry = Arc::new(SinkEntry::new(
                            is_default,
                            output_box_imp.reset_default_check_button.clone(),
                            sink,
                        ));
                        let sink_clone = sink_entry.clone();
                        let entry = Arc::new(ListEntry::new(&*sink_entry));
                        entry.set_activatable(false);
                        list.insert(index, (entry.clone(), sink_clone, alias));
                        output_box_imp.reset_sinks.append(&*entry);
                    }
                    let list = output_box_imp.reset_model_list.read().unwrap();
                    output_box_imp.reset_sink_dropdown.set_model(Some(&*list));
                    let map = output_box_imp.reset_sink_map.read().unwrap();
                    let name = output_box_imp.reset_default_sink.borrow();
                    let name = &name.alias;
                    let index = map.get(name);
                    if let Some(index) = index {
                        output_box_imp.reset_sink_dropdown.set_selected(index.1);
                    }
                    output_box_imp.reset_sink_dropdown.connect_selected_notify(
                        clone!(@weak output_box_imp => move |dropdown| {
                            let selected = dropdown.selected_item();
                            if selected.is_none() {
                                return;
                            }
                            let selected = selected.unwrap();
                            let selected = selected.downcast_ref::<StringObject>().unwrap();
                            let selected = selected.string().to_string();

                            let sink = output_box_imp.reset_sink_map.read().unwrap();
                            let sink = sink.get(&selected);
                            if sink.is_none() {
                                return;
                            }
                            let sink = Arc::new(sink.unwrap().2.clone());
                            set_default_sink(sink);
                        }),
                    );
                }
                output_box_ref
                    .imp()
                    .reset_volume_slider
                    .connect_change_value(move |_, _, value| {
                        let imp = output_box_ref_slider.imp();
                        let fraction = (value / 655.36).round();
                        let percentage = (fraction).to_string() + "%";
                        imp.reset_volume_percentage.set_text(&percentage);
                        let sink = imp.reset_default_sink.borrow();
                        let index = sink.index;
                        let channels = sink.channels;
                        {
                            let mut time = imp.volume_time_stamp.borrow_mut();
                            if time.is_some()
                                && time.unwrap().elapsed().unwrap() < Duration::from_millis(50)
                            {
                                return Propagation::Proceed;
                            }
                            *time = Some(SystemTime::now());
                        }
                        set_sink_volume(value, index, channels);
                        Propagation::Proceed
                    });
                output_box_ref
                    .imp()
                    .reset_sink_mute
                    .connect_clicked(move |_| {
                        let imp = output_box_ref_mute.imp();
                        let mut stream = imp.reset_default_sink.borrow_mut();
                        stream.muted = !stream.muted;
                        if stream.muted {
                            imp.reset_sink_mute
                                .set_icon_name("audio-volume-muted-symbolic");
                        } else {
                            imp.reset_sink_mute
                                .set_icon_name("audio-volume-high-symbolic");
                        }
                        toggle_sink_mute(stream.index, stream.muted);
                    });
            });
        });
    });
}

pub fn populate_inputstreams(output_box: Arc<SinkBox>) {
    let output_box_ref = output_box.clone();

    gio::spawn_blocking(move || {
        let streams = get_input_streams();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box_imp = output_box_ref.imp();
                let mut list = output_box_imp.reset_input_stream_list.write().unwrap();
                for stream in streams {
                    let index = stream.index;
                    let input_stream = Arc::new(InputStreamEntry::new(output_box.clone(), stream));
                    let entry = Arc::new(ListEntry::new(&*input_stream));
                    entry.set_activatable(false);
                    list.insert(index, (entry.clone(), input_stream.clone()));
                    output_box_imp.reset_input_streams.append(&*entry);
                }
            });
        });
    });
}

pub fn populate_cards(output_box: Arc<SinkBox>) {
    gio::spawn_blocking(move || {
        let output_box_ref = output_box.clone();
        let cards = get_cards();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let imp = output_box_ref.imp();
                for card in cards {
                    imp.reset_cards.add(&CardEntry::new(card));
                }
            });
        });
    });
}

fn get_input_streams() -> Vec<InputStream> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ReSetDaemon",
        Duration::from_millis(1000),
    );
    let res: Result<(Vec<InputStream>,), Error> =
        proxy.method_call("org.Xetibo.ReSetAudio", "ListInputStreams", ());
    if res.is_err() {
        return Vec::new();
    }
    res.unwrap().0
}

fn get_sinks() -> Vec<Sink> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ReSetDaemon",
        Duration::from_millis(1000),
    );
    let res: Result<(Vec<Sink>,), Error> =
        proxy.method_call("org.Xetibo.ReSetAudio", "ListSinks", ());
    if res.is_err() {
        return Vec::new();
    }
    res.unwrap().0
}

fn get_default_sink_name() -> String {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ReSetDaemon",
        Duration::from_millis(1000),
    );
    let res: Result<(String,), Error> =
        proxy.method_call("org.Xetibo.ReSetAudio", "GetDefaultSinkName", ());
    if res.is_err() {
        return String::from("");
    }
    res.unwrap().0
}

fn get_default_sink() -> Sink {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ReSetDaemon",
        Duration::from_millis(1000),
    );
    let res: Result<(Sink,), Error> =
        proxy.method_call("org.Xetibo.ReSetAudio", "GetDefaultSink", ());
    if res.is_err() {
        return Sink::default();
    }
    res.unwrap().0
}

fn get_cards() -> Vec<Card> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ReSetDaemon",
        Duration::from_millis(1000),
    );
    let res: Result<(Vec<Card>,), Error> =
        proxy.method_call("org.Xetibo.ReSetAudio", "ListCards", ());
    if res.is_err() {
        return Vec::new();
    }
    res.unwrap().0
}

pub fn start_output_box_listener(conn: Connection, sink_box: Arc<SinkBox>) -> Connection {
    let sink_added = SinkAdded::match_rule(
        Some(&"org.Xetibo.ReSetDaemon".into()),
        Some(&Path::from("/org/Xetibo/ReSetDaemon")),
    )
    .static_clone();
    let sink_removed = SinkRemoved::match_rule(
        Some(&"org.Xetibo.ReSetDaemon".into()),
        Some(&Path::from("/org/Xetibo/ReSetDaemon")),
    )
    .static_clone();
    let sink_changed = SinkChanged::match_rule(
        Some(&"org.Xetibo.ReSetDaemon".into()),
        Some(&Path::from("/org/Xetibo/ReSetDaemon")),
    )
    .static_clone();
    let input_stream_added = InputStreamAdded::match_rule(
        Some(&"org.Xetibo.ReSetDaemon".into()),
        Some(&Path::from("/org/Xetibo/ReSetDaemon")),
    )
    .static_clone();
    let input_stream_removed = InputStreamRemoved::match_rule(
        Some(&"org.Xetibo.ReSetDaemon".into()),
        Some(&Path::from("/org/Xetibo/ReSetDaemon")),
    )
    .static_clone();
    let input_stream_changed = InputStreamChanged::match_rule(
        Some(&"org.Xetibo.ReSetDaemon".into()),
        Some(&Path::from("/org/Xetibo/ReSetDaemon")),
    )
    .static_clone();

    let sink_added_box = sink_box.clone();
    let sink_removed_box = sink_box.clone();
    let sink_changed_box = sink_box.clone();
    let input_stream_added_box = sink_box.clone();
    let input_stream_removed_box = sink_box.clone();
    let input_stream_changed_box = sink_box.clone();

    let res = conn.add_match(sink_added, move |ir: SinkAdded, _, _| {
        let sink_box = sink_added_box.clone();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box = sink_box.clone();
                let output_box_imp = output_box.imp();
                let mut list = output_box_imp.reset_sink_list.write().unwrap();
                let sink_index = ir.sink.index;
                let alias = ir.sink.alias.clone();
                let name = ir.sink.name.clone();
                let mut is_default = false;
                if output_box_imp.reset_default_sink.borrow().name == ir.sink.name {
                    is_default = true;
                }
                let sink_entry = Arc::new(SinkEntry::new(
                    is_default,
                    output_box_imp.reset_default_check_button.clone(),
                    ir.sink,
                ));
                let sink_clone = sink_entry.clone();
                let entry = Arc::new(ListEntry::new(&*sink_entry));
                entry.set_activatable(false);
                list.insert(sink_index, (entry.clone(), sink_clone, alias.clone()));
                output_box_imp.reset_sinks.append(&*entry);
                let mut map = output_box_imp.reset_sink_map.write().unwrap();
                let mut index = output_box_imp.reset_model_index.write().unwrap();
                output_box_imp
                    .reset_model_list
                    .write()
                    .unwrap()
                    .append(&alias);
                map.insert(alias, (sink_index, *index, name));
                *index += 1;
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on sink add");
        return conn;
    }

    let res = conn.add_match(sink_removed, move |ir: SinkRemoved, _, _| {
        let sink_box = sink_removed_box.clone();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box = sink_box.clone();
                let output_box_imp = output_box.imp();
                let mut list = output_box_imp.reset_sink_list.write().unwrap();
                let entry = list.get(&ir.index);
                if entry.is_none() {
                    return;
                }
                output_box_imp.reset_sinks.remove(&*entry.unwrap().0);
                let alias = list.remove(&ir.index);
                if alias.is_none() {
                    return;
                }
                let mut map = output_box_imp.reset_sink_map.write().unwrap();
                let entry_index = map.remove(&alias.unwrap().2);
                if let Some(entry_index) = entry_index {
                    output_box_imp
                        .reset_model_list
                        .write()
                        .unwrap()
                        .remove(entry_index.1);
                }
                let mut index = output_box_imp.reset_model_index.write().unwrap();
                if *index != 0 {
                    *index -= 1;
                }
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on sink remove");
        return conn;
    }

    let res = conn.add_match(sink_changed, move |ir: SinkChanged, _, _| {
        let sink_box = sink_changed_box.clone();
        let default_sink = get_default_sink_name();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box = sink_box.clone();
                let output_box_imp = output_box.imp();
                let is_default = ir.sink.name == default_sink;
                let volume = ir.sink.volume.first().unwrap_or(&0_u32);
                let fraction = (*volume as f64 / 655.36).round();
                let percentage = (fraction).to_string() + "%";

                let list = output_box_imp.reset_sink_list.read().unwrap();
                let entry = list.get(&ir.sink.index);
                if entry.is_none() {
                    return;
                }
                let imp = entry.unwrap().1.imp();
                if is_default {
                    output_box_imp.reset_volume_percentage.set_text(&percentage);
                    output_box_imp.reset_volume_slider.set_value(*volume as f64);
                    output_box_imp.reset_default_sink.replace(ir.sink.clone());
                    if ir.sink.muted {
                        output_box_imp
                            .reset_sink_mute
                            .set_icon_name("audio-volume-muted-symbolic");
                    } else {
                        output_box_imp
                            .reset_sink_mute
                            .set_icon_name("audio-volume-high-symbolic");
                    }
                    imp.reset_selected_sink.set_active(true);
                } else {
                    imp.reset_selected_sink.set_active(false);
                }
                imp.reset_sink_name
                    .set_title(ir.sink.alias.clone().as_str());
                imp.reset_volume_percentage.set_text(&percentage);
                imp.reset_volume_slider.set_value(*volume as f64);
                if ir.sink.muted {
                    imp.reset_sink_mute
                        .set_icon_name("audio-volume-muted-symbolic");
                } else {
                    imp.reset_sink_mute
                        .set_icon_name("audio-volume-high-symbolic");
                }
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on sink change");
        return conn;
    }

    let res = conn.add_match(input_stream_added, move |ir: InputStreamAdded, _, _| {
        let sink_box = input_stream_added_box.clone();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box = sink_box.clone();
                let output_box_imp = output_box.imp();
                let mut list = output_box_imp.reset_input_stream_list.write().unwrap();
                let index = ir.stream.index;
                let input_stream = Arc::new(InputStreamEntry::new(output_box.clone(), ir.stream));
                let entry = Arc::new(ListEntry::new(&*input_stream));
                entry.set_activatable(false);
                list.insert(index, (entry.clone(), input_stream.clone()));
                output_box_imp.reset_input_streams.append(&*entry);
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on stream add");
        return conn;
    }

    let res = conn.add_match(input_stream_changed, move |ir: InputStreamChanged, _, _| {
        let imp = input_stream_changed_box.imp();
        let alias: String;
        {
            let sink_list = imp.reset_sink_list.read().unwrap();
            let alias_opt = sink_list.get(&ir.stream.sink_index);
            if let Some(alias_opt) = alias_opt {
                alias = alias_opt.2.clone();
            } else {
                alias = String::from("");
            }
        }
        let sink_box = input_stream_changed_box.clone();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box = sink_box.clone();
                let output_box_imp = output_box.imp();
                let entry: Arc<InputStreamEntry>;
                {
                    let list = output_box_imp.reset_input_stream_list.read().unwrap();
                    let entry_opt = list.get(&ir.stream.index);
                    if entry_opt.is_none() {
                        return;
                    }
                    entry = entry_opt.unwrap().1.clone();
                }
                let imp = entry.imp();
                if ir.stream.muted {
                    imp.reset_sink_mute
                        .set_icon_name("audio-volume-muted-symbolic");
                } else {
                    imp.reset_sink_mute
                        .set_icon_name("audio-volume-high-symbolic");
                }
                let name = ir.stream.application_name.clone() + ": " + ir.stream.name.as_str();
                imp.reset_sink_selection.set_title(name.as_str());
                let volume = ir.stream.volume.first().unwrap_or(&0_u32);
                let fraction = (*volume as f64 / 655.36).round();
                let percentage = (fraction).to_string() + "%";
                imp.reset_volume_percentage.set_text(&percentage);
                imp.reset_volume_slider.set_value(*volume as f64);
                let map = output_box_imp.reset_sink_map.read().unwrap();
                let index = map.get(&alias);
                if let Some(index) = index {
                    imp.reset_sink_selection.set_selected(index.1);
                }
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on stream change");
        return conn;
    }

    let res = conn.add_match(input_stream_removed, move |ir: InputStreamRemoved, _, _| {
        let sink_box = input_stream_removed_box.clone();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box = sink_box.clone();
                let output_box_imp = output_box.imp();
                let mut list = output_box_imp.reset_input_stream_list.write().unwrap();
                let entry = list.get(&ir.index);
                if entry.is_none() {
                    return;
                }
                output_box_imp
                    .reset_input_streams
                    .remove(&*entry.unwrap().0);
                list.remove(&ir.index);
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on stream remove");
        return conn;
    }

    conn
}
