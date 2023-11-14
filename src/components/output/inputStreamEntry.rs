use std::cell::RefCell;
use std::sync::Arc;
use std::time::Duration;

use adw::glib;
use adw::glib::Object;
use adw::prelude::{ButtonExt, RangeExt};
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::types::ObjectSubclassIsExt;
use glib::subclass::ObjectImplRef;
use glib::{clone, Propagation};
use ReSet_Lib::audio::audio::InputStream;

use super::inputStreamEntryImpl;

glib::wrapper! {
    pub struct InputStreamEntry(ObjectSubclass<inputStreamEntryImpl::InputStreamEntry>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl InputStreamEntry {
    pub fn new(stream: InputStream) -> Self {
        let obj: Self = Object::builder().build();
        // TODO use event callback for progress bar -> this is the "im speaking" indicator
        // TODO map dropdown
        {
            let imp = obj.imp();
            if stream.muted {
                imp.resetSinkMute
                    .set_icon_name("audio-volume-muted-symbolic");
            } else {
                imp.resetSinkMute
                    .set_icon_name("audio-volume-high-symbolic");
            }
            let name = stream.application_name.clone() + ": " + stream.name.as_str();
            imp.resetSinkName.set_text(name.as_str());
            let volume = stream.volume.first().unwrap_or_else(|| &(0 as u32));
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
                    let stream = imp.stream.borrow();
                    let index = stream.index;
                    let channels = stream.channels;
                    set_inputstream_volume(value, index, channels);
                    Propagation::Proceed
                }),
            );
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
                    toggle_input_stream_mute(index, muted);
                }));
        }
        obj
    }
}

fn set_inputstream_volume(value: f64, index: u32, channels: u16) -> bool {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(bool,), Error> = proxy.method_call(
        "org.xetibo.ReSet",
        "SetInputStreamVolume",
        (index, channels, value as u32),
    );
    if res.is_err() {
        return false;
    }
    res.unwrap().0
}

fn toggle_input_stream_mute(index: u32, muted: bool) -> bool {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(bool,), Error> =
        proxy.method_call("org.xetibo.ReSet", "SetInputStreamMute", (index, muted));
    if res.is_err() {
        return false;
    }
    res.unwrap().0
}
