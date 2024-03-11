use adw::prelude::PreferencesGroupExt;
use adw::ComboRow;
use re_set_lib::audio::audio_structures::Card;
use re_set_lib::audio::audio_structures::InputStream;
use re_set_lib::audio::audio_structures::Sink;
use re_set_lib::signals::InputStreamAdded;
use re_set_lib::signals::InputStreamChanged;
use re_set_lib::signals::InputStreamRemoved;
use re_set_lib::signals::SinkAdded;
use re_set_lib::signals::SinkChanged;
use re_set_lib::signals::SinkRemoved;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use adw::glib::Object;
use adw::prelude::ListBoxRowExt;
use adw::prelude::{BoxExt, ButtonExt, CheckButtonExt, ComboRowExt, RangeExt};
use dbus::blocking::Connection;
use dbus::message::SignalArgs;
use dbus::{Error, Path};
use glib::subclass::prelude::ObjectSubclassIsExt;
use glib::{Cast, Propagation, Variant};
use gtk::prelude::ActionableExt;
use gtk::{gio, StringObject};

use crate::components::base::card_entry::CardEntry;
use crate::components::base::error_impl::show_error;
use crate::components::base::error_impl::ReSetErrorImpl;
use crate::components::base::list_entry::ListEntry;
use crate::components::output::sink_entry::set_sink_volume;
use crate::components::utils::AUDIO;
use crate::components::utils::BASE;
use crate::components::utils::DBUS_PATH;
use crate::components::utils::{create_dropdown_label_factory, set_combo_row_ellipsis};

use super::input_stream_entry::InputStreamEntry;
use super::sink_box_event_handlers::input_stream_added_handler;
use super::sink_box_event_handlers::input_stream_changed_handler;
use super::sink_box_event_handlers::input_stream_removed_handler;
use super::sink_box_event_handlers::sink_added_handler;
use super::sink_box_event_handlers::sink_changed_handler;
use super::sink_box_event_handlers::sink_removed_handler;
use super::sink_box_impl;
use super::sink_entry::{set_default_sink, toggle_sink_mute, SinkEntry};

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
        let sinks = get_sinks(output_box.clone());
        {
            let output_box_imp = output_box.imp();
            let list = output_box_imp.reset_model_list.write().unwrap();
            let mut map = output_box_imp.reset_sink_map.write().unwrap();
            let mut model_index = output_box_imp.reset_model_index.write().unwrap();
            output_box_imp
                .reset_default_sink
                .replace(get_default_sink(output_box.clone()));
            for sink in sinks.iter() {
                list.append(&sink.alias);
                map.insert(sink.alias.clone(), (sink.index, sink.name.clone()));
                *model_index += 1;
            }
        }
        populate_inputstreams(output_box.clone());
        populate_cards(output_box.clone());
        populate_sink_information(output_box, sinks);
    });
}

fn drop_down_handler(output_box: Arc<SinkBox>, dropdown: &ComboRow) {
    let output_box_ref = output_box.clone();
    let output_box_imp = output_box.imp();
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
    let new_sink_name = Arc::new(sink.unwrap().1.clone());
    gio::spawn_blocking(move || {
        let result = set_default_sink(new_sink_name, output_box_ref.clone());
        if result.is_none() {
            return;
        }
        let new_sink = result.unwrap();
        refresh_default_sink(new_sink, output_box_ref, false);
    });
}

fn volume_slider_handler(output_box: Arc<SinkBox>, value: f64) -> glib::Propagation {
    let imp = output_box.imp();
    let fraction = (value / 655.36).round();
    let percentage = (fraction).to_string() + "%";
    imp.reset_volume_percentage.set_text(&percentage);
    let sink = imp.reset_default_sink.borrow();
    let index = sink.index;
    let channels = sink.channels;
    {
        let mut time = imp.volume_time_stamp.borrow_mut();
        if time.is_some() && time.unwrap().elapsed().unwrap() < Duration::from_millis(50) {
            return Propagation::Proceed;
        }
        *time = Some(SystemTime::now());
    }
    set_sink_volume(value, index, channels, output_box.clone());
    Propagation::Proceed
}

fn mute_handler(output_box: Arc<SinkBox>) {
    let imp = output_box.imp();
    let mut stream = imp.reset_default_sink.borrow_mut();
    stream.muted = !stream.muted;
    if stream.muted {
        imp.reset_sink_mute
            .set_icon_name("audio-volume-muted-symbolic");
    } else {
        imp.reset_sink_mute
            .set_icon_name("audio-volume-high-symbolic");
    }
    toggle_sink_mute(stream.index, stream.muted, output_box.clone());
}

