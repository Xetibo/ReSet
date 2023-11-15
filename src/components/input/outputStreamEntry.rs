use std::sync::Arc;
use std::time::Duration;

use adw::glib;
use adw::glib::Object;
use adw::prelude::{ButtonExt, RangeExt};
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::types::ObjectSubclassIsExt;
use glib::{clone, Cast, Propagation};
use gtk::StringObject;
use ReSet_Lib::audio::audio::OutputStream;

use super::outputStreamEntryImpl;
use super::sourceBox::SourceBox;

glib::wrapper! {
    pub struct OutputStreamEntry(ObjectSubclass<outputStreamEntryImpl::OutputStreamEntry>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl OutputStreamEntry {
    pub fn new(source_box: Arc<SourceBox>, stream: OutputStream) -> Self {
        let obj: Self = Object::builder().build();
        // TODO use event callback for progress bar -> this is the "im speaking" indicator
        // TODO map mute to callback
        // TODO map dropdown
        {
            let box_imp = source_box.imp();
            let imp = obj.imp();
            let name = stream.application_name.clone() + ": " + stream.name.as_str();
            imp.resetSourceName.set_text(name.as_str());
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
                    let mut stream = imp.stream.try_borrow();
                    while stream.is_err() {
                        stream = imp.stream.try_borrow();
                    }
                    let stream = stream.unwrap();
                    let index = stream.index;
                    let channels = stream.channels;
                    set_outputstream_volume(value, index, channels);
                    Propagation::Proceed
                }),
            );
            {
                let mut list = box_imp.resetModelList.try_borrow();
                while list.is_err() {
                    list = box_imp.resetModelList.try_borrow();
                }
                let list = list.unwrap();
                imp.resetSelectedSource.set_model(Some(&*list));
                let mut map = box_imp.resetSourceMap.try_borrow();
                while map.is_err() {
                    map = box_imp.resetSourceMap.try_borrow();
                }
                let map = map.unwrap();
                let mut name = box_imp.resetDefaultSource.try_borrow();
                while name.is_err() {
                    name = box_imp.resetDefaultSource.try_borrow();
                }
                let name = name.unwrap();
                let name = &name.alias;
                let index = map.get(name);
                if index.is_some() {
                    imp.resetSelectedSource.set_selected(index.unwrap().1);
                }
            }
            imp.resetSelectedSource.connect_selected_notify(
                clone!(@weak imp, @weak box_imp => move |dropdown| {
                    let selected = dropdown.selected_item();
                    if selected.is_none() {
                        return;
                    }
                    let selected = selected.unwrap();
                    let selected = selected.downcast_ref::<StringObject>().unwrap();
                    let selected = selected.string().to_string();
                    let mut source = box_imp.resetSourceMap.try_borrow();
                    while source.is_err() {
                        source = box_imp.resetSourceMap.try_borrow();
                    }
                    let source = source.unwrap();
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
            imp.resetSourceMute
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
                        imp.resetSourceMute
                           .set_icon_name("audio-volume-muted-symbolic");
                    } else {
                        imp.resetSourceMute
                           .set_icon_name("audio-volume-high-symbolic");
                    }
                    toggle_output_stream_mute(index, muted);
                }));
        }
        obj
    }
}

fn set_outputstream_volume(value: f64, index: u32, channels: u16) -> bool {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(bool,), Error> = proxy.method_call(
        "org.xetibo.ReSet",
        "SetOutputStreamVolume",
        (index, channels, value as u32),
    );
    if res.is_err() {
        return false;
    }
    res.unwrap().0
}

fn toggle_output_stream_mute(index: u32, muted: bool) -> bool {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(bool,), Error> =
        proxy.method_call("org.xetibo.ReSet", "SetOutputStreamMute", (index, muted));
    if res.is_err() {
        return false;
    }
    res.unwrap().0
}

fn set_source_of_output_stream(stream: u32, source: u32) -> bool {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(bool,), Error> = proxy.method_call(
        "org.xetibo.ReSet",
        "SetSourceOfOutputStream",
        (stream, source),
    );
    if res.is_err() {
        return false;
    }
    res.unwrap().0
}