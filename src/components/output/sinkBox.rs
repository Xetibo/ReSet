use std::sync::Arc;
use std::time::Duration;

use crate::components::base::listEntry::ListEntry;
use crate::components::base::utils::Listeners;
use crate::components::output::sinkEntry::set_sink_volume;
use adw::glib::Object;
use adw::prelude::{BoxExt, ButtonExt, RangeExt};
use adw::{glib, prelude::ListBoxRowExt};
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::prelude::ObjectSubclassIsExt;
use glib::{clone, Cast, Propagation, Variant};
use gtk::prelude::ActionableExt;
use gtk::{gio, StringObject};
use ReSet_Lib::audio::audio::{InputStream, Sink};

use super::inputStreamEntry::InputStreamEntry;
use super::sinkBoxImpl;
use super::sinkEntry::{set_default_sink, toggle_sink_mute, SinkEntry};

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
        let sinks = get_sinks();
        {
            let output_box_imp = output_box.imp();
            output_box_imp.resetDefaultSink.replace(get_default_sink());
            let list = output_box_imp.resetModelList.borrow_mut();
            let mut map = output_box_imp.resetSinkMap.borrow_mut();
            let mut i: u32 = 0;
            for sink in sinks.iter() {
                dbg!(sink.clone());
                list.append(&sink.alias);
                map.insert(sink.alias.clone(), (sink.index, i, sink.name.clone()));
                i += 1;
            }
        }
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box_ref_slider = output_box.clone();
                let output_box_ref_mute = output_box.clone();
                {
                    let output_box_imp = output_box_ref.imp();
                    let default_sink = output_box_imp.resetDefaultSink.clone();
                    let sink = default_sink.borrow();

                    let volume = sink.volume.first().unwrap_or_else(|| &(0 as u32));
                    let fraction = (*volume as f64 / 655.36).round();
                    let percentage = (fraction).to_string() + "%";
                    output_box_imp.resetVolumePercentage.set_text(&percentage);
                    output_box_imp.resetVolumeSlider.set_value(*volume as f64);
                    for stream in sinks {
                        let mut is_default = false;
                        if output_box_imp.resetDefaultSink.borrow().name == stream.name {
                            is_default = true;
                        }
                        let entry = ListEntry::new(&SinkEntry::new(
                            is_default,
                            output_box_imp.resetDefaultCheckButton.clone(),
                            stream,
                        ));
                        entry.set_activatable(false);
                        output_box_imp.resetSinks.append(&entry);
                    }
                    let list = output_box_imp.resetModelList.borrow();
                    output_box_imp.resetSinkDropdown.set_model(Some(&*list));
                    let map = output_box_imp.resetSinkMap.borrow();
                    let name = output_box_imp.resetDefaultSink.borrow();
                    let name = &name.alias;
                    let index = map.get(name);
                    if index.is_some() {
                        output_box_imp
                            .resetSinkDropdown
                            .set_selected(index.unwrap().1);
                    }
                    output_box_imp.resetSinkDropdown.connect_selected_notify(
                        clone!(@weak output_box_imp => move |dropdown| {
                            let selected = dropdown.selected_item();
                            if selected.is_none() {
                                return;
                            }
                            let selected = selected.unwrap();
                            let selected = selected.downcast_ref::<StringObject>().unwrap();
                            let selected = selected.string().to_string();

                            let sink = output_box_imp.resetSinkMap.borrow();
                            let sink = sink.get(&selected);
                            if sink.is_none() {
                                return;
                            }
                            let sink = Arc::new(sink.unwrap().2.clone());
                            set_default_sink(sink);
                        }),
                    );
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
                        let sink = imp.resetDefaultSink.borrow();
                        let index = sink.index;
                        let channels = sink.channels;
                        set_sink_volume(value, index, channels);
                        Propagation::Proceed
                    });
                output_box_ref
                    .imp()
                    .resetSinkMute
                    .connect_clicked(move |_| {
                        let imp = output_box_ref_mute.imp();
                        let stream = imp.resetDefaultSink.clone();
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
                        toggle_sink_mute(index, muted);
                    });
            });
        });
    });
}

pub fn populate_inputstreams(_listeners: Arc<Listeners>, output_box: Arc<SinkBox>) {
    // TODO add listener
    let output_box_ref = output_box.clone();

    gio::spawn_blocking(move || {
        let streams = get_input_streams();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box_imp = output_box_ref.imp();
                for stream in streams {
                    let entry = ListEntry::new(&InputStreamEntry::new(output_box.clone(), stream));
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
