use std::sync::Arc;
use std::time::{Duration, SystemTime};

use adw::glib;
use adw::glib::Object;
use adw::prelude::{ButtonExt, CheckButtonExt, RangeExt};
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::types::ObjectSubclassIsExt;
use glib::{clone, Propagation};
use gtk::{gio, CheckButton};
use ReSet_Lib::audio::audio::Sink;

use super::sinkEntryImpl;

glib::wrapper! {
    pub struct SinkEntry(ObjectSubclass<sinkEntryImpl::SinkEntry>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl SinkEntry {
    pub fn new(is_default: bool, check_group: Arc<CheckButton>, stream: Sink) -> Self {
        let obj: Self = Object::builder().build();
        // TODO use event callback for progress bar -> this is the "im speaking" indicator
        // TODO handle events
        {
            let imp = obj.imp();
            imp.resetSinkName.set_text(stream.alias.clone().as_str());
            let name = Arc::new(stream.name.clone());
            let volume = stream.volume.first().unwrap_or(&0_u32);
            let fraction = (*volume as f64 / 655.36).round();
            let percentage = (fraction).to_string() + "%";
            imp.resetVolumePercentage.set_text(&percentage);
            imp.resetVolumeSlider.set_value(*volume as f64);
            imp.stream.replace(stream);
            imp.resetVolumeSlider.connect_change_value(
                clone!(@weak imp => @default-return Propagation::Stop, move |_, _, value| {
                    let fraction = (value / 655.36).round();
                    let percentage = (fraction).to_string() + "%";
                    imp.resetVolumePercentage.set_text(&percentage);
                     let sink = imp.stream.borrow();
                     let index = sink.index;
                     let channels = sink.channels;
                    {
                        let mut time = imp.volumeTimeStamp.borrow_mut();
                        if time.is_some() && time.unwrap().elapsed().unwrap() < Duration::from_millis(50) {
                            return Propagation::Proceed;
                        }
                        *time = Some(SystemTime::now());
                    }
                     set_sink_volume(value, index, channels);
                    Propagation::Proceed
                }),
            );
            imp.resetSelectedSink.set_group(Some(&*check_group));
            if is_default {
                imp.resetSelectedSink.set_active(true);
            } else {
                imp.resetSelectedSink.set_active(false);
            }
            imp.resetSelectedSink.connect_toggled(move |button| {
                if button.is_active() {
                    set_default_sink(name.clone());
                }
            });
            imp.resetSinkMute
                .connect_clicked(clone!(@weak imp => move |_| {
                    let stream = imp.stream.clone();
                    let mut stream = stream.borrow_mut();
                    stream.muted = !stream.muted;
                    let muted = stream.muted;
                    let index = stream.index;
                    if muted {
                        imp.resetSinkMute
                           .set_icon_name("audio-volume-muted-symbolic");
                    } else {
                        imp.resetSinkMute
                           .set_icon_name("audio-volume-high-symbolic");
                    }
                    toggle_sink_mute(index, muted);
                }));
        }
        obj
    }
}

pub fn set_sink_volume(value: f64, index: u32, channels: u16) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.xetibo.ReSet",
            "/org/xetibo/ReSet",
            Duration::from_millis(1000),
        );
        let _: Result<(), Error> = proxy.method_call(
            "org.xetibo.ReSet",
            "SetSinkVolume",
            (index, channels, value as u32),
        );
        // if res.is_err() {
        //     return false;
        // }
        // res.unwrap().0
    });
    true
}

pub fn toggle_sink_mute(index: u32, muted: bool) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.xetibo.ReSet",
            "/org/xetibo/ReSet",
            Duration::from_millis(1000),
        );
        let _: Result<(), Error> =
            proxy.method_call("org.xetibo.ReSet", "SetSinkMute", (index, muted));
        // if res.is_err() {
        //     return false;
        // }
        // res.unwrap().0
    });
    true
}

pub fn set_default_sink(name: Arc<String>) {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.xetibo.ReSet",
            "/org/xetibo/ReSet",
            Duration::from_millis(1000),
        );
        let _: Result<(), Error> =
            proxy.method_call("org.xetibo.ReSet", "SetDefaultSink", (name.as_str(),));
        // if res.is_err() {
        //     return;
        // }
        // handle change
    });
}

// TODO propagate error from dbus
