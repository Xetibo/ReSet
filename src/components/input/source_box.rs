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
use glib::{clone, Cast, ControlFlow, Propagation, Variant};
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
use super::source_box_event_handlers::{
    output_stream_added_handler, output_stream_changed_handler, output_stream_removed_handler,
    source_added_handler, source_changed_handler, source_removed_handler,
};
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
        populate_source_information(input_box, sources);
    });
}

fn populate_source_information(input_box: Arc<SourceBox>, sources: Vec<Source>) {
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
                input_box_imp
                    .reset_source_dropdown
                    .connect_selected_notify(move |dropdown| {
                        if let ControlFlow::Break() =
                            dropdown_handler(input_box_ref_toggle, dropdown, input_box_imp)
                        {
                            return;
                        }
                    });
            }
            input_box_ref
                .imp()
                .reset_volume_slider
                .connect_change_value(move |_, _, value| {
                    volume_slider_handler(input_box_ref_slider, value)
                });

            input_box_ref
                .imp()
                .reset_source_mute
                .connect_clicked(move |_| {
                    mute_clicked_handler(input_box_ref_mute);
                });
        });
    });
}

fn dropdown_handler(
    input_box_ref_toggle: Arc<SourceBox>,
    dropdown: &adw::ComboRow,
    input_box_imp: &source_box_impl::SourceBox,
) -> ControlFlow<()> {
    let input_box = input_box_ref_toggle.clone();
    let selected = dropdown.selected_item();
    if selected.is_none() {
        return ControlFlow::Break(());
    }
    let selected = selected.unwrap();
    let selected = selected.downcast_ref::<StringObject>().unwrap();
    let selected = selected.string().to_string();
    let source = input_box_imp.reset_source_map.read().unwrap();
    let source = source.get(&selected);
    if source.is_none() {
        return ControlFlow::Break(());
    }
    let source = Arc::new(source.unwrap().1.clone());
    gio::spawn_blocking(move || {
        let result = set_default_source(source);
        if result.is_none() {
            return ControlFlow::Break(());
        }
        refresh_default_source(result.unwrap(), input_box.clone(), false);
    });

    ControlFlow::Continue(())
}

fn volume_slider_handler(input_box_ref_slider: Arc<SourceBox>, value: f64) -> Propagation {
    let imp = input_box_ref_slider.imp();
    let fraction = (value / 655.36).round();
    let percentage = (fraction).to_string() + "%";
    imp.reset_volume_percentage.set_text(&percentage);
    let source = imp.reset_default_source.borrow();
    let index = source.index;
    let channels = source.channels;
    {
        let mut time = imp.volume_time_stamp.borrow_mut();
        if time.is_some() && time.unwrap().elapsed().unwrap() < Duration::from_millis(50) {
            return Propagation::Proceed;
        }
        *time = Some(SystemTime::now());
    }
    set_source_volume(value, index, channels);
    Propagation::Proceed
}

fn mute_clicked_handler(input_box_ref_mute: Arc<SourceBox>) {
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

pub fn get_default_source_name() -> String {
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
        source_added_handler(source_box, ir)
    });
    if res.is_err() {
        // TODO: handle this with the log/error macro
        println!("fail on source add event");
        return conn;
    }

    let res = conn.add_match(source_removed, move |ir: SourceRemoved, _, _| {
        source_removed_handler(source_box, ir)
    });
    if res.is_err() {
        println!("fail on source remove event");
        return conn;
    }

    let res = conn.add_match(source_changed, move |ir: SourceChanged, _, _| {
        source_changed_handler(source_box, ir)
    });
    if res.is_err() {
        println!("fail on source change event");
        return conn;
    }

    let res = conn.add_match(output_stream_added, move |ir: OutputStreamAdded, _, _| {
        output_stream_added_handler(output_stream_added_box, ir)
    });
    if res.is_err() {
        println!("fail on output stream add event");
        return conn;
    }

    let res = conn.add_match(
        output_stream_changed,
        move |ir: OutputStreamChanged, _, _| {
            output_stream_changed_handler(output_stream_changed_box, ir)
        },
    );
    if res.is_err() {
        println!("fail on output stream change event");
        return conn;
    }

    let res = conn.add_match(
        output_stream_removed,
        move |ir: OutputStreamRemoved, _, _| {
            output_stream_removed_handler(output_stream_removed_box, ir)
        },
    );
    if res.is_err() {
        println!("fail on output stream remove event");
        return conn;
    }

    conn
}
