use std::sync::Arc;
use std::thread;
use std::time::Duration;

use adw::glib;
use adw::glib::Object;
use adw::prelude::{ButtonExt, RangeExt, CheckButtonExt};
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::types::ObjectSubclassIsExt;
use glib::{clone, Propagation};
use ReSet_Lib::audio::audio::Source;
use gtk::CheckButton;

use super::sourceEntryImpl;

glib::wrapper! {
    pub struct SourceEntry(ObjectSubclass<sourceEntryImpl::SourceEntry>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl SourceEntry {
    pub fn new(is_default: bool, check_group: Arc<CheckButton>, stream: Source) -> Self {
        let obj: Self = Object::builder().build();
        // TODO use event callback for progress bar -> this is the "im speaking" indicator
        // TODO handle events
        {
            let imp = obj.imp();
            imp.resetSourceName.set_text(stream.alias.clone().as_str());
            let name = Arc::new(stream.name.clone());
            let volume = stream.volume.first().unwrap_or_else(|| &(0 as u32));
            let fraction = (*volume as f64 / 655.36).round();
            let percentage = (fraction).to_string() + "%";
            imp.resetVolumePercentage.set_text(&percentage);
            imp.resetVolumeSlider.set_value(*volume as f64);
            imp.stream.replace(stream);
            imp.resetVolumeSlider.connect_change_value(
                clone!(@weak imp => @default-return Propagation::Stop, move |_, _, value| {
                    let fraction = (value / 655.36).round();
                    println!("{fraction}");
                    let percentage = (fraction).to_string() + "%";
                    imp.resetVolumePercentage.set_text(&percentage);
                    let source = imp.stream.borrow();
                    let index = source.index;
                    let channels = source.channels;
                    set_source_volume(value, index, channels);
                    Propagation::Proceed
                }),
            );
            imp.resetSelectedSource.set_group(Some(&*check_group));
            // check_group.set_group(Some(&*imp.resetSelectedSink));
            if is_default {
                imp.resetSelectedSource.set_active(true);
            } else {
                imp.resetSelectedSource.set_active(false);
            }
            imp.resetSelectedSource.connect_toggled(move |button| {
                if button.is_active() {
                    set_default_source(name.clone());
                }
            });
            imp.resetSourceMute
                .connect_clicked(clone!(@weak imp => move |_| {
                    let stream = imp.stream.clone();
                    let mut stream = stream.borrow_mut();
                    stream.muted = !stream.muted;
                    let muted = stream.muted;
                    let index = stream.index;
                    if muted {
                        imp.resetSourceMute
                           .set_icon_name("audio-volume-muted-symbolic");
                    } else {
                        imp.resetSourceMute
                           .set_icon_name("audio-volume-high-symbolic");
                    }
                    toggle_source_mute(index, muted);
                }));
        }
        obj
    }
}

pub fn set_source_volume(value: f64, index: u32, channels: u16) -> bool {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(bool,), Error> = proxy.method_call(
        "org.xetibo.ReSet",
        "SetSourceVolume",
        (index, channels, value as u32),
    );
    if res.is_err() {
        return false;
    }
    res.unwrap().0
}

pub fn toggle_source_mute(index: u32, muted: bool) -> bool {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(bool,), Error> =
        proxy.method_call("org.xetibo.ReSet", "SetSourceMute", (index, muted));
    if res.is_err() {
        return false;
    }
    res.unwrap().0
}

pub fn set_default_source(name: Arc<String>) {
    thread::spawn(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.xetibo.ReSet",
            "/org/xetibo/ReSet",
            Duration::from_millis(1000),
        );
        let res: Result<(bool,), Error> =
            proxy.method_call("org.xetibo.ReSet", "SetDefaultSink", (name.as_str(),));
        if res.is_err() {
            return;
        }
        // handle change
    });
}
