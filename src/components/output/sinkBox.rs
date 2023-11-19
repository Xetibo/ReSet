use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::components::base::cardEntry::CardEntry;
use crate::components::base::listEntry::ListEntry;
use crate::components::base::utils::{
    InputStreamAdded, InputStreamChanged, InputStreamRemoved, SinkAdded, SinkChanged, SinkRemoved,
};
use crate::components::output::sinkEntry::set_sink_volume;
use adw::glib::Object;
use adw::prelude::{BoxExt, ButtonExt, CheckButtonExt, RangeExt};
use adw::{glib, prelude::ListBoxRowExt};
use dbus::blocking::Connection;
use dbus::message::SignalArgs;
use dbus::{Error, Path};
use glib::subclass::prelude::ObjectSubclassIsExt;
use glib::{clone, Cast, Propagation, Variant};
use gtk::prelude::ActionableExt;
use gtk::{gio, StringObject};
use ReSet_Lib::audio::audio::{Card, InputStream, Sink};

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
        let obj: Self = Object::builder().build();
        {
            let imp = obj.imp();
            let mut model_index = imp.resetModelIndex.write().unwrap();
            *model_index = 0;
        }
        obj
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
            .resetCardsRow
            .set_action_name(Some("navigation.push"));
        selfImp
            .resetCardsRow
            .set_action_target_value(Some(&Variant::from("profileConfiguration")));
        selfImp.resetCardsRow.connect_action_name_notify(|_| {});

        selfImp
            .resetInputStreamButton
            .set_action_name(Some("navigation.pop"));
        selfImp
            .resetInputCardsBackButton
            .set_action_name(Some("navigation.pop"));
    }
}

impl Default for SinkBox {
    fn default() -> Self {
        Self::new()
    }
}

pub fn populate_sinks(output_box: Arc<SinkBox>) {
    gio::spawn_blocking(move || {
        let output_box_ref = output_box.clone();
        let sinks = get_sinks();
        {
            let output_box_imp = output_box.imp();
            let mut map = output_box_imp.resetSinkMap.write().unwrap();
            let list = output_box_imp.resetModelList.write().unwrap();
            let mut model_index = output_box_imp.resetModelIndex.write().unwrap();
            output_box_imp.resetDefaultSink.replace(get_default_sink());
            let mut i: u32 = 0;
            for sink in sinks.iter() {
                list.append(&sink.alias);
                map.insert(sink.alias.clone(), (sink.index, i, sink.name.clone()));
                i += 1;
                *model_index += 1;
            }
        }
        populate_inputstreams(output_box.clone());
        populate_cards(output_box.clone());
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box_ref_slider = output_box.clone();
                let output_box_ref_mute = output_box.clone();
                {
                    let output_box_imp = output_box_ref.imp();
                    let default_sink = output_box_imp.resetDefaultSink.clone();
                    let sink = default_sink.borrow();

                    let volume = sink.volume.first().unwrap_or(&0);
                    let fraction = (*volume as f64 / 655.36).round();
                    let percentage = (fraction).to_string() + "%";
                    output_box_imp.resetVolumePercentage.set_text(&percentage);
                    output_box_imp.resetVolumeSlider.set_value(*volume as f64);
                    let mut list = output_box_imp.resetSinkList.write().unwrap();
                    for sink in sinks {
                        let index = sink.index;
                        let alias = sink.alias.clone();
                        let mut is_default = false;
                        if output_box_imp.resetDefaultSink.borrow().name == sink.name {
                            is_default = true;
                        }
                        let sink_entry = Arc::new(SinkEntry::new(
                            is_default,
                            output_box_imp.resetDefaultCheckButton.clone(),
                            sink,
                        ));
                        let sink_clone = sink_entry.clone();
                        let entry = Arc::new(ListEntry::new(&*sink_entry));
                        entry.set_activatable(false);
                        list.insert(index, (entry.clone(), sink_clone, alias));
                        output_box_imp.resetSinks.append(&*entry);
                    }
                    let list = output_box_imp.resetModelList.read().unwrap();
                    output_box_imp.resetSinkDropdown.set_model(Some(&*list));
                    let map = output_box_imp.resetSinkMap.read().unwrap();
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

                            let sink = output_box_imp.resetSinkMap.read().unwrap();
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
                        let percentage = (fraction).to_string() + "%";
                        imp.resetVolumePercentage.set_text(&percentage);
                        let sink = imp.resetDefaultSink.borrow();
                        let index = sink.index;
                        let channels = sink.channels;
                        {
                            let mut time = imp.volumeTimeStamp.borrow_mut();
                            if time.is_some()
                                && time.unwrap().elapsed().unwrap() < Duration::from_millis(50)
                            {
                                return Propagation::Proceed;
                            }
                            *time = Some(SystemTime::now());
                        }
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

pub fn populate_inputstreams(output_box: Arc<SinkBox>) {
    let output_box_ref = output_box.clone();

    gio::spawn_blocking(move || {
        let streams = get_input_streams();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box_imp = output_box_ref.imp();
                let mut list = output_box_imp.resetInputStreamList.write().unwrap();
                for stream in streams {
                    let index = stream.index;
                    let input_stream = Arc::new(InputStreamEntry::new(output_box.clone(), stream));
                    let entry = Arc::new(ListEntry::new(&*input_stream));
                    entry.set_activatable(false);
                    list.insert(index, (entry.clone(), input_stream.clone()));
                    output_box_imp.resetInputStreams.append(&*entry);
                }
            });
        });
    });
}

