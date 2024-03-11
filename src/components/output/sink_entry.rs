use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::components::base::error_impl::show_error;
use crate::components::utils::set_action_row_ellipsis;
use adw::glib::Object;
use adw::prelude::{ButtonExt, CheckButtonExt, PreferencesRowExt, RangeExt};
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::types::ObjectSubclassIsExt;
use glib::{clone, Propagation};
use gtk::{gio, CheckButton};
use re_set_lib::audio::audio_structures::Sink;

use crate::components::utils::{AUDIO, BASE, DBUS_PATH};

use super::sink_box::{refresh_default_sink, SinkBox};
use super::sink_entry_impl;

glib::wrapper! {
    pub struct SinkEntry(ObjectSubclass<sink_entry_impl::SinkEntry>)
    @extends adw::PreferencesGroup, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for SinkEntry {}
unsafe impl Sync for SinkEntry {}

impl SinkEntry {
    pub fn new(
        is_default: bool,
        check_group: Arc<CheckButton>,
        stream: Sink,
        output_box: Arc<SinkBox>,
    ) -> Self {
        let obj: Self = Object::builder().build();
        // TODO use event callback for progress bar -> this is the "im speaking" indicator
        {
            let imp = obj.imp();
            imp.reset_sink_name.set_title(stream.alias.clone().as_str());
            let name = Arc::new(stream.name.clone());
            let volume = stream.volume.first().unwrap_or(&0_u32);
            let fraction = (*volume as f64 / 655.36).round();
            let percentage = (fraction).to_string() + "%";
            let output_box_slider = output_box.clone();
            let output_box_ref = output_box.clone();
            imp.reset_volume_percentage.set_text(&percentage);
            imp.reset_volume_slider.set_value(*volume as f64);
            imp.stream.replace(stream);
            imp.reset_volume_slider.connect_change_value(
                clone!(@weak imp => @default-return Propagation::Stop, move |_, _, value| {
                    let fraction = (value / 655.36).round();
                    let percentage = (fraction).to_string() + "%";
                    imp.reset_volume_percentage.set_text(&percentage);
                     let sink = imp.stream.borrow();
                     let index = sink.index;
                     let channels = sink.channels;
                    {
                        let mut time = imp.volume_time_stamp.borrow_mut();
                        if time.is_some() && time.unwrap().elapsed().unwrap() < Duration::from_millis(50) {
                            return Propagation::Proceed;
                        }
                        *time = Some(SystemTime::now());
                    }
                     set_sink_volume(value, index, channels, output_box_slider.clone());
                    Propagation::Proceed
                }),
            );
            imp.reset_selected_sink.set_group(Some(&*check_group));
            if is_default {
                imp.reset_selected_sink.set_active(true);
            } else {
                imp.reset_selected_sink.set_active(false);
            }
            imp.reset_selected_sink.connect_toggled(move |button| {
                let output_box_ref = output_box.clone();
                if button.is_active() {
                    let name = name.clone();
                    gio::spawn_blocking(move || {
                        let result = set_default_sink(name, output_box_ref.clone());
                        if result.is_none() {
                            return;
                        }
                        refresh_default_sink(result.unwrap(), output_box_ref, true);
                    });
                }
            });
            imp.reset_sink_mute
                .connect_clicked(clone!(@weak imp => move |_| {
                    let stream = imp.stream.clone();
                    let mut stream = stream.borrow_mut();
                    stream.muted = !stream.muted;
                    if stream.muted {
                        imp.reset_sink_mute
                           .set_icon_name("audio-volume-muted-symbolic");
                    } else {
                        imp.reset_sink_mute
                           .set_icon_name("audio-volume-high-symbolic");
                    }
                    toggle_sink_mute(stream.index, stream.muted, output_box_ref.clone());
                }));
            set_action_row_ellipsis(imp.reset_sink_name.get());
        }
        obj
    }
}

pub fn set_sink_volume(value: f64, index: u32, channels: u16, output_box: Arc<SinkBox>) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
        let res: Result<(), Error> =
            proxy.method_call(AUDIO, "SetSinkVolume", (index, channels, value as u32));
        if res.is_err() {
            show_error::<SinkBox>(output_box, "Failed to set sink volume")
        }
    });
    true
}

pub fn toggle_sink_mute(index: u32, muted: bool, output_box: Arc<SinkBox>) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
        let res: Result<(), Error> = proxy.method_call(AUDIO, "SetSinkMute", (index, muted));
        if res.is_err() {
            show_error::<SinkBox>(output_box, "Failed to mute sink")
        }
    });
    true
}

pub fn set_default_sink(name: Arc<String>, output_box: Arc<SinkBox>) -> Option<Sink> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Sink,), Error> = proxy.method_call(AUDIO, "SetDefaultSink", (name.as_str(),));
    if res.is_err() {
        show_error::<SinkBox>(output_box, "Failed to set default sink");
        return None;
    }
    Some(res.unwrap().0)
}
