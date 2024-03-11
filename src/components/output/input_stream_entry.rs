use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::components::base::error_impl::show_error;
use crate::components::utils::{
    create_dropdown_label_factory, set_combo_row_ellipsis, AUDIO, BASE, DBUS_PATH,
};
use adw::glib;
use adw::glib::Object;
use adw::prelude::{ButtonExt, ComboRowExt, PreferencesRowExt, RangeExt};
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::types::ObjectSubclassIsExt;
use glib::{clone, Cast, Propagation};
use gtk::{gio, StringObject};
use re_set_lib::audio::audio_structures::InputStream;

use super::input_stream_entry_impl;
use super::sink_box::SinkBox;

glib::wrapper! {
    pub struct InputStreamEntry(ObjectSubclass<input_stream_entry_impl::InputStreamEntry>)
    @extends adw::PreferencesGroup, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for InputStreamEntry {}
unsafe impl Sync for InputStreamEntry {}

impl InputStreamEntry {
    pub fn new(sink_box: Arc<SinkBox>, stream: InputStream) -> Self {
        let obj: Self = Object::builder().build();
        // TODO use event callback for progress bar -> this is the "im speaking" indicator
        let output_box_mute_ref = sink_box.clone();
        let output_box_volume_ref = sink_box.clone();
        let output_box_sink_ref = sink_box.clone();
        {
            let index = stream.sink_index;
            let box_imp = sink_box.imp();
            let imp = obj.imp();
            if stream.muted {
                imp.reset_sink_mute
                    .set_icon_name("audio-volume-muted-symbolic");
            } else {
                imp.reset_sink_mute
                    .set_icon_name("audio-volume-high-symbolic");
            }
            let name = stream.application_name.clone() + ": " + stream.name.as_str();
            imp.reset_sink_selection.set_title(name.as_str());
            imp.reset_sink_selection
                .set_factory(Some(&create_dropdown_label_factory()));
            set_combo_row_ellipsis(imp.reset_sink_selection.get());
            let volume = stream.volume.first().unwrap_or(&0_u32);
            let fraction = (*volume as f64 / 655.36).round();
            let percentage = (fraction).to_string() + "%";
            imp.reset_volume_percentage.set_text(&percentage);
            imp.reset_volume_slider.set_value(*volume as f64);
            imp.stream.replace(stream);
            {
                let sink = box_imp.reset_default_sink.borrow();
                imp.associated_sink.replace((sink.index, sink.name.clone()));
            }
            imp.reset_volume_slider.connect_change_value(
                clone!(@weak imp => @default-return Propagation::Stop, move |_, _, value| {
                    let fraction = (value / 655.36).round();
                    let percentage = (fraction).to_string() + "%";
                    imp.reset_volume_percentage.set_text(&percentage);
                    let mut stream = imp.stream.try_borrow();
                    while stream.is_err() {
                        stream = imp.stream.try_borrow();
                    }
                    let stream = stream.unwrap();
                    let index = stream.index;
                    let channels = stream.channels;
                    {
                        let mut time = imp.volume_time_stamp.borrow_mut();
                        if time.is_some() && time.unwrap().elapsed().unwrap() < Duration::from_millis(50) {
                            return Propagation::Proceed;
                        }
                        *time = Some(SystemTime::now());
                    }
                    set_inputstream_volume(value, index, channels, output_box_volume_ref.clone());
                    Propagation::Proceed
                }),
            );
            {
                let list = box_imp.reset_model_list.read().unwrap();
                imp.reset_sink_selection.set_model(Some(&*list));
                let sink_list = box_imp.reset_sink_list.read().unwrap();
                let name = sink_list.get(&index);
                let index = box_imp.reset_model_index.read().unwrap();
                let model_list = box_imp.reset_model_list.read().unwrap();
                if let Some(name) = name {
                    for entry in 0..*index {
                        if model_list.string(entry) == Some(name.2.clone().into()) {
                            imp.reset_sink_selection.set_selected(entry);
                            break;
                        }
                    }
                } else {
                    let mut name = box_imp.reset_default_sink.try_borrow();
                    while name.is_err() {
                        name = box_imp.reset_default_sink.try_borrow();
                    }
                    let name = name.unwrap();
                    for entry in 0..*index {
                        if model_list.string(entry) == Some(name.alias.clone().into()) {
                            imp.reset_sink_selection.set_selected(entry);
                            break;
                        }
                    }
                }
            }
            imp.reset_sink_selection.connect_selected_notify(
                clone!(@weak imp, @weak box_imp => move |dropdown| {
                    let selected = dropdown.selected_item();
                    if selected.is_none() {
                        return;
                    }
                    let selected = selected.unwrap();
                    let selected = selected.downcast_ref::<StringObject>().unwrap();
                    let selected = selected.string().to_string();
                    let sink = box_imp.reset_sink_map.read().unwrap();
                    // if sink.is_err() {
                    //     return;
                    // }
                    // let sink = sink.unwrap();
                    let sink = sink.get(&selected);
                    if sink.is_none() {
                        return;
                    }
                    let mut stream = imp.stream.try_borrow();
                    while stream.is_err() {
                        stream = imp.stream.try_borrow();
                    }
                    let stream = stream.unwrap();
                let sink = sink.unwrap().0;
                    set_sink_of_input_stream(stream.index, sink, output_box_sink_ref.clone());
                }),
            );
            imp.reset_sink_mute
                .connect_clicked(clone!(@weak imp => move |_| {
                    let stream = imp.stream.clone();
                    let mut stream = stream.try_borrow_mut();
                    while stream.is_err() {
                        stream = imp.stream.try_borrow_mut();
                    }
                    let mut stream = stream.unwrap();
                    stream.muted = !stream.muted;
                    let muted = stream.muted;
                    let index = stream.index;
                    if muted {
                        imp.reset_sink_mute
                           .set_icon_name("audio-volume-muted-symbolic");
                    } else {
                        imp.reset_sink_mute
                           .set_icon_name("audio-volume-high-symbolic");
                    }
                    toggle_input_stream_mute(index, muted, output_box_mute_ref.clone());
                }));
        }
        obj
    }
}

fn set_inputstream_volume(value: f64, index: u32, channels: u16, output_box: Arc<SinkBox>) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
        let res: Result<(), Error> = proxy.method_call(
            AUDIO,
            "SetInputStreamVolume",
            (index, channels, value as u32),
        );
        if res.is_err() {
            show_error::<SinkBox>(output_box.clone(), "Failed to set input stream volume");
        }
    });
    true
}

fn toggle_input_stream_mute(index: u32, muted: bool, output_box: Arc<SinkBox>) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
        let res: Result<(), Error> = proxy.method_call(AUDIO, "SetInputStreamMute", (index, muted));
        if res.is_err() {
            show_error::<SinkBox>(output_box.clone(), "Failed to mute input stream");
        }
    });
    true
}

fn set_sink_of_input_stream(stream: u32, sink: u32, output_box: Arc<SinkBox>) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
        let res: Result<(), Error> =
            proxy.method_call(AUDIO, "SetSinkOfInputStream", (stream, sink));
        if res.is_err() {
            show_error::<SinkBox>(output_box.clone(), "Failed to set sink of input stream");
        }
    });
    true
}
