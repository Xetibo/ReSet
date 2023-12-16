use adw::prelude::PreferencesRowExt;
use re_set_lib::audio::audio_structures::{Card, OutputStream, Source};
use re_set_lib::signals::{
    OutputStreamAdded, OutputStreamChanged, OutputStreamRemoved, SourceAdded, SourceChanged,
    SourceRemoved,
};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use adw::glib;
use adw::glib::Object;
use adw::prelude::{
    BoxExt, ButtonExt, CheckButtonExt, ComboRowExt, ListBoxRowExt, PreferencesGroupExt, RangeExt,
};
use dbus::blocking::Connection;
use dbus::message::SignalArgs;
use dbus::{Error, Path};
use glib::subclass::prelude::ObjectSubclassIsExt;
use glib::{clone, Cast, Propagation, Variant};
use gtk::prelude::ActionableExt;
use gtk::{gio, StringObject};

use crate::components::base::card_entry::CardEntry;
use crate::components::base::list_entry::ListEntry;
use crate::components::input::source_box_impl;
use crate::components::input::source_entry::set_source_volume;
use crate::components::utils::{
    create_dropdown_label_factory, set_combo_row_ellipsis, AUDIO, BASE, DBUS_PATH,
};

use super::output_stream_entry::OutputStreamEntry;
use super::source_entry::{set_default_source, toggle_source_mute, SourceEntry};

