use std::cell::RefCell;
use std::sync::Arc;
use std::time::Duration;

use adw::glib;
use adw::glib::Object;
use adw::prelude::RangeExt;
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::types::ObjectSubclassIsExt;
use glib::{clone, Propagation};
use ReSet_Lib::audio::audio::Sink;

use super::sinkEntryImpl;

glib::wrapper! {
    pub struct SinkEntry(ObjectSubclass<sinkEntryImpl::SinkEntry>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl SinkEntry {
    pub fn new(stream: Sink) -> Self {
        let obj: Self = Object::builder().build();
        // TODO use event callback for progress bar -> this is the "im speaking" indicator
        // TODO map the slider to volume
        // TODO properly use volume fraction
        // TODO map mute to callback
        // TODO map dropdown
        {
            let imp = obj.imp();
            imp.resetSinkName.set_text(stream.name.clone().as_str());
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
                    set_sink_volume(value, imp.stream.clone());
                    Propagation::Proceed
                }),
            );
        }
        obj
    }
}

pub fn set_sink_volume(value: f64, stream: Arc<RefCell<Sink>>) -> bool {
    let mut stream = stream.borrow_mut().clone();
    // let x = stream.volume.iter_mut().map(|_| value as u32);
    stream.volume = vec![value as u32; stream.channels as usize];
    dbg!(stream.volume.clone());

    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(bool,), Error> =
        proxy.method_call("org.xetibo.ReSet", "SetSinkVolume", (stream,));
    if res.is_err() {
        return false;
    }
    res.unwrap().0
}
