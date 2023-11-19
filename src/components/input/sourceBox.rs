use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::components::base::cardEntry::CardEntry;
use crate::components::base::listEntry::ListEntry;
use crate::components::base::utils::{
    Listeners, OutputStreamAdded, OutputStreamChanged, OutputStreamRemoved, SourceAdded,
    SourceChanged, SourceRemoved,
};
use crate::components::input::sourceBoxImpl;
use crate::components::input::sourceEntry::set_source_volume;
use adw::glib;
use adw::glib::Object;
use adw::prelude::{BoxExt, ButtonExt, CheckButtonExt, ListBoxRowExt, RangeExt};
use dbus::blocking::Connection;
use dbus::message::SignalArgs;
use dbus::{Error, Path};
use glib::subclass::prelude::ObjectSubclassIsExt;
use glib::{clone, Cast, Propagation, Variant};
use gtk::prelude::ActionableExt;
use gtk::{gio, StringObject};
use ReSet_Lib::audio::audio::{Card, OutputStream, Source};

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
            .resetCardsRow
            .set_action_name(Some("navigation.push"));
        selfImp
            .resetCardsRow
            .set_action_target_value(Some(&Variant::from("profileConfiguration")));
        selfImp
            .resetOutputStreamButton
            .set_action_name(Some("navigation.pop"));
        selfImp
            .resetInputCardsBackButton
            .set_action_name(Some("navigation.pop"));
    }
}

pub fn populate_sources(input_box: Arc<SourceBox>) {
    gio::spawn_blocking(move || {
        let output_box_imp = input_box.imp();
        let sources = get_sources();
        {
            let list = output_box_imp.resetModelList.write().unwrap();
            let mut map = output_box_imp.resetSourceMap.write().unwrap();
            let mut model_index = output_box_imp.resetModelIndex.write().unwrap();
            let mut i: u32 = 0;
            for source in sources.iter() {
                list.append(&source.alias);
                map.insert(source.alias.clone(), (source.index, i, source.name.clone()));
                i += 1;
                *model_index += 1;
            }
        }
        output_box_imp
            .resetDefaultSource
            .replace(get_default_source());

        populate_outputstreams(input_box.clone());
        populate_cards(input_box.clone());
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                // TODO handle events
                let output_box_ref_slider = input_box.clone();
                let output_box_ref_mute = input_box.clone();
                let output_box_ref = input_box.clone();
                {
                    let output_box_imp = output_box_ref.imp();
                    let default_sink = output_box_imp.resetDefaultSource.clone();
                    let source = default_sink.borrow();

                    let volume = source.volume.first().unwrap_or_else(|| &(0 as u32));
                    let fraction = (*volume as f64 / 655.36).round();
                    let percentage = (fraction).to_string() + "%";
                    output_box_imp.resetVolumePercentage.set_text(&percentage);
                    output_box_imp.resetVolumeSlider.set_value(*volume as f64);
                    let mut list = output_box_imp.resetSourceList.write().unwrap();
                    for stream in sources {
                        let index = source.index;
                        let alias = source.alias.clone();
                        let mut is_default = false;
                        if output_box_imp.resetDefaultSource.borrow().name == stream.name {
                            is_default = true;
                        }
                        let source_entry = Arc::new(SourceEntry::new(
                            is_default,
                            output_box_imp.resetDefaultCheckButton.clone(),
                            stream,
                        ));
                        let source_clone = source_entry.clone();
                        let entry = Arc::new(ListEntry::new(&*source_entry));
                        entry.set_activatable(false);
                        list.insert(index, (entry.clone(), source_clone, alias));
                        output_box_imp.resetSources.append(&*entry);
                    }
                    let list = output_box_imp.resetModelList.read().unwrap();
                    output_box_imp.resetSourceDropdown.set_model(Some(&*list));
                    let map = output_box_imp.resetSourceMap.read().unwrap();
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

                            let source = output_box_imp.resetSourceMap.read().unwrap();
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
                        let percentage = (fraction).to_string() + "%";
                        imp.resetVolumePercentage.set_text(&percentage);
                        let source = imp.resetDefaultSource.borrow();
                        let index = source.index;
                        let channels = source.channels;
                        {
                            let mut time = imp.volumeTimeStamp.borrow_mut();
                            if time.is_some()
                                && time.unwrap().elapsed().unwrap() < Duration::from_millis(50)
                            {
                                return Propagation::Proceed;
                            }
                            *time = Some(SystemTime::now());
                        }
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
                                .set_icon_name("microphone-disabled-symbolic");
                        } else {
                            imp.resetSourceMute
                                .set_icon_name("audio-input-microphone-symbolic");
                        }
                        toggle_source_mute(index, muted);
                    });
            });
        });
    });
}

