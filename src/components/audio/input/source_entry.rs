use std::sync::Arc;
use std::time::Duration;

use crate::components::audio::generic_entry::{new_entry, Audio};
use crate::components::base::error_impl::show_error;
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::types::ObjectSubclassIsExt;
use gtk::{gio, CheckButton};
use re_set_lib::audio::audio_structures::Source;

use crate::components::utils::{AUDIO, BASE, DBUS_PATH};

use super::source_box::SourceBox;
use super::source_entry_impl;

glib::wrapper! {
    pub struct SourceEntry(ObjectSubclass<source_entry_impl::SourceEntry>)
    @extends adw::PreferencesGroup, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for SourceEntry {}
unsafe impl Sync for SourceEntry {}

impl Audio<Source, super::source_entry_impl::SourceEntry> for SourceEntry {
    fn entry_imp(&self) -> &super::source_entry_impl::SourceEntry {
        self.imp()
    }
}

impl SourceEntry {
    pub fn new(
        is_default: bool,
        check_group: Arc<CheckButton>,
        source: Source,
        input_box: Arc<SourceBox>,
    ) -> Arc<Self> {
        new_entry::<
            Source,
            SourceBox,
            SourceEntry,
            super::source_entry_impl::SourceEntry,
            super::source_box_impl::SourceBox,
        >(is_default, check_group, source, input_box)
    }
}

pub fn set_source_volume(value: f64, index: u32, channels: u16, input_box: Arc<SourceBox>) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
        let res: Result<(), Error> =
            proxy.method_call(AUDIO, "SetSourceVolume", (index, channels, value as u32));
        if res.is_err() {
            // TODO: also log this with LOG/ERROR
            show_error::<SourceBox>(input_box.clone(), "Failed to set source volume");
        }
    });
    true
}

pub fn toggle_source_mute(index: u32, muted: bool, input_box: Arc<SourceBox>) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
        let res: Result<(), Error> = proxy.method_call(AUDIO, "SetSourceMute", (index, muted));
        if res.is_err() {
            show_error::<SourceBox>(input_box.clone(), "Failed to mute source");
        }
    });
    true
}

pub fn set_default_source(name: Arc<String>, input_box: Arc<SourceBox>) -> Option<Source> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Source,), Error> =
        proxy.method_call(AUDIO, "SetDefaultSource", (name.as_str(),));
    if res.is_err() {
        show_error::<SourceBox>(input_box.clone(), "Failed to set default source");
        return None;
    }
    Some(res.unwrap().0)
}
