use std::sync::Arc;
use std::time::Duration;

use crate::components::base::listEntry::ListEntry;
use crate::components::base::utils::Listeners;
use crate::components::output::sinkEntry::set_sink_volume;
use adw::glib::Object;
use adw::prelude::{BoxExt, RangeExt};
use adw::{glib, prelude::ListBoxRowExt};
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::prelude::ObjectSubclassIsExt;
use glib::{clone, Propagation, Variant};
use gtk::gio;
use gtk::prelude::ActionableExt;
use ReSet_Lib::audio::audio::{InputStream, Sink};

use super::inputStreamEntry::InputStreamEntry;
use super::sinkBoxImpl;
use super::sinkEntry::SinkEntry;

glib::wrapper! {
    pub struct SinkBox(ObjectSubclass<sinkBoxImpl::SinkBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for SinkBox {}
unsafe impl Sync for SinkBox {}

impl SinkBox {
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
            .resetInputStreamButton
            .set_action_name(Some("navigation.pop"));
    }
}

pub fn populate_sinks(output_box: Arc<SinkBox>) {
    gio::spawn_blocking(move || {
        let output_box_ref = output_box.clone();
        {
            let output_box_imp = output_box.imp();
            output_box_imp.resetDefaultSink.replace(get_default_sink());
        }
        let sinks = get_sinks();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                // TODO handle default mapping
                let output_box_ref_slider = output_box.clone();
                {
                    let output_box_imp = output_box_ref.imp();
                    let default_sink = output_box_imp.resetDefaultSink.clone(); // Clone outside closure
                    let sink = default_sink.borrow(); //

                    let volume = sink.volume.first().unwrap_or_else(|| &(0 as u32));
                    let fraction = (*volume as f64 / 655.36).round();
                    let percentage = (fraction).to_string() + "%";
                    output_box_imp.resetVolumePercentage.set_text(&percentage);
                    output_box_imp.resetVolumeSlider.set_value(*volume as f64);
                    for stream in sinks {
                        // TODO create sink handler -> currently only allows input streams
                        let entry = ListEntry::new(&SinkEntry::new(stream));
                        entry.set_activatable(false);
                        output_box_imp.resetSinks.append(&entry);
                    }
                }
                output_box_ref
                    .imp()
                    .resetVolumeSlider
                    .connect_change_value(move |_, _, value| {
                        let imp = output_box_ref_slider.imp();
                        let fraction = (value / 655.36).round();
                        println!("{fraction}");
                        let percentage = (fraction).to_string() + "%";
                        imp.resetVolumePercentage.set_text(&percentage);
                        set_sink_volume(value, imp.resetDefaultSink.clone());
                        Propagation::Proceed
                    });
            });
        });
    });
}

pub fn populate_inputstreams(listeners: Arc<Listeners>, output_box: Arc<SinkBox>) {
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
                    let entry = ListEntry::new(&InputStreamEntry::new(stream));
                    entry.set_activatable(false);
                    output_box_imp.resetInputStreams.append(&entry);
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

fn get_default_sink() -> Sink {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(Sink,), Error> = proxy.method_call("org.xetibo.ReSet", "GetDefaultSink", ());
    if res.is_err() {
        return Sink::default();
    }
    res.unwrap().0
}
