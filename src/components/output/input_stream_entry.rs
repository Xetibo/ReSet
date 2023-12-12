use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::components::utils::{create_dropdown_label_factory, set_combo_row_ellipsis};
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
                    set_inputstream_volume(value, index, channels);
                    Propagation::Proceed
                }),
            );
            {
                let list = box_imp.reset_model_list.read().unwrap();
                // while list.is_err() {
                //     list = box_imp.resetModelList.try_borrow();
                // }
                // let list = list.unwrap();
                imp.reset_sink_selection.set_model(Some(&*list));
                let map = box_imp.reset_sink_map.read().unwrap();
                let sink_list = box_imp.reset_sink_list.read().unwrap();
                let name = sink_list.get(&index);
                if let Some(name) = name {
                    let name = &name.2;
                    let index = map.get(name);
                    if let Some(index) = index {
                        imp.reset_sink_selection.set_selected(index.1);
                    }
                } else {
                    let mut name = box_imp.reset_default_sink.try_borrow();
                    while name.is_err() {
                        name = box_imp.reset_default_sink.try_borrow();
                    }
                    let name = &name.unwrap().alias;
                    let index = map.get(name);
                    if let Some(index) = index {
                        imp.reset_sink_selection.set_selected(index.1);
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
                    set_sink_of_input_stream(stream.index, sink);
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
                    toggle_input_stream_mute(index, muted);
                }));
        }
        obj
    }
}

fn set_inputstream_volume(value: f64, index: u32, channels: u16) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSet.Daemon",
            "/org/Xetibo/ReSet/Daemon",
            Duration::from_millis(1000),
        );
        let _: Result<(), Error> = proxy.method_call(
            "org.Xetibo.ReSet.Audio",
            "SetInputStreamVolume",
            (index, channels, value as u32),
        );
        // if res.is_err() {
        //     return false;
        // }
        // res.unwrap().0
    });
    true
}

fn toggle_input_stream_mute(index: u32, muted: bool) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSet.Daemon",
            "/org/Xetibo/ReSet/Daemon",
            Duration::from_millis(1000),
        );
        let _: Result<(), Error> = proxy.method_call(
            "org.Xetibo.ReSet.Audio",
            "SetInputStreamMute",
            (index, muted),
        );
        // if res.is_err() {
        //     return false;
        // }
        // res.unwrap().0
    });
    true
}

fn set_sink_of_input_stream(stream: u32, sink: u32) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSet.Daemon",
            "/org/Xetibo/ReSet/Daemon",
            Duration::from_millis(1000),
        );
        let _: Result<(), Error> = proxy.method_call(
            "org.Xetibo.ReSet.Audio",
            "SetSinkOfInputStream",
            (stream, sink),
        );
        // if res.is_err() {
        //     return false;
        // }
        // res.unwrap().0
    });
    true
}

// TODO propagate error from dbus