glib::wrapper! {
    pub struct SourceBox(ObjectSubclass<source_box_impl::SourceBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for SourceBox {}
unsafe impl Sync for SourceBox {}

impl SourceBox {
    pub fn new() -> Self {
        let obj: Self = Object::builder().build();
        {
            let imp = obj.imp();
            let mut model_index = imp.reset_model_index.write().unwrap();
            *model_index = 0;
        }
        obj
    }

    pub fn setup_callbacks(&self) {
        let self_imp = self.imp();
        self_imp.reset_source_row.set_activatable(true);
        self_imp
            .reset_source_row
            .set_action_name(Some("navigation.push"));
        self_imp
            .reset_source_row
            .set_action_target_value(Some(&Variant::from("sources")));
        self_imp.reset_cards_row.set_activatable(true);
        self_imp
            .reset_cards_row
            .set_action_name(Some("navigation.push"));
        self_imp
            .reset_cards_row
            .set_action_target_value(Some(&Variant::from("profileConfiguration")));

        self_imp.reset_output_stream_button.set_activatable(true);
        self_imp
            .reset_output_stream_button
            .set_action_name(Some("navigation.pop"));

        self_imp.reset_input_cards_back_button.set_activatable(true);
        self_imp
            .reset_input_cards_back_button
            .set_action_name(Some("navigation.pop"));

        self_imp
            .reset_source_dropdown
            .set_factory(Some(&create_dropdown_label_factory()));
        set_combo_row_ellipsis(self_imp.reset_source_dropdown.get());
    }
}

impl Default for SourceBox {
    fn default() -> Self {
        Self::new()
    }
}

pub fn populate_sources(input_box: Arc<SourceBox>) {
    gio::spawn_blocking(move || {
        let sources = get_sources();
        {
            let input_box_imp = input_box.imp();
            let list = input_box_imp.reset_model_list.write().unwrap();
            let mut map = input_box_imp.reset_source_map.write().unwrap();
            let mut model_index = input_box_imp.reset_model_index.write().unwrap();
            input_box_imp
                .reset_default_source
                .replace(get_default_source());
            for source in sources.iter() {
                list.append(&source.alias);
                map.insert(source.alias.clone(), (source.index, source.name.clone()));
                *model_index += 1;
            }
        }

        populate_outputstreams(input_box.clone());
        populate_cards(input_box.clone());
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let input_box_ref_slider = input_box.clone();
                let input_box_ref_toggle = input_box.clone();
                let input_box_ref_mute = input_box.clone();
                let input_box_ref = input_box.clone();
                {
                    let input_box_imp = input_box_ref.imp();
                    let default_sink = input_box_imp.reset_default_source.clone();
                    let source = default_sink.borrow();

                    if source.muted {
                        input_box_imp
                            .reset_source_mute
                            .set_icon_name("microphone-disabled-symbolic");
                    } else {
                        input_box_imp
                            .reset_source_mute
                            .set_icon_name("audio-input-microphone-symbolic");
                    }

                    let volume = source.volume.first().unwrap_or(&0_u32);
                    let fraction = (*volume as f64 / 655.36).round();
                    let percentage = (fraction).to_string() + "%";
                    input_box_imp.reset_volume_percentage.set_text(&percentage);
                    input_box_imp.reset_volume_slider.set_value(*volume as f64);
                    let mut list = input_box_imp.reset_source_list.write().unwrap();
                    for source in sources {
                        let index = source.index;
                        let alias = source.alias.clone();
                        let mut is_default = false;
                        if input_box_imp.reset_default_source.borrow().name == source.name {
                            is_default = true;
                        }
                        let source_entry = Arc::new(SourceEntry::new(
                            is_default,
                            input_box_imp.reset_default_check_button.clone(),
                            source,
                            input_box.clone(),
                        ));
                        let source_clone = source_entry.clone();
                        let entry = Arc::new(ListEntry::new(&*source_entry));
                        entry.set_activatable(false);
                        list.insert(index, (entry.clone(), source_clone, alias));
                        input_box_imp.reset_sources.append(&*entry);
                    }
                    let list = input_box_imp.reset_model_list.read().unwrap();
                    input_box_imp.reset_source_dropdown.set_model(Some(&*list));
                    let name = input_box_imp.reset_default_source.borrow();

                    let index = input_box_imp.reset_model_index.read().unwrap();
                    let model_list = input_box_imp.reset_model_list.read().unwrap();
                    for entry in 0..*index {
                        if model_list.string(entry) == Some(name.alias.clone().into()) {
                            input_box_imp.reset_source_dropdown.set_selected(entry);
                            break;
                        }
                    }
                    input_box_imp.reset_source_dropdown.connect_selected_notify(
                        clone!(@weak input_box_imp => move |dropdown| {
                            let input_box = input_box_ref_toggle.clone();
                            let selected = dropdown.selected_item();
                            if selected.is_none() {
                                return;
                            }
                            let selected = selected.unwrap();
                            let selected = selected.downcast_ref::<StringObject>().unwrap();
                            let selected = selected.string().to_string();

                            let source = input_box_imp.reset_source_map.read().unwrap();
                            let source = source.get(&selected);
                            if source.is_none() {
                                return;
                            }
                            let source = Arc::new(source.unwrap().1.clone());
                            gio::spawn_blocking(move || {
                                let result = set_default_source(source);
                                if result.is_none(){
                                    return;
                                }
                                refresh_default_source(result.unwrap(), input_box.clone(), false);
                            });
                        }),
                    );
                }
                input_box_ref
                    .imp()
                    .reset_volume_slider
                    .connect_change_value(move |_, _, value| {
                        let imp = input_box_ref_slider.imp();
                        let fraction = (value / 655.36).round();
                        let percentage = (fraction).to_string() + "%";
                        imp.reset_volume_percentage.set_text(&percentage);
                        let source = imp.reset_default_source.borrow();
                        let index = source.index;
                        let channels = source.channels;
                        {
                            let mut time = imp.volume_time_stamp.borrow_mut();
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

                input_box_ref
                    .imp()
                    .reset_source_mute
                    .connect_clicked(move |_| {
                        let imp = input_box_ref_mute.imp();
                        let mut source = imp.reset_default_source.borrow_mut();
                        source.muted = !source.muted;
                        if source.muted {
                            imp.reset_source_mute
                                .set_icon_name("microphone-disabled-symbolic");
                        } else {
                            imp.reset_source_mute
                                .set_icon_name("audio-input-microphone-symbolic");
                        }
                        toggle_source_mute(source.index, source.muted);
                    });
            });
        });
    });
}

pub fn refresh_default_source(new_source: Source, input_box: Arc<SourceBox>, entry: bool) {
    let volume = *new_source.volume.first().unwrap_or(&0_u32);
    let fraction = (volume as f64 / 655.36).round();
    let percentage = (fraction).to_string() + "%";
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let imp = input_box.imp();
            if !entry {
                let list = imp.reset_source_list.read().unwrap();
                let entry = list.get(&new_source.index);
                if entry.is_none() {
                    return;
                }
                let entry_imp = entry.unwrap().1.imp();
                entry_imp.reset_selected_source.set_active(true);
            } else {
                let model_list = imp.reset_model_list.read().unwrap();
                for entry in 0..*imp.reset_model_index.read().unwrap() {
                    if model_list.string(entry) == Some(new_source.alias.clone().into()) {
                        imp.reset_source_dropdown.set_selected(entry);
                        break;
                    }
                }
            }
            imp.reset_volume_percentage.set_text(&percentage);
            imp.reset_volume_slider.set_value(volume as f64);
            if new_source.muted {
                imp.reset_source_mute
                    .set_icon_name("microphone-disabled-symbolic");
            } else {
                imp.reset_source_mute
                    .set_icon_name("audio-input-microphone-symbolic");
            }
            imp.reset_default_source.replace(new_source);
        });
    });
}

