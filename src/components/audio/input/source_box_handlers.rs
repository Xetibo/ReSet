use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use adw::prelude::{ComboRowExt, PreferencesRowExt};
use glib::prelude::Cast;
use glib::{subclass::types::ObjectSubclassIsExt, ControlFlow, Propagation};
use gtk::{
    gio,
    prelude::{BoxExt, ButtonExt, CheckButtonExt, ListBoxRowExt, RangeExt},
    StringObject,
};
use re_set_lib::{
    audio::audio_structures::{OutputStream, Source},
    signals::{
        OutputStreamAdded, OutputStreamChanged, OutputStreamRemoved, SourceAdded, SourceChanged,
        SourceRemoved,
    },
};

use crate::components::{
    audio::{
        generic_audio_box_utils::refresh_default_audio_object, generic_utils::audio_dbus_call,
    },
    base::list_entry::ListEntry,
};

use super::{
    output_stream_entry::OutputStreamEntry,
    source_box::SourceBox,
    source_const::{GETDEFAULTNAME, SETDEFAULT, SETMUTE, SETVOLUME},
    source_entry::SourceEntry,
};

pub fn source_added_handler(source_box: Arc<SourceBox>, ir: SourceAdded) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let source_box = source_box.clone();
            let source_box_imp = source_box.imp();
            let source_index = ir.source.index;
            let alias = ir.source.alias.clone();
            let name = ir.source.name.clone();
            let mut is_default = false;
            if source_box_imp.reset_default_source.borrow().name == ir.source.name {
                is_default = true;
            }
            let source_entry = SourceEntry::new(
                is_default,
                source_box_imp.reset_default_check_button.clone(),
                ir.source,
                source_box.clone(),
            );
            let source_clone = source_entry.clone();
            let entry = Arc::new(ListEntry::new(&*source_entry));
            entry.set_activatable(false);
            let mut list = source_box_imp.reset_source_list.write().unwrap();
            list.insert(source_index, (entry.clone(), source_clone, alias.clone()));
            source_box_imp.reset_sources.append(&*entry);
            let mut map = source_box_imp.reset_source_map.write().unwrap();
            let mut index = source_box_imp.reset_model_index.write().unwrap();
            let model_list = source_box_imp.reset_model_list.write().unwrap();
            if model_list.string(*index - 1) == Some("Monitor of Dummy Output".into()) {
                model_list.append(&alias);
                model_list.remove(*index - 1);
                map.insert(alias, (source_index, name));
                source_box_imp.reset_source_dropdown.set_selected(0);
            } else {
                model_list.append(&alias);
                map.insert(alias.clone(), (source_index, name));
                if alias == "Monitor of Dummy Output" {
                    source_box_imp.reset_source_dropdown.set_selected(0);
                }
                *index += 1;
            }
        });
    });
    true
}

pub fn source_removed_handler(source_box: Arc<SourceBox>, ir: SourceRemoved) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let source_box = source_box.clone();
            let source_box_imp = source_box.imp();
            let entry: Option<(Arc<ListEntry>, Arc<SourceEntry>, String)>;
            {
                let mut list = source_box_imp.reset_source_list.write().unwrap();
                entry = list.remove(&ir.index);
                if entry.is_none() {
                    return;
                }
            }
            source_box_imp
                .reset_sources
                .remove(&*entry.clone().unwrap().0);
            let mut map = source_box_imp.reset_source_map.write().unwrap();
            let alias = entry.unwrap().2;
            map.remove(&alias);
            let mut index = source_box_imp.reset_model_index.write().unwrap();
            let model_list = source_box_imp.reset_model_list.write().unwrap();

            if *index == 1 {
                model_list.append("Monitor of Dummy Output");
            }
            for entry in 0..*index {
                if model_list.string(entry) == Some(alias.clone().into()) {
                    model_list.splice(entry, 1, &[]);
                    break;
                }
            }
            if *index > 1 {
                *index -= 1;
            }
        });
    });
    true
}

pub fn source_changed_handler(source_box: Arc<SourceBox>, ir: SourceChanged) -> bool {
    let source =
        audio_dbus_call::<SourceBox, (String,), ()>(source_box.clone(), (), &GETDEFAULTNAME);
    if source.is_none() {
        return false;
    }
    let default_source = source.unwrap().0;
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let source_box = source_box.clone();
            let source_box_imp = source_box.imp();
            let is_default = ir.source.name == default_source;
            let volume = ir.source.volume.first().unwrap_or(&0_u32);
            let fraction = (*volume as f64 / 655.36).round();
            let percentage = (fraction).to_string() + "%";

            let list = source_box_imp.reset_source_list.read().unwrap();
            let entry = list.get(&ir.source.index);
            if entry.is_none() {
                return;
            }
            let imp = entry.unwrap().1.imp();
            if is_default {
                source_box_imp.reset_volume_percentage.set_text(&percentage);
                source_box_imp.reset_volume_slider.set_value(*volume as f64);
                source_box_imp
                    .reset_default_source
                    .replace(ir.source.clone());
                if ir.source.muted {
                    source_box_imp
                        .reset_source_mute
                        .set_icon_name("microphone-disabled-symbolic");
                } else {
                    source_box_imp
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
}

pub fn output_stream_added_handler(source_box: Arc<SourceBox>, ir: OutputStreamAdded) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let source_box = source_box.clone();
            let source_box_imp = source_box.imp();
            let mut list = source_box_imp.reset_output_stream_list.write().unwrap();
            let index = ir.stream.index;
            let output_stream = OutputStreamEntry::new(source_box.clone(), ir.stream);
            let entry = Arc::new(ListEntry::new(&*output_stream));
            entry.set_activatable(false);
            list.insert(index, (entry.clone(), output_stream.clone()));
            source_box_imp.reset_output_streams.append(&*entry);
        });
    });
    true
}

pub fn output_stream_changed_handler(source_box: Arc<SourceBox>, ir: OutputStreamChanged) -> bool {
    let imp = source_box.imp();
    let alias: String;
    {
        let source_list = imp.reset_source_list.read().unwrap();
        if let Some(alias_opt) = source_list.get(&ir.stream.source_index) {
            alias = alias_opt.2.clone();
        } else {
            alias = String::from("");
        }
    }
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let source_box = source_box.clone();
            let source_box_imp = source_box.imp();
            let entry: Arc<OutputStreamEntry>;
            {
                let list = source_box_imp.reset_output_stream_list.read().unwrap();
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
            let index = source_box_imp.reset_model_index.read().unwrap();
            let model_list = source_box_imp.reset_model_list.read().unwrap();
            for entry in 0..*index {
                if model_list.string(entry) == Some(alias.clone().into()) {
                    imp.reset_source_selection.set_selected(entry);
                    break;
                }
            }
        });
    });
    true
}

pub fn output_stream_removed_handler(source_box: Arc<SourceBox>, ir: OutputStreamRemoved) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let source_box = source_box.clone();
            let source_box_imp = source_box.imp();
            let mut list = source_box_imp.reset_output_stream_list.write().unwrap();
            let entry = list.remove(&ir.index);
            if entry.is_none() {
                return;
            }
            source_box_imp
                .reset_output_streams
                .remove(&*entry.unwrap().0);
        });
    });
    true
}

