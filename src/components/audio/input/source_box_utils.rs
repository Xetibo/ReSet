use std::{sync::Arc, time::Duration};

use adw::prelude::{ComboRowExt, PreferencesGroupExt};
use dbus::{blocking::Connection, Error};
use glib::subclass::types::ObjectSubclassIsExt;
use gtk::{
    gio,
    prelude::{BoxExt, ButtonExt, CheckButtonExt, ListBoxRowExt, RangeExt},
};
use re_set_lib::audio::audio_structures::{Card, OutputStream, Source};

use crate::components::{
    base::{card_entry::CardEntry, error_impl::show_error, list_entry::ListEntry},
    utils::{AUDIO, BASE, DBUS_PATH},
};

use super::{
    output_stream_entry::OutputStreamEntry,
    source_box::SourceBox,
    source_box_handlers::{dropdown_handler, mute_clicked_handler, volume_slider_handler},
    source_entry::SourceEntry,
};

pub fn populate_source_information(source_box: Arc<SourceBox>, sources: Vec<Source>) {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let source_box_ref_slider = source_box.clone();
            let source_box_ref_toggle = source_box.clone();
            let source_box_ref_mute = source_box.clone();
            let source_box_imp = source_box.imp();
            let default_sink = source_box_imp.reset_default_source.clone();
            let source = default_sink.borrow();

            if source.muted {
                source_box_imp
                    .reset_source_mute
                    .set_icon_name("microphone-disabled-symbolic");
            } else {
                source_box_imp
                    .reset_source_mute
                    .set_icon_name("audio-input-microphone-symbolic");
            }

            let volume = source.volume.first().unwrap_or(&0_u32);
            let fraction = (*volume as f64 / 655.36).round();
            let percentage = (fraction).to_string() + "%";
            source_box_imp.reset_volume_percentage.set_text(&percentage);
            source_box_imp.reset_volume_slider.set_value(*volume as f64);
            let mut list = source_box_imp.reset_source_list.write().unwrap();
            for source in sources {
                let index = source.index;
                let alias = source.alias.clone();
                let mut is_default = false;
                if source_box_imp.reset_default_source.borrow().name == source.name {
                    is_default = true;
                }
                let source_entry = Arc::new(SourceEntry::new(
                    is_default,
                    source_box_imp.reset_default_check_button.clone(),
                    source,
                    source_box.clone(),
                ));
                let source_clone = source_entry.clone();
                let entry = Arc::new(ListEntry::new(&*source_entry));
                entry.set_activatable(false);
                list.insert(index, (entry.clone(), source_clone, alias));
                source_box_imp.reset_sources.append(&*entry);
            }
            let list = source_box_imp.reset_model_list.read().unwrap();
            source_box_imp.reset_source_dropdown.set_model(Some(&*list));
            let name = source_box_imp.reset_default_source.borrow();

            let index = source_box_imp.reset_model_index.read().unwrap();
            let model_list = source_box_imp.reset_model_list.read().unwrap();
            for entry in 0..*index {
                if model_list.string(entry) == Some(name.alias.clone().into()) {
                    source_box_imp.reset_source_dropdown.set_selected(entry);
                    break;
                }
            }
            source_box_imp
                .reset_source_dropdown
                .connect_selected_notify(move |dropdown| {
                    dropdown_handler(source_box_ref_toggle.clone(), dropdown);
                });
            source_box_imp
                .reset_volume_slider
                .connect_change_value(move |_, _, value| {
                    volume_slider_handler(source_box_ref_slider.clone(), value)
                });

            source_box_imp.reset_source_mute.connect_clicked(move |_| {
                mute_clicked_handler(source_box_ref_mute.clone());
            });
        });
    });
}

pub fn refresh_default_source(new_source: Source, source_box: Arc<SourceBox>, entry: bool) {
    let volume = *new_source.volume.first().unwrap_or(&0_u32);
    let fraction = (volume as f64 / 655.36).round();
    let percentage = (fraction).to_string() + "%";
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let imp = source_box.imp();
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

pub fn populate_outputstreams(source_box: Arc<SourceBox>) {
    let source_box_ref = source_box.clone();

    gio::spawn_blocking(move || {
        let streams = get_output_streams(source_box.clone());
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let source_box_imp = source_box_ref.imp();
                let mut list = source_box_imp.reset_output_stream_list.write().unwrap();
                for stream in streams {
                    let index = stream.index;
                    let input_stream = Arc::new(OutputStreamEntry::new(source_box.clone(), stream));
                    let input_stream_clone = input_stream.clone();
                    let entry = Arc::new(ListEntry::new(&*input_stream));
                    entry.set_activatable(false);
                    list.insert(index, (entry.clone(), input_stream_clone));
                    source_box_imp.reset_output_streams.append(&*entry);
                }
            });
        });
    });
}

pub fn populate_cards(source_box: Arc<SourceBox>) {
    gio::spawn_blocking(move || {
        let source_box_ref = source_box.clone();
        let cards = get_cards(source_box.clone());
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let imp = source_box_ref.imp();
                for card in cards {
                    imp.reset_cards.add(&CardEntry::new(card));
                }
            });
        });
    });
}

pub fn get_output_streams(source_box: Arc<SourceBox>) -> Vec<OutputStream> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Vec<OutputStream>,), Error> =
        proxy.method_call(AUDIO, "ListOutputStreams", ());
    if res.is_err() {
        show_error::<SourceBox>(source_box.clone(), "Failed to get output streams");
        return Vec::new();
    }
    res.unwrap().0
}

pub fn get_sources(source_box: Arc<SourceBox>) -> Vec<Source> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Vec<Source>,), Error> = proxy.method_call(AUDIO, "ListSources", ());
    if res.is_err() {
        show_error::<SourceBox>(source_box.clone(), "Failed to get sources");
        return Vec::new();
    }
    res.unwrap().0
}

pub fn get_cards(source_box: Arc<SourceBox>) -> Vec<Card> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Vec<Card>,), Error> = proxy.method_call(AUDIO, "ListCards", ());
    if res.is_err() {
        show_error::<SourceBox>(source_box.clone(), "Failed to get profiles");
        return Vec::new();
    }
    res.unwrap().0
}

pub fn get_default_source_name(source_box: Arc<SourceBox>) -> String {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(String,), Error> = proxy.method_call(AUDIO, "GetDefaultSourceName", ());
    if res.is_err() {
        show_error::<SourceBox>(source_box.clone(), "Failed to get default source name");
        return String::from("");
    }
    res.unwrap().0
}

pub fn get_default_source(source_box: Arc<SourceBox>) -> Source {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Source,), Error> = proxy.method_call(AUDIO, "GetDefaultSource", ());
    if res.is_err() {
        show_error::<SourceBox>(source_box.clone(), "Failed to get default source");
        return Source::default();
    }
    res.unwrap().0
}