pub fn populate_outputstreams(input_box: Arc<SourceBox>) {
    let input_box_ref = input_box.clone();

    gio::spawn_blocking(move || {
        let streams = get_output_streams();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let input_box_imp = input_box_ref.imp();
                let mut list = input_box_imp.reset_output_stream_list.write().unwrap();
                for stream in streams {
                    let index = stream.index;
                    let input_stream = Arc::new(OutputStreamEntry::new(input_box.clone(), stream));
                    let input_stream_clone = input_stream.clone();
                    let entry = Arc::new(ListEntry::new(&*input_stream));
                    entry.set_activatable(false);
                    list.insert(index, (entry.clone(), input_stream_clone));
                    input_box_imp.reset_output_streams.append(&*entry);
                }
            });
        });
    });
}

pub fn populate_cards(input_box: Arc<SourceBox>) {
    gio::spawn_blocking(move || {
        let input_box_ref = input_box.clone();
        let cards = get_cards();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let imp = input_box_ref.imp();
                for card in cards {
                    imp.reset_cards.add(&CardEntry::new(card));
                }
            });
        });
    });
}

fn get_output_streams() -> Vec<OutputStream> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Vec<OutputStream>,), Error> =
        proxy.method_call(AUDIO, "ListOutputStreams", ());
    if res.is_err() {
        return Vec::new();
    }
    res.unwrap().0
}

fn get_sources() -> Vec<Source> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Vec<Source>,), Error> = proxy.method_call(AUDIO, "ListSources", ());
    if res.is_err() {
        return Vec::new();
    }
    res.unwrap().0
}

fn get_cards() -> Vec<Card> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Vec<Card>,), Error> = proxy.method_call(AUDIO, "ListCards", ());
    if res.is_err() {
        return Vec::new();
    }
    res.unwrap().0
}

fn get_default_source_name() -> String {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(String,), Error> = proxy.method_call(AUDIO, "GetDefaultSourceName", ());
    if res.is_err() {
        return String::from("");
    }
    res.unwrap().0
}

fn get_default_source() -> Source {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Source,), Error> = proxy.method_call(AUDIO, "GetDefaultSource", ());
    if res.is_err() {
        return Source::default();
    }
    res.unwrap().0
}