pub fn populate_cards(output_box: Arc<SinkBox>) {
    gio::spawn_blocking(move || {
        let output_box_ref = output_box.clone();
        let cards = get_cards();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let imp = output_box_ref.imp();
                for card in cards {
                    imp.resetCards
                        .append(&ListEntry::new(&CardEntry::new(card)));
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

fn get_cards() -> Vec<Card> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(Vec<Card>,), Error> = proxy.method_call("org.xetibo.ReSet", "ListCards", ());
    if res.is_err() {
        return Vec::new();
    }
    res.unwrap().0
}

pub fn start_output_box_listener(conn: Connection, sink_box: Arc<SinkBox>) -> Connection {
    let sink_added = SinkAdded::match_rule(
        Some(&"org.xetibo.ReSet".into()),
        Some(&Path::from("/org/xetibo/ReSet")),
    )
    .static_clone();
    let sink_removed = SinkRemoved::match_rule(
        Some(&"org.xetibo.ReSet".into()),
        Some(&Path::from("/org/xetibo/ReSet")),
    )
    .static_clone();
    let sink_changed = SinkChanged::match_rule(
        Some(&"org.xetibo.ReSet".into()),
        Some(&Path::from("/org/xetibo/ReSet")),
    )
    .static_clone();
    let input_stream_added = InputStreamAdded::match_rule(
        Some(&"org.xetibo.ReSet".into()),
        Some(&Path::from("/org/xetibo/ReSet")),
    )
    .static_clone();
    let input_stream_removed = InputStreamRemoved::match_rule(
        Some(&"org.xetibo.ReSet".into()),
        Some(&Path::from("/org/xetibo/ReSet")),
    )
    .static_clone();
    let input_stream_changed = InputStreamChanged::match_rule(
        Some(&"org.xetibo.ReSet".into()),
        Some(&Path::from("/org/xetibo/ReSet")),
    )
    .static_clone();

    let sink_added_box = sink_box.clone();
    let sink_removed_box = sink_box.clone();
    let sink_changed_box = sink_box.clone();
    let input_stream_added_box = sink_box.clone();
    let input_stream_removed_box = sink_box.clone();
    let input_stream_changed_box = sink_box.clone();

    let res = conn.add_match(sink_added, move |ir: SinkAdded, _, _| {
        let sink_box = sink_added_box.clone();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box = sink_box.clone();
                let output_box_imp = output_box.imp();
                let mut list = output_box_imp.resetSinkList.write().unwrap();
                let sink_index = ir.sink.index;
                let alias = ir.sink.alias.clone();
                let name = ir.sink.name.clone();
                let mut is_default = false;
                if output_box_imp.resetDefaultSink.borrow().name == ir.sink.name {
                    is_default = true;
                }
                let sink_entry = Arc::new(SinkEntry::new(
                    is_default,
                    output_box_imp.resetDefaultCheckButton.clone(),
                    ir.sink,
                ));
                let sink_clone = sink_entry.clone();
                let entry = Arc::new(ListEntry::new(&*sink_entry));
                entry.set_activatable(false);
                list.insert(sink_index, (entry.clone(), sink_clone, alias.clone()));
                output_box_imp.resetSinks.append(&*entry);
                let mut map = output_box_imp.resetSinkMap.write().unwrap();
                let mut index = output_box_imp.resetModelIndex.write().unwrap();
                output_box_imp
                    .resetModelList
                    .write()
                    .unwrap()
                    .append(&alias);
                map.insert(alias, (sink_index, *index, name));
                *index += 1;
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on sink add");
        return conn;
    }

    let res = conn.add_match(sink_removed, move |ir: SinkRemoved, _, _| {
        let sink_box = sink_removed_box.clone();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box = sink_box.clone();
                let output_box_imp = output_box.imp();
                let mut list = output_box_imp.resetSinkList.write().unwrap();
                let entry = list.get(&ir.index);
                if entry.is_none() {
                    return;
                }
                output_box_imp.resetSinks.remove(&*entry.unwrap().0);
                let alias = list.remove(&ir.index);
                if alias.is_none() {
                    return;
                }
                let mut map = output_box_imp.resetSinkMap.write().unwrap();
                let entry_index = map.remove(&alias.unwrap().2);
                if entry_index.is_some() {
                    output_box_imp
                        .resetModelList
                        .write()
                        .unwrap()
                        .remove(entry_index.unwrap().1);
                }
                let mut index = output_box_imp.resetModelIndex.write().unwrap();
                if *index != 0 {
                    *index -= 1;
                }
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on sink remove");
        return conn;
    }

    let res = conn.add_match(sink_changed, move |ir: SinkChanged, _, _| {
        let sink_box = sink_changed_box.clone();
        let default_sink = get_default_sink();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box = sink_box.clone();
                let output_box_imp = output_box.imp();
                let list = output_box_imp.resetSinkList.read().unwrap();
                let entry = list.get(&ir.sink.index);
                if entry.is_none() {
                    return;
                }
                let imp = entry.unwrap().1.imp();
                let is_default = ir.sink.name == default_sink.name;
                imp.resetSinkName.set_text(ir.sink.alias.clone().as_str());
                let volume = ir.sink.volume.first().unwrap_or_else(|| &(0 as u32));
                let fraction = (*volume as f64 / 655.36).round();
                let percentage = (fraction).to_string() + "%";
                imp.resetVolumePercentage.set_text(&percentage);
                imp.resetVolumeSlider.set_value(*volume as f64);
                if is_default {
                    imp.resetSelectedSink.set_active(true);
                } else {
                    imp.resetSelectedSink.set_active(false);
                }
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on sink remove");
        return conn;
    }

    let res = conn.add_match(input_stream_added, move |ir: InputStreamAdded, _, _| {
        let sink_box = input_stream_added_box.clone();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box = sink_box.clone();
                let output_box_imp = output_box.imp();
                let mut list = output_box_imp.resetInputStreamList.write().unwrap();
                let index = ir.stream.index;
                let input_stream = Arc::new(InputStreamEntry::new(output_box.clone(), ir.stream));
                let entry = Arc::new(ListEntry::new(&*input_stream));
                entry.set_activatable(false);
                list.insert(index, (entry.clone(), input_stream.clone()));
                output_box_imp.resetInputStreams.append(&*entry);
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on stream add");
        return conn;
    }

    let res = conn.add_match(input_stream_changed, move |ir: InputStreamChanged, _, _| {
        let imp = input_stream_changed_box.imp();
        let alias: String;
        {
            let sink_list = imp.resetSinkList.read().unwrap();
            let alias_opt = sink_list.get(&ir.stream.sink_index);
            if alias_opt.is_some() {
                alias = alias_opt.unwrap().2.clone();
            } else {
                alias = String::from("");
            }
        }
        let sink_box = input_stream_changed_box.clone();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box = sink_box.clone();
                let output_box_imp = output_box.imp();
                let entry: Arc<InputStreamEntry>;
                {
                    let list = output_box_imp.resetInputStreamList.read().unwrap();
                    let entry_opt = list.get(&ir.stream.index);
                    if entry_opt.is_none() {
                        return;
                    }
                    entry = entry_opt.unwrap().1.clone();
                }
                let imp = entry.imp();
                if ir.stream.muted {
                    imp.resetSinkMute
                        .set_icon_name("audio-volume-muted-symbolic");
                } else {
                    imp.resetSinkMute
                        .set_icon_name("audio-volume-high-symbolic");
                }
                let name = ir.stream.application_name.clone() + ": " + ir.stream.name.as_str();
                imp.resetSinkName.set_text(name.as_str());
                let volume = ir.stream.volume.first().unwrap_or_else(|| &(0 as u32));
                let fraction = (*volume as f64 / 655.36).round();
                let percentage = (fraction).to_string() + "%";
                imp.resetVolumePercentage.set_text(&percentage);
                imp.resetVolumeSlider.set_value(*volume as f64);
                let map = output_box_imp.resetSinkMap.read().unwrap();
                let index = map.get(&alias);
                if index.is_some() {
                    imp.resetSelectedSink.set_selected(index.unwrap().1);
                }
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on stream change");
        return conn;
    }

    let res = conn.add_match(input_stream_removed, move |ir: InputStreamRemoved, _, _| {
        let sink_box = input_stream_removed_box.clone();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box = sink_box.clone();
                let output_box_imp = output_box.imp();
                let mut list = output_box_imp.resetInputStreamList.write().unwrap();
                let entry = list.get(&ir.index);
                if entry.is_none() {
                    return;
                }
                output_box_imp.resetInputStreams.remove(&*entry.unwrap().0);
                list.remove(&ir.index);
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on stream remove");
        return conn;
    }

    conn
}
