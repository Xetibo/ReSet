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
use re_set_lib::audio::audio_structures::OutputStream;

use super::output_stream_entry_impl;
use super::source_box::SourceBox;

glib::wrapper! {
    pub struct OutputStreamEntry(ObjectSubclass<output_stream_entry_impl::OutputStreamEntry>)
    @extends adw::PreferencesGroup, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for OutputStreamEntry {}
unsafe impl Sync for OutputStreamEntry {}

impl OutputStreamEntry {
    pub fn new(source_box: Arc<SourceBox>, stream: OutputStream) -> Self {
        let obj: Self = Object::builder().build();
        // TODO use event callback for progress bar -> this is the "im speaking" indicator
        {
            let box_imp = source_box.imp();
            let imp = obj.imp();
            let name = stream.application_name.clone() + ": " + stream.name.as_str();
            imp.reset_source_selection.set_title(name.as_str());
            imp.reset_source_selection
                .set_factory(Some(&create_dropdown_label_factory()));
            set_combo_row_ellipsis(imp.reset_source_selection.get());
            let volume = stream.volume.first().unwrap_or(&0_u32);
            let fraction = (*volume as f64 / 655.36).round();
            let percentage = (fraction).to_string() + "%";
            imp.reset_volume_percentage.set_text(&percentage);
            imp.reset_volume_slider.set_value(*volume as f64);
            imp.stream.replace(stream);
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
                        if time.is_some()
                            && time.unwrap().elapsed().unwrap() < Duration::from_millis(50)
                        {
                            return Propagation::Proceed;
                        }
                        *time = Some(SystemTime::now());
                    }
                    set_outputstream_volume(value, index, channels);
                    Propagation::Proceed
                }),
            );
            {
                let list = box_imp.reset_model_list.read().unwrap();
                imp.reset_source_selection.set_model(Some(&*list));
                let map = box_imp.reset_source_map.write().unwrap();
                let mut name = box_imp.reset_default_source.try_borrow();
                while name.is_err() {
                    name = box_imp.reset_default_source.try_borrow();
                }
                let name = name.unwrap();
                let name = &name.alias;
                let index = map.get(name);
                if let Some(index) = index {
                    imp.reset_source_selection.set_selected(index.1);
                }
            }
            imp.reset_source_selection.connect_selected_notify(
                clone!(@weak imp, @weak box_imp => move |dropdown| {
                    let selected = dropdown.selected_item();
                    if selected.is_none() {
                        return;
                    }
                    let selected = selected.unwrap();
                    let selected = selected.downcast_ref::<StringObject>().unwrap();
                    let selected = selected.string().to_string();
                    let source = box_imp.reset_source_map.write().unwrap();
                    let source = source.get(&selected);
                    if source.is_none() {
                        return;
                    }
                    let mut stream = imp.stream.try_borrow();
                    while stream.is_err() {
                        stream = imp.stream.try_borrow();
                    }
                    let stream = stream.unwrap();
                    let source = source.unwrap().0;
                    set_source_of_output_stream(stream.index, source);
                }),
            );
            imp.reset_source_mute
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
                        imp.reset_source_mute
                           .set_icon_name("microphone-disabled-symbolic");
                    } else {
                        imp.reset_source_mute
                           .set_icon_name("audio-input-microphone-symbolic");
                    }
                    toggle_output_stream_mute(index, muted);
                }));
        }
        obj
    }
}

fn set_outputstream_volume(value: f64, index: u32, channels: u16) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSetDaemon",
            "/org/Xetibo/ReSetDaemon",
            Duration::from_millis(1000),
        );
        let _: Result<(), Error> = proxy.method_call(
            "org.Xetibo.ReSetAudio",
            "SetOutputStreamVolume",
            (index, channels, value as u32),
        );
        // if res.is_err() {
        //     return false;
        // }
        // res.unwrap().0
    });
    true
}

fn toggle_output_stream_mute(index: u32, muted: bool) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSetDaemon",
            "/org/Xetibo/ReSetDaemon",
            Duration::from_millis(1000),
        );
        let _: Result<(), Error> = proxy.method_call(
            "org.Xetibo.ReSetAudio",
            "SetOutputStreamMute",
            (index, muted),
        );
        // if res.is_err() {
        //     return false;
        // }
        // res.unwrap().0
    });
    true
}

fn set_source_of_output_stream(stream: u32, source: u32) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSetDaemon",
            "/org/Xetibo/ReSetDaemon",
            Duration::from_millis(1000),
        );
        let _: Result<(bool,), Error> = proxy.method_call(
            "org.Xetibo.ReSetAudio",
            "SetSourceOfOutputStream",
            (stream, source),
        );
        // if res.is_err() {
        //     return false;
        // }
        // res.unwrap().0
    });
    true
}

// TODO propagate error from dbus
