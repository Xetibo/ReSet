use std::sync::Arc;
use std::time::Duration;

use crate::components::base::listEntry::ListEntry;
use crate::components::base::utils::Listeners;
use crate::components::output::audioBoxImpl;
use adw::glib::Object;
use adw::prelude::BoxExt;
use adw::{glib, prelude::ListBoxRowExt};
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::prelude::ObjectSubclassIsExt;
use glib::Variant;
use gtk::gio;
use gtk::prelude::ActionableExt;
use ReSet_Lib::audio::audio::{InputStream, Sink};

use super::audioSource::{self, AudioSourceEntry};

glib::wrapper! {
    pub struct AudioBox(ObjectSubclass<audioBoxImpl::AudioBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for AudioBox {}
unsafe impl Sync for AudioBox {}

impl AudioBox {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn setupCallbacks(&self) {
        let selfImp = self.imp();
        selfImp
            .resetSinksRow
            .set_action_name(Some("navigation.push"));
        selfImp
            .resetSinksRow
            .set_action_target_value(Some(&Variant::from("outputDevices")));

        selfImp
            .resetOutputStreamButton
            .set_action_name(Some("navigation.pop"));
    }
}

pub fn populate_sinks(output_box: Arc<AudioBox>) {
    let output_box_ref = output_box.clone();
    gio::spawn_blocking(move || {
        let output_box_imp = output_box.imp();
        let sinks = get_sinks();
        output_box_imp.resetDefaultSink.replace(get_default_sink());
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box_imp = output_box_ref.imp();
                // TODO handle default mapping 
                // output_box_imp.resetVolumePercentage.set_text();
                // output_box_imp.resetVolumeSlider.let
                for stream in sinks {
                    let entry = ListEntry::new(&AudioSourceEntry::new(
                        stream.name,
                        stream.volume,
                        stream.muted,
                        stream.index,
                    ));
                    entry.set_activatable(false);
                    output_box_imp.resetSinks.append(&entry);
                }
            });
        });
    });
}

pub fn populate_streams(listeners: Arc<Listeners>, output_box: Arc<AudioBox>) {
    // TODO add listener
    let output_box_ref = output_box.clone();
    // let output_box_ref_listener = output_box.clone();
    let output_box_imp = output_box.imp();
    // let sources = output_box_imp.resetSinks.clone();
    // let output_streams = output_box_imp.resetInputStreams.clone();

    gio::spawn_blocking(move || {
        let streams = get_input_streams();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box_imp = output_box_ref.imp();
                for stream in streams {
                    let entry = ListEntry::new(&AudioSourceEntry::new(
                        stream.name,
                        stream.volume,
                        stream.muted,
                        stream.index,
                    ));
                    entry.set_activatable(false);
                    output_box_imp.resetOutputStreams.append(&entry);
                }
            });
        });
    });
}

fn get_input_streams() -> Vec<InputStream> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(Vec<InputStream>,), Error> =
        proxy.method_call("org.xetibo.ReSet", "ListInputStreams", ());
    if res.is_err() {
        return Vec::new();
    }
    res.unwrap().0
}

fn get_sinks() -> Vec<Sink> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(Vec<Sink>,), Error> = proxy.method_call("org.xetibo.ReSet", "ListSinks", ());
    if res.is_err() {
        return Vec::new();
    }
    res.unwrap().0
}

fn get_default_sink() -> Option<Sink> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(Sink,), Error> = proxy.method_call("org.xetibo.ReSet", "GetDefaultSink", ());
    if res.is_err() {
        return None;
    }
    Some(res.unwrap().0)
}
