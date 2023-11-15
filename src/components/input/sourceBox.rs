use std::sync::Arc;
use std::time::Duration;

use crate::components::base::listEntry::ListEntry;
use crate::components::base::utils::Listeners;
use crate::components::input::sourceBoxImpl;
use crate::components::input::sourceEntry::set_source_volume;
use adw::glib;
use adw::glib::Object;
use adw::prelude::{BoxExt, ButtonExt, ListBoxRowExt, RangeExt};
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::prelude::ObjectSubclassIsExt;
use glib::{clone, Cast, Propagation, Variant};
use gtk::prelude::ActionableExt;
use gtk::{gio, StringObject};
use ReSet_Lib::audio::audio::{OutputStream, Source};

use super::outputStreamEntry::OutputStreamEntry;
use super::sourceEntry::{set_default_source, toggle_source_mute, SourceEntry};

glib::wrapper! {
    pub struct SourceBox(ObjectSubclass<sourceBoxImpl::SourceBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for SourceBox {}
unsafe impl Sync for SourceBox {}

impl SourceBox {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn setupCallbacks(&self) {
        let selfImp = self.imp();
        selfImp
            .resetSourceRow
            .set_action_name(Some("navigation.push"));
        selfImp
            .resetSourceRow
            .set_action_target_value(Some(&Variant::from("sources")));

        selfImp
            .resetOutputStreamButton
            .set_action_name(Some("navigation.pop"));
    }
}

pub fn populate_sources(output_box: Arc<SourceBox>) {
    gio::spawn_blocking(move || {
        let output_box_imp = output_box.imp();
        let sources = get_sources();
        {
            let list = output_box_imp.resetModelList.borrow_mut();
            let mut map = output_box_imp.resetSourceMap.borrow_mut();
            let mut i: u32 = 0;
            for source in sources.iter() {
                list.append(&source.alias);
                map.insert(source.alias.clone(), (source.index, i, source.name.clone()));
                i += 1;
            }
        }
        output_box_imp
            .resetDefaultSource
            .replace(get_default_source());
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                // TODO handle events
                let output_box_ref_slider = output_box.clone();
                let output_box_ref_mute = output_box.clone();
                let output_box_ref = output_box.clone();
                {
                    let output_box_imp = output_box_ref.imp();
                    let default_sink = output_box_imp.resetDefaultSource.clone(); // Clone outside closure
                    let source = default_sink.borrow(); //

                    let volume = source.volume.first().unwrap_or_else(|| &(0 as u32));
                    let fraction = (*volume as f64 / 655.36).round();
                    let percentage = (fraction).to_string() + "%";
                    output_box_imp.resetVolumePercentage.set_text(&percentage);
                    output_box_imp.resetVolumeSlider.set_value(*volume as f64);
                    for stream in sources {
                        let mut is_default = false;
                        if output_box_imp.resetDefaultSource.borrow().name == stream.name {
                            is_default = true;
                        }
                        let entry = ListEntry::new(&SourceEntry::new(
                            is_default,
                            output_box_imp.resetDefaultCheckButton.clone(),
                            stream,
                        ));
                        entry.set_activatable(false);
                        output_box_imp.resetSources.append(&entry);
                    }
                    let list = output_box_imp.resetModelList.borrow();
                    output_box_imp.resetSourceDropdown.set_model(Some(&*list));
                    let map = output_box_imp.resetSourceMap.borrow();
                    let name = output_box_imp.resetDefaultSource.borrow();
                    let name = &name.alias;
                    let index = map.get(name);
                    if index.is_some() {
                        output_box_imp
                            .resetSourceDropdown
                            .set_selected(index.unwrap().1);
                    }
                    output_box_imp.resetSourceDropdown.connect_selected_notify(
                        clone!(@weak output_box_imp => move |dropdown| {
                            let selected = dropdown.selected_item();
                            if selected.is_none() {
                                return;
                            }
                            let selected = selected.unwrap();
                            let selected = selected.downcast_ref::<StringObject>().unwrap();
                            let selected = selected.string().to_string();

                            let source = output_box_imp.resetSourceMap.borrow();
                            let source = source.get(&selected);
                            if source.is_none() {
                                return;
                            }
                            let sink = Arc::new(source.unwrap().2.clone());
                            set_default_source(sink);
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
                        let source = imp.resetDefaultSource.borrow();
                        let index = source.index;
                        let channels = source.channels;
                        set_source_volume(value, index, channels);
                        Propagation::Proceed
                    });

                output_box_ref
                    .imp()
                    .resetSourceMute
                    .connect_clicked(move |_| {
                        let imp = output_box_ref_mute.imp();
                        let stream = imp.resetDefaultSource.clone();
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
                    });
            });
        });
    });
}

pub fn populate_outputstreams(_listeners: Arc<Listeners>, output_box: Arc<SourceBox>) {
    // TODO add listener
    let output_box_ref = output_box.clone();

    gio::spawn_blocking(move || {
        let streams = get_output_streams();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box_imp = output_box_ref.imp();
                for stream in streams {
                    let entry = ListEntry::new(&OutputStreamEntry::new(output_box.clone(), stream));
                    entry.set_activatable(false);
                    output_box_imp.resetOutputStreams.append(&entry);
                }
            });
        });
    });
}

fn get_output_streams() -> Vec<OutputStream> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(Vec<OutputStream>,), Error> =
        proxy.method_call("org.xetibo.ReSet", "ListOutputStreams", ());
    if res.is_err() {
        return Vec::new();
    }
    res.unwrap().0
}

fn get_sources() -> Vec<Source> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(Vec<Source>,), Error> =
        proxy.method_call("org.xetibo.ReSet", "ListSources", ());
    if res.is_err() {
        return Vec::new();
    }
    res.unwrap().0
}

fn get_default_source() -> Source {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(Source,), Error> =
        proxy.method_call("org.xetibo.ReSet", "GetDefaultSource", ());
    if res.is_err() {
        return Source::default();
    }
    res.unwrap().0
}
