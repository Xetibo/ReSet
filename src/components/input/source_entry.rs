use std::sync::Arc;
use std::time::{Duration, SystemTime};

use adw::glib;
use adw::glib::Object;
use adw::prelude::{ButtonExt, CheckButtonExt, PreferencesRowExt, RangeExt};
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::types::ObjectSubclassIsExt;
use glib::{clone, Propagation};
use gtk::{gio, CheckButton};
use ReSet_Lib::audio::audio::Source;

use super::source_entry_impl;

glib::wrapper! {
    pub struct SourceEntry(ObjectSubclass<source_entry_impl::SourceEntry>)
    @extends adw::PreferencesGroup, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for SourceEntry {}
unsafe impl Sync for SourceEntry {}

impl SourceEntry {
    pub fn new(is_default: bool, check_group: Arc<CheckButton>, stream: Source) -> Self {
        let obj: Self = Object::builder().build();
        // TODO use event callback for progress bar -> this is the "im speaking" indicator
        {
            let imp = obj.imp();
            imp.reset_source_name.set_title(stream.alias.clone().as_str());
            let name = Arc::new(stream.name.clone());
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
                    let source = imp.stream.borrow();
                    let index = source.index;
                    let channels = source.channels;
                    {
                        let mut time = imp.volume_time_stamp.borrow_mut();
                        if time.is_some()
                            && time.unwrap().elapsed().unwrap() < Duration::from_millis(50)
                        {
                            return Propagation::Proceed;
                        }
                        *time = Some(SystemTime::now());
                    }
                    set_source_volume(value, index, channels);
                    Propagation::Proceed
                }),
            );
            imp.reset_selected_source.set_group(Some(&*check_group));
            // check_group.set_group(Some(&*imp.resetSelectedSink));
            if is_default {
                imp.reset_selected_source.set_active(true);
            } else {
                imp.reset_selected_source.set_active(false);
            }
            imp.reset_selected_source.connect_toggled(move |button| {
                if button.is_active() {
                    set_default_source(name.clone());
                }
            });
            imp.reset_source_mute
                .connect_clicked(clone!(@weak imp => move |_| {
                    let stream = imp.stream.clone();
                    let mut stream = stream.borrow_mut();
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
                    toggle_source_mute(index, muted);
                }));
        }
        obj
    }
}

pub fn set_source_volume(value: f64, index: u32, channels: u16) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSetDaemon",
            "/org/Xetibo/ReSetDaemon",
            Duration::from_millis(1000),
        );
        let _: Result<(), Error> = proxy.method_call(
            "org.Xetibo.ReSetAudio",
            "SetSourceVolume",
            (index, channels, value as u32),
        );
        // if res.is_err() {
        //     return false;
        // }
        // res.unwrap().0
    });
    true
}

pub fn toggle_source_mute(index: u32, muted: bool) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSetDaemon",
            "/org/Xetibo/ReSetDaemon",
            Duration::from_millis(1000),
        );
        let _: Result<(), Error> =
            proxy.method_call("org.Xetibo.ReSetAudio", "SetSourceMute", (index, muted));
        // if res.is_err() {
        //     return false;
        // }
        // res.unwrap().0
    });
    true
}

pub fn set_default_source(name: Arc<String>) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSetDaemon",
            "/org/Xetibo/ReSetDaemon",
            Duration::from_millis(1000),
        );
        let _: Result<(), Error> =
            proxy.method_call("org.Xetibo.ReSetAudio", "SetDefaultSink", (name.as_str(),));
        // if res.is_err() {
        //     return;
        // }
        // handle change
    });
    true
}

// TODO propagate error from dbus
