use std::sync::Arc;
use std::time::Duration;

use crate::components::audio::generic_entry::{new_entry, Audio};
use crate::components::base::error_impl::show_error;
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::types::ObjectSubclassIsExt;
use gtk::{gio, CheckButton};
use re_set_lib::audio::audio_structures::Sink;

use crate::components::utils::{AUDIO, BASE, DBUS_PATH};

use super::sink_box::SinkBox;
use super::sink_entry_impl;

glib::wrapper! {
    pub struct SinkEntry(ObjectSubclass<sink_entry_impl::SinkEntry>)
    @extends adw::PreferencesGroup, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for SinkEntry {}
unsafe impl Sync for SinkEntry {}

impl Audio<Sink, super::sink_entry_impl::SinkEntry> for SinkEntry {
    fn entry_imp(&self) -> &super::sink_entry_impl::SinkEntry {
        self.imp()
    }
}

impl SinkEntry {
    pub fn new(
        is_default: bool,
        check_group: Arc<CheckButton>,
        sink: Sink,
        output_box: Arc<SinkBox>,
    ) -> Arc<Self> {
        new_entry::<
            Sink,
            SinkBox,
            SinkEntry,
            super::sink_entry_impl::SinkEntry,
            super::sink_box_impl::SinkBox,
        >(is_default, check_group, sink, output_box)
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
