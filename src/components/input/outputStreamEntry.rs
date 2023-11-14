use std::cell::RefCell;
use std::sync::Arc;
use std::time::Duration;

use adw::glib;
use adw::glib::Object;
use adw::prelude::RangeExt;
use dbus::blocking::Connection;
use dbus::Error;
use ReSet_Lib::audio::audio::OutputStream;
use glib::{clone, Propagation};
use glib::subclass::types::ObjectSubclassIsExt;

use super::outputStreamEntryImpl;

glib::wrapper! {
    pub struct OutputStreamEntry(ObjectSubclass<outputStreamEntryImpl::OutputStreamEntry>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl OutputStreamEntry {
    pub fn new(stream: OutputStream) -> Self {
        let obj: Self = Object::builder().build();
        // TODO use event callback for progress bar -> this is the "im speaking" indicator
        // TODO map mute to callback
        // TODO map dropdown
        {
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
                    set_outputstream_volume(value, imp.stream.clone());
                    Propagation::Proceed
                }),
            );
        }
        obj
    }
}

fn set_outputstream_volume(value: f64, stream: Arc<RefCell<OutputStream>>) -> bool {
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
        proxy.method_call("org.xetibo.ReSet", "SetOutputStreamVolume", (stream,));
    if res.is_err() {
        return false;
    }
    res.unwrap().0
}