pub fn populate_outputstreams(input_box: Arc<SourceBox>) {
    // TODO add listener
    let input_box_ref = input_box.clone();

    gio::spawn_blocking(move || {
        let streams = get_output_streams();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let input_box_imp = input_box_ref.imp();
                let mut list = input_box_imp.resetOutputStreamList.write().unwrap();
                for stream in streams {
                    let index = stream.index;
                    let input_stream = Arc::new(OutputStreamEntry::new(input_box.clone(), stream));
                    let input_stream_clone = input_stream.clone();
                    let entry = Arc::new(ListEntry::new(&*input_stream));
                    entry.set_activatable(false);
                    list.insert(index, (entry.clone(), input_stream_clone));
                    input_box_imp.resetOutputStreams.append(&*entry);
                }
            });
        });
    });
}

pub fn populate_cards(input_box: Arc<SourceBox>) {
    gio::spawn_blocking(move || {
        let output_box_ref = input_box.clone();
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

pub fn start_input_box_listener(conn: Connection, source_box: Arc<SourceBox>) -> Connection {
    let source_added = SourceAdded::match_rule(
        Some(&"org.xetibo.ReSet".into()),
        Some(&Path::from("/org/xetibo/ReSet")),
    )
    .static_clone();
    let source_removed = SourceRemoved::match_rule(
        Some(&"org.xetibo.ReSet".into()),
        Some(&Path::from("/org/xetibo/ReSet")),
    )
    .static_clone();
    let source_changed = SourceChanged::match_rule(
        Some(&"org.xetibo.ReSet".into()),
        Some(&Path::from("/org/xetibo/ReSet")),
    )
    .static_clone();
    let output_stream_added = OutputStreamAdded::match_rule(
        Some(&"org.xetibo.ReSet".into()),
        Some(&Path::from("/org/xetibo/ReSet")),
    )
    .static_clone();
    let output_stream_removed = OutputStreamRemoved::match_rule(
        Some(&"org.xetibo.ReSet".into()),
        Some(&Path::from("/org/xetibo/ReSet")),
    )
    .static_clone();
    let output_stream_changed = OutputStreamChanged::match_rule(
        Some(&"org.xetibo.ReSet".into()),
        Some(&Path::from("/org/xetibo/ReSet")),
    )
    .static_clone();

    let source_added_box = source_box.clone();
    let source_removed_box = source_box.clone();
    let source_changed_box = source_box.clone();
    let output_stream_added_box = source_box.clone();
    let output_stream_removed_box = source_box.clone();
    let output_stream_changed_box = source_box.clone();

    let res = conn.add_match(source_added, move |ir: SourceAdded, _, _| {
        let source_box = source_added_box.clone();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box = source_box.clone();
                let output_box_imp = output_box.imp();
                let mut list = output_box_imp.resetSourceList.write().unwrap();
                let source_index = ir.source.index;
                let alias = ir.source.alias.clone();
                let name = ir.source.name.clone();
                let mut is_default = false;
                if output_box_imp.resetDefaultSource.borrow().name == ir.source.name {
                    is_default = true;
                }
                let source_entry = Arc::new(SourceEntry::new(
                    is_default,
                    output_box_imp.resetDefaultCheckButton.clone(),
                    ir.source,
                ));
                let source_clone = source_entry.clone();
                let entry = Arc::new(ListEntry::new(&*source_entry));
                entry.set_activatable(false);
                list.insert(source_index, (entry.clone(), source_clone, alias.clone()));
                output_box_imp.resetSources.append(&*entry);
                let mut map = output_box_imp.resetSourceMap.write().unwrap();
                let mut index = output_box_imp.resetModelIndex.write().unwrap();
                map.insert(alias, (source_index, *index, name));
                *index += 1;
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on source add");
        return conn;
    }

    let res = conn.add_match(source_removed, move |ir: SourceRemoved, _, _| {
        let source_box = source_removed_box.clone();
        println!("removed {}", ir.index);
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box = source_box.clone();
                let output_box_imp = output_box.imp();
                let mut list = output_box_imp.resetSourceList.write().unwrap();
                let entry = list.get(&ir.index);
                if entry.is_none() {
                    return;
                }
                output_box_imp.resetSources.remove(&*entry.unwrap().0);
                list.remove(&ir.index);
                let alias = list.remove(&ir.index);
                if alias.is_none() {
                    return;
                }
                let mut map = output_box_imp.resetSourceMap.write().unwrap();
                map.remove(&alias.unwrap().2);
                let mut index = output_box_imp.resetModelIndex.write().unwrap();
                if *index != 0 {
                    *index -= 1;
                }
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on source remove");
        return conn;
    }

    let res = conn.add_match(source_changed, move |ir: SourceChanged, _, _| {
        let source_box = source_changed_box.clone();
        let default_source = get_default_source();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box = source_box.clone();
                let output_box_imp = output_box.imp();
                let list = output_box_imp.resetSourceList.read().unwrap();
                let entry = list.get(&ir.source.index);
                if entry.is_none() {
                    return;
                }
                let imp = entry.unwrap().1.imp();
                let is_default = ir.source.name == default_source.name;
                imp.resetSourceName
                    .set_text(ir.source.alias.clone().as_str());
                let volume = ir.source.volume.first().unwrap_or_else(|| &(0 as u32));
                let fraction = (*volume as f64 / 655.36).round();
                let percentage = (fraction).to_string() + "%";
                imp.resetVolumePercentage.set_text(&percentage);
                imp.resetVolumeSlider.set_value(*volume as f64);
                if is_default {
                    imp.resetSelectedSource.set_active(true);
                } else {
                    imp.resetSelectedSource.set_active(false);
                }
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on source remove");
        return conn;
    }

    let res = conn.add_match(output_stream_added, move |ir: OutputStreamAdded, _, _| {
        let source_box = output_stream_added_box.clone();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let output_box = source_box.clone();
                let output_box_imp = output_box.imp();
                let mut list = output_box_imp.resetOutputStreamList.write().unwrap();
                let index = ir.stream.index;
                let output_stream = Arc::new(OutputStreamEntry::new(output_box.clone(), ir.stream));
                let output_stream_clone = output_stream.clone();
                let entry = Arc::new(ListEntry::new(&*output_stream));
                entry.set_activatable(false);
                list.insert(index, (entry.clone(), output_stream_clone));
                output_box_imp.resetOutputStreams.append(&*entry);
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on stream add");
        return conn;
    }

    let res = conn.add_match(
        output_stream_changed,
        move |ir: OutputStreamChanged, _, _| {
            let imp = output_stream_changed_box.imp();
            let alias: String;
            {
                let source_list = imp.resetSourceList.read().unwrap();
                let alias_opt = source_list.get(&ir.stream.source_index);
                if alias_opt.is_some() {
                    alias = alias_opt.unwrap().2.clone();
                } else {
                    alias = String::from("");
                }
            }
            let source_box = output_stream_changed_box.clone();
            glib::spawn_future(async move {
                glib::idle_add_once(move || {
                    let output_box = source_box.clone();
                    let output_box_imp = output_box.imp();
                    let entry: Arc<OutputStreamEntry>;
                    {
                        let list = output_box_imp.resetOutputStreamList.read().unwrap();
                        let entry_opt = list.get(&ir.stream.index);
                        if entry_opt.is_none() {
                            return;
                        }
                        entry = entry_opt.unwrap().1.clone();
                    }
                    let imp = entry.imp();
                    if ir.stream.muted {
                        imp.resetSourceMute
                            .set_icon_name("microphone-disabled-symbolic");
                    } else {
                        imp.resetSourceMute
                            .set_icon_name("audio-input-microphone-symbolic");
                    }
                    let name = ir.stream.application_name.clone() + ": " + ir.stream.name.as_str();
                    imp.resetSourceName.set_text(name.as_str());
                    let volume = ir.stream.volume.first().unwrap_or_else(|| &(0 as u32));
                    let fraction = (*volume as f64 / 655.36).round();
                    let percentage = (fraction).to_string() + "%";
                    imp.resetVolumePercentage.set_text(&percentage);
                    imp.resetVolumeSlider.set_value(*volume as f64);
                    let map = output_box_imp.resetSourceMap.read().unwrap();
                    let index = map.get(&alias);
                    if index.is_some() {
                        imp.resetSelectedSource.set_selected(index.unwrap().1);
                    }
                });
            });
            true
        },
    );
    if res.is_err() {
        println!("fail on stream change");
        return conn;
    }

    let res = conn.add_match(
        output_stream_removed,
        move |ir: OutputStreamRemoved, _, _| {
            let source_box = output_stream_removed_box.clone();
            glib::spawn_future(async move {
                glib::idle_add_once(move || {
                    let output_box = source_box.clone();
                    let output_box_imp = output_box.imp();
                    let mut list = output_box_imp.resetOutputStreamList.write().unwrap();
                    let entry = list.remove(&ir.index);
                    if entry.is_none() {
                        println!("tried to remove nonexistant?? wat");
                        return;
                    }
                    output_box_imp.resetOutputStreams.remove(&*entry.unwrap().0);
                });
            });
            true
        },
    );
    if res.is_err() {
        println!("fail on stream remove");
        return conn;
    }

    conn
}