pub fn start_input_box_listener(conn: Connection, source_box: Arc<SourceBox>) -> Connection {
    let source_added =
        SourceAdded::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();
    let source_removed =
        SourceRemoved::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();
    let source_changed =
        SourceChanged::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();
    let output_stream_added =
        OutputStreamAdded::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH)))
            .static_clone();
    let output_stream_removed =
        OutputStreamRemoved::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH)))
            .static_clone();
    let output_stream_changed =
        OutputStreamChanged::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH)))
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
                let input_box = source_box.clone();
                let input_box_imp = input_box.imp();
                let source_index = ir.source.index;
                let alias = ir.source.alias.clone();
                let name = ir.source.name.clone();
                let mut is_default = false;
                if input_box_imp.reset_default_source.borrow().name == ir.source.name {
                    is_default = true;
                }
                let source_entry = Arc::new(SourceEntry::new(
                    is_default,
                    input_box_imp.reset_default_check_button.clone(),
                    ir.source,
                    input_box.clone(),
                ));
                let source_clone = source_entry.clone();
                let entry = Arc::new(ListEntry::new(&*source_entry));
                entry.set_activatable(false);
                let mut list = input_box_imp.reset_source_list.write().unwrap();
                list.insert(source_index, (entry.clone(), source_clone, alias.clone()));
                input_box_imp.reset_sources.append(&*entry);
                let mut map = input_box_imp.reset_source_map.write().unwrap();
                let mut index = input_box_imp.reset_model_index.write().unwrap();
                let model_list = input_box_imp.reset_model_list.write().unwrap();
                if model_list.string(*index - 1) == Some("Monitor of Dummy Output".into()) {
                    model_list.append(&alias);
                    model_list.remove(*index - 1);
                    map.insert(alias, (source_index, name));
                    input_box_imp.reset_source_dropdown.set_selected(0);
                } else {
                    model_list.append(&alias);
                    map.insert(alias.clone(), (source_index, name));
                    if alias == "Monitor of Dummy Output" {
                        input_box_imp.reset_source_dropdown.set_selected(0);
                    }
                    *index += 1;
                }
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on source add event");
        return conn;
    }

    let res = conn.add_match(source_removed, move |ir: SourceRemoved, _, _| {
        let source_box = source_removed_box.clone();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let input_box = source_box.clone();
                let input_box_imp = input_box.imp();
                let entry: Option<(Arc<ListEntry>, Arc<SourceEntry>, String)>;
                {
                    let mut list = input_box_imp.reset_source_list.write().unwrap();
                    entry = list.remove(&ir.index);
                    if entry.is_none() {
                        return;
                    }
                }
                input_box_imp
                    .reset_sources
                    .remove(&*entry.clone().unwrap().0);
                let mut map = input_box_imp.reset_source_map.write().unwrap();
                let alias = entry.unwrap().2;
                map.remove(&alias);
                let mut index = input_box_imp.reset_model_index.write().unwrap();
                let model_list = input_box_imp.reset_model_list.write().unwrap();

                if *index == 1 {
                    model_list.append("Monitor of Dummy Output");
                }
                for entry in 0..*index {
                    if model_list.string(entry) == Some(alias.clone().into()) {
                        model_list.remove(entry);
                        break;
                    }
                }
                if *index > 1 {
                    *index -= 1;
                }
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on source remove event");
        return conn;
    }

    let res = conn.add_match(source_changed, move |ir: SourceChanged, _, _| {
        let source_box = source_changed_box.clone();
        let default_source = get_default_source_name();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let input_box = source_box.clone();
                let input_box_imp = input_box.imp();
                let is_default = ir.source.name == default_source;
                let volume = ir.source.volume.first().unwrap_or(&0_u32);
                let fraction = (*volume as f64 / 655.36).round();
                let percentage = (fraction).to_string() + "%";

                let list = input_box_imp.reset_source_list.read().unwrap();
                let entry = list.get(&ir.source.index);
                if entry.is_none() {
                    return;
                }
                let imp = entry.unwrap().1.imp();
                if is_default {
                    input_box_imp.reset_volume_percentage.set_text(&percentage);
                    input_box_imp.reset_volume_slider.set_value(*volume as f64);
                    input_box_imp
                        .reset_default_source
                        .replace(ir.source.clone());
                    if ir.source.muted {
                        input_box_imp
                            .reset_source_mute
                            .set_icon_name("microphone-disabled-symbolic");
                    } else {
                        input_box_imp
                            .reset_source_mute
                            .set_icon_name("audio-input-microphone-symbolic");
                    }
                    imp.reset_selected_source.set_active(true);
                } else {
                    imp.reset_selected_source.set_active(false);
                }
                imp.reset_source_name
                    .set_title(ir.source.alias.clone().as_str());
                imp.reset_volume_percentage.set_text(&percentage);
                imp.reset_volume_slider.set_value(*volume as f64);
                if ir.source.muted {
                    imp.reset_source_mute
                        .set_icon_name("microphone-disabled-symbolic");
                } else {
                    imp.reset_source_mute
                        .set_icon_name("audio-input-microphone-symbolic");
                }
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on source change event");
        return conn;
    }

    let res = conn.add_match(output_stream_added, move |ir: OutputStreamAdded, _, _| {
        let source_box = output_stream_added_box.clone();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let input_box = source_box.clone();
                let input_box_imp = input_box.imp();
                let mut list = input_box_imp.reset_output_stream_list.write().unwrap();
                let index = ir.stream.index;
                let output_stream = Arc::new(OutputStreamEntry::new(input_box.clone(), ir.stream));
                let entry = Arc::new(ListEntry::new(&*output_stream));
                entry.set_activatable(false);
                list.insert(index, (entry.clone(), output_stream.clone()));
                input_box_imp.reset_output_streams.append(&*entry);
            });
        });
        true
    });
    if res.is_err() {
        println!("fail on output stream add event");
        return conn;
    }

    let res = conn.add_match(
        output_stream_changed,
        move |ir: OutputStreamChanged, _, _| {
            let imp = output_stream_changed_box.imp();
            let alias: String;
            {
                let source_list = imp.reset_source_list.read().unwrap();
                if let Some(alias_opt) = source_list.get(&ir.stream.source_index) {
                    alias = alias_opt.2.clone();
                } else {
                    alias = String::from("");
                }
            }
            let source_box = output_stream_changed_box.clone();
            glib::spawn_future(async move {
                glib::idle_add_once(move || {
                    let input_box = source_box.clone();
                    let input_box_imp = input_box.imp();
                    let entry: Arc<OutputStreamEntry>;
                    {
                        let list = input_box_imp.reset_output_stream_list.read().unwrap();
                        let entry_opt = list.get(&ir.stream.index);
                        if entry_opt.is_none() {
                            return;
                        }
                        entry = entry_opt.unwrap().1.clone();
                    }
                    let imp = entry.imp();
                    if ir.stream.muted {
                        imp.reset_source_mute
                            .set_icon_name("microphone-disabled-symbolic");
                    } else {
                        imp.reset_source_mute
                            .set_icon_name("audio-input-microphone-symbolic");
                    }
                    let name = ir.stream.application_name.clone() + ": " + ir.stream.name.as_str();
                    imp.reset_source_selection.set_title(name.as_str());
                    let volume = ir.stream.volume.first().unwrap_or(&0_u32);
                    let fraction = (*volume as f64 / 655.36).round();
                    let percentage = (fraction).to_string() + "%";
                    imp.reset_volume_percentage.set_text(&percentage);
                    imp.reset_volume_slider.set_value(*volume as f64);
                    let index = input_box_imp.reset_model_index.read().unwrap();
                    let model_list = input_box_imp.reset_model_list.read().unwrap();
                    for entry in 0..*index {
                        if model_list.string(entry) == Some(alias.clone().into()) {
                            imp.reset_source_selection.set_selected(entry);
                            break;
                        }
                    }
                });
            });
            true
        },
    );
    if res.is_err() {
        println!("fail on output stream change event");
        return conn;
    }

    let res = conn.add_match(
        output_stream_removed,
        move |ir: OutputStreamRemoved, _, _| {
            let source_box = output_stream_removed_box.clone();
            glib::spawn_future(async move {
                glib::idle_add_once(move || {
                    let input_box = source_box.clone();
                    let input_box_imp = input_box.imp();
                    let mut list = input_box_imp.reset_output_stream_list.write().unwrap();
                    let entry = list.remove(&ir.index);
                    if entry.is_none() {
                        return;
                    }
                    input_box_imp
                        .reset_output_streams
                        .remove(&*entry.unwrap().0);
                });
            });
            true
        },
    );
    if res.is_err() {
        println!("fail on output stream remove event");
        return conn;
    }

    conn
}