fn populate_sink_information(output_box: Arc<SinkBox>, sinks: Vec<Sink>) {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let output_box_ref_select = output_box.clone();
            let output_box_ref_slider = output_box.clone();
            let output_box_ref_mute = output_box.clone();
            let output_box_ref = output_box.clone();
            {
                let output_box_imp = output_box_ref.imp();
                let default_sink = output_box_imp.reset_default_sink.clone();
                let sink = default_sink.borrow();

                if sink.muted {
                    output_box_imp
                        .reset_sink_mute
                        .set_icon_name("audio-volume-muted-symbolic");
                } else {
                    output_box_imp
                        .reset_sink_mute
                        .set_icon_name("audio-volume-high-symbolic");
                }

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
                        output_box.clone(),
                    ));
                    let sink_clone = sink_entry.clone();
                    let entry = Arc::new(ListEntry::new(&*sink_entry));
                    entry.set_activatable(false);
                    list.insert(index, (entry.clone(), sink_clone, alias));
                    output_box_imp.reset_sinks.append(&*entry);
                }
                let list = output_box_imp.reset_model_list.read().unwrap();
                output_box_imp.reset_sink_dropdown.set_model(Some(&*list));
                let name = output_box_imp.reset_default_sink.borrow();

                let index = output_box_imp.reset_model_index.read().unwrap();
                let model_list = output_box_imp.reset_model_list.read().unwrap();
                for entry in 0..*index {
                    if model_list.string(entry) == Some(name.alias.clone().into()) {
                        output_box_imp.reset_sink_dropdown.set_selected(entry);
                        break;
                    }
                }
                output_box_imp
                    .reset_sink_dropdown
                    .connect_selected_notify(move |dropdown| {
                        drop_down_handler(output_box_ref_select.clone(), dropdown);
                    });
            }
            output_box_ref
                .imp()
                .reset_volume_slider
                .connect_change_value(move |_, _, value| {
                    volume_slider_handler(output_box_ref_slider.clone(), value)
                });
            output_box_ref
                .imp()
                .reset_sink_mute
                .connect_clicked(move |_| {
                    mute_handler(output_box_ref_mute.clone());
                });
        });
    });
}

pub fn refresh_default_sink(new_sink: Sink, output_box: Arc<SinkBox>, entry: bool) {
    let volume = *new_sink.volume.first().unwrap_or(&0_u32);
    let fraction = (volume as f64 / 655.36).round();
    let percentage = (fraction).to_string() + "%";
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let imp = output_box.imp();
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

pub fn populate_inputstreams(output_box: Arc<SinkBox>) {
    let output_box_ref = output_box.clone();

    gio::spawn_blocking(move || {
        let streams = get_input_streams(output_box.clone());
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
        let cards = get_cards(output_box.clone());
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

fn get_input_streams(output_box: Arc<SinkBox>) -> Vec<InputStream> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Vec<InputStream>,), Error> = proxy.method_call(AUDIO, "ListInputStreams", ());
    if res.is_err() {
        show_error::<SinkBox>(output_box.clone(), "Failed to list input streams");
        return Vec::new();
    }
    res.unwrap().0
}

fn get_sinks(output_box: Arc<SinkBox>) -> Vec<Sink> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Vec<Sink>,), Error> = proxy.method_call(AUDIO, "ListSinks", ());
    if res.is_err() {
        show_error::<SinkBox>(output_box.clone(), "Failed to list sinks");
        return Vec::new();
    }
    res.unwrap().0
}

fn get_cards(output_box: Arc<SinkBox>) -> Vec<Card> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Vec<Card>,), Error> = proxy.method_call(AUDIO, "ListCards", ());
    if res.is_err() {
        show_error::<SinkBox>(output_box.clone(), "Failed to list profiles");
        return Vec::new();
    }
    res.unwrap().0
}

pub fn get_default_sink_name(output_box: Arc<SinkBox>) -> String {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(String,), Error> = proxy.method_call(AUDIO, "GetDefaultSinkName", ());
    if res.is_err() {
        show_error::<SinkBox>(output_box.clone(), "Failed to get default sink name");
        return String::from("");
    }
    res.unwrap().0
}

fn get_default_sink(output_box: Arc<SinkBox>) -> Sink {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Sink,), Error> = proxy.method_call(AUDIO, "GetDefaultSink", ());
    if res.is_err() {
        show_error::<SinkBox>(output_box.clone(), "Failed to get default sink");
        return Sink::default();
    }
    res.unwrap().0
}

pub fn start_output_box_listener(conn: Connection, sink_box: Arc<SinkBox>) -> Connection {
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
