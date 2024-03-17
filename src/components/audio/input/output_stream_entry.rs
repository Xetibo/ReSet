use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::components::base::error_impl::show_error;
use crate::components::utils::{
    create_dropdown_label_factory, set_combo_row_ellipsis, AUDIO, BASE, DBUS_PATH,
};
use adw::glib::Object;
use adw::prelude::{ButtonExt, ComboRowExt, PreferencesRowExt, RangeExt};
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::types::ObjectSubclassIsExt;
use glib::{clone, Propagation};
use glib::prelude::Cast;
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
        let output_box_volume_ref = source_box.clone();
        let output_box_mute_ref = source_box.clone();
        let output_box_source_ref = source_box.clone();
        {
            let index = stream.index;
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
                    set_outputstream_volume(value, index, channels, output_box_volume_ref.clone());
                    Propagation::Proceed
                }),
            );
            {
                let list = box_imp.reset_model_list.read().unwrap();
                imp.reset_source_selection.set_model(Some(&*list));
                let source_list = box_imp.reset_source_list.read().unwrap();
                let name = source_list.get(&index);
                let index = box_imp.reset_model_index.read().unwrap();
                let model_list = box_imp.reset_model_list.read().unwrap();
                if let Some(name) = name {
                    for entry in 0..*index {
                        if model_list.string(entry) == Some(name.2.clone().into()) {
                            imp.reset_source_selection.set_selected(entry);
                            break;
                        }
                    }
                } else {
                    let mut name = box_imp.reset_default_source.try_borrow();
                    while name.is_err() {
                        name = box_imp.reset_default_source.try_borrow();
                    }
                    let name = name.unwrap();
                    for entry in 0..*index {
                        if model_list.string(entry) == Some(name.alias.clone().into()) {
                            imp.reset_source_selection.set_selected(entry);
                            break;
                        }
                    }
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
                    set_source_of_output_stream(stream.index, source, output_box_source_ref.clone());
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
                    toggle_output_stream_mute(index, muted, output_box_mute_ref.clone());
                }));
        }
        obj
    }
}

fn set_outputstream_volume(
    value: f64,
    index: u32,
    channels: u16,
    input_box: Arc<SourceBox>,
) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
        let res: Result<(), Error> = proxy.method_call(
            AUDIO,
            "SetOutputStreamVolume",
            (index, channels, value as u32),
        );
        if res.is_err() {
            show_error::<SourceBox>(input_box.clone(), "Failed to set output stream volume");
        }
    });
    true
}

fn toggle_output_stream_mute(index: u32, muted: bool, input_box: Arc<SourceBox>) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
        let res: Result<(), Error> =
            proxy.method_call(AUDIO, "SetOutputStreamMute", (index, muted));
        if res.is_err() {
            show_error::<SourceBox>(input_box.clone(), "Failed to mute output stream");
        }
    });
    true
}

fn set_source_of_output_stream(stream: u32, source: u32, input_box: Arc<SourceBox>) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
        let res: Result<(bool,), Error> =
            proxy.method_call(AUDIO, "SetSourceOfOutputStream", (stream, source));
        if res.is_err() {
            show_error::<SourceBox>(input_box.clone(), "Failed to set source of output stream");
        }
    });
    true
}

// TODO propagate error from dbus
