use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use adw::{
    prelude::{ComboRowExt, PreferencesRowExt},
    ComboRow,
};
use glib::prelude::Cast;
use glib::{subclass::types::ObjectSubclassIsExt, Propagation};
use gtk::{
    gio,
    prelude::{BoxExt, ButtonExt, CheckButtonExt, ListBoxRowExt, RangeExt},
    StringObject,
};
use re_set_lib::{
    audio::audio_structures::Sink,
    signals::{
        InputStreamAdded, InputStreamChanged, InputStreamRemoved, SinkAdded, SinkChanged,
        SinkRemoved,
    },
};

use crate::components::{audio::generic_utils::audio_dbus_call, base::list_entry::ListEntry};

use super::{
    input_stream_entry::InputStreamEntry,
    sink_box::SinkBox,
    sink_box_utils::{get_default_sink_name, refresh_default_sink},
    sink_const::{SETDEFAULT, SETMUTE, SETVOLUME},
    sink_entry::SinkEntry,
};

pub fn drop_down_handler(sink_box: Arc<SinkBox>, dropdown: &ComboRow) {
    let sink_box_ref = sink_box.clone();
    let sink_box_imp = sink_box.imp();
    let selected = dropdown.selected_item();
    if selected.is_none() {
        return;
    }
    let selected = selected.unwrap();
    let selected = selected.downcast_ref::<StringObject>().unwrap();
    let selected = selected.string().to_string();

    let sink = sink_box_imp.reset_sink_map.read().unwrap();
    let sink = sink.get(&selected);
    if sink.is_none() {
        return;
    }
    let new_sink_name = Arc::new(sink.unwrap().1.clone());
    gio::spawn_blocking(move || {
        let result = audio_dbus_call::<SinkBox, (Sink,), (&String,)>(
            sink_box_ref.clone(),
            (&new_sink_name,),
            &SETDEFAULT,
        );
        if result.is_none() {
            return;
        }
        let new_sink = result.unwrap().0;
        refresh_default_sink(new_sink, sink_box_ref, false);
    });
}

pub fn volume_slider_handler(sink_box: Arc<SinkBox>, value: f64) -> glib::Propagation {
    let imp = sink_box.imp();
    let fraction = (value / 655.36).round();
    let percentage = (fraction).to_string() + "%";
    imp.reset_volume_percentage.set_text(&percentage);
    let sink = imp.reset_default_sink.borrow();
    let index = sink.index;
    let channels = sink.channels;
    {
        let mut time = imp.volume_time_stamp.borrow_mut();
        if time.is_some() && time.unwrap().elapsed().unwrap() < Duration::from_millis(50) {
            return Propagation::Proceed;
        }
        *time = Some(SystemTime::now());
    }
    audio_dbus_call::<SinkBox, (), (u32, u16, u32)>(
        sink_box.clone(),
        (index, channels, value as u32),
        &SETVOLUME,
    );
    Propagation::Proceed
}

pub fn mute_handler(sink_box: Arc<SinkBox>) {
    let imp = sink_box.imp();
    let mut stream = imp.reset_default_sink.borrow_mut();
    stream.muted = !stream.muted;
    if stream.muted {
        imp.reset_sink_mute
            .set_icon_name("audio-volume-muted-symbolic");
    } else {
        imp.reset_sink_mute
            .set_icon_name("audio-volume-high-symbolic");
    }
    audio_dbus_call::<SinkBox, (), (u32, bool)>(
        sink_box.clone(),
        (stream.index, stream.muted),
        &SETMUTE,
    );
}

pub fn sink_added_handler(sink_box: Arc<SinkBox>, ir: SinkAdded) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let sink_box_imp = sink_box.imp();
            let sink_index = ir.sink.index;
            let alias = ir.sink.alias.clone();
            let name = ir.sink.name.clone();
            let mut is_default = false;

            if sink_box_imp.reset_default_sink.borrow().name == ir.sink.name {
                is_default = true;
            }
            let sink_entry = SinkEntry::new(
                is_default,
                sink_box_imp.reset_default_check_button.clone(),
                ir.sink,
                sink_box.clone(),
            );
            let sink_clone = sink_entry.clone();
            let entry = Arc::new(ListEntry::new(&*sink_entry));
            entry.set_activatable(false);
            let mut list = sink_box_imp.reset_sink_list.write().unwrap();
            list.insert(sink_index, (entry.clone(), sink_clone, alias.clone()));
            sink_box_imp.reset_sinks.append(&*entry);
            let mut map = sink_box_imp.reset_sink_map.write().unwrap();
            let mut index = sink_box_imp.reset_model_index.write().unwrap();
            let model_list = sink_box_imp.reset_model_list.write().unwrap();
            if model_list.string(*index - 1) == Some("Dummy Output".into()) {
                model_list.append(&alias);
                model_list.remove(*index - 1);
                map.insert(alias, (sink_index, name));
                sink_box_imp.reset_sink_dropdown.set_selected(0);
            } else {
                model_list.append(&alias);
                map.insert(alias.clone(), (sink_index, name));
                if alias == "Dummy Output" {
                    sink_box_imp.reset_sink_dropdown.set_selected(0);
                }
                *index += 1;
            }
        });
    });
    true
}

pub fn sink_removed_handler(sink_box: Arc<SinkBox>, ir: SinkRemoved) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let sink_box_imp = sink_box.imp();

            let entry: Option<(Arc<ListEntry>, Arc<SinkEntry>, String)>;
            {
                let mut list = sink_box_imp.reset_sink_list.write().unwrap();
                entry = list.remove(&ir.index);
                if entry.is_none() {
                    return;
                }
            }
            sink_box_imp.reset_sinks.remove(&*entry.clone().unwrap().0);
            let alias = entry.unwrap().2;
            let mut index = sink_box_imp.reset_model_index.write().unwrap();
            let model_list = sink_box_imp.reset_model_list.write().unwrap();

            // add dummy entry when no other devices are available
            if *index == 1 {
                model_list.append("Dummy Output");
            }

            let mut map = sink_box_imp.reset_sink_map.write().unwrap();
            map.remove(&alias);

            for entry in 0..*index {
                if model_list.string(entry) == Some(alias.clone().into()) {
                    model_list.splice(entry, 1, &[]);
                    break;
                }
            }

            // dummy enforces a minimum of 1
            if *index > 1 {
                *index -= 1;
            }
        });
    });
    true
}

pub fn sink_changed_handler(sink_box: Arc<SinkBox>, ir: SinkChanged) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let default_sink = get_default_sink_name(sink_box.clone());
            let sink_box_imp = sink_box.imp();
            let is_default = ir.sink.name == default_sink;
            let volume = ir.sink.volume.first().unwrap_or(&0_u32);
            let fraction = (*volume as f64 / 655.36).round();
            let percentage = (fraction).to_string() + "%";

            let list = sink_box_imp.reset_sink_list.read().unwrap();
            let entry = list.get(&ir.sink.index);
            if entry.is_none() {
                return;
            }
            let imp = entry.unwrap().1.imp();
            if is_default {
                sink_box_imp.reset_volume_percentage.set_text(&percentage);
                sink_box_imp.reset_volume_slider.set_value(*volume as f64);
                sink_box_imp.reset_default_sink.replace(ir.sink.clone());
                if ir.sink.muted {
                    sink_box_imp
                        .reset_sink_mute
                        .set_icon_name("audio-volume-muted-symbolic");
                } else {
                    sink_box_imp
                        .reset_sink_mute
                        .set_icon_name("audio-volume-high-symbolic");
                }
                imp.reset_selected_sink.set_active(true);
            } else {
                imp.reset_selected_sink.set_active(false);
            }
            imp.reset_sink_name
                .set_title(ir.sink.alias.clone().as_str());
            imp.reset_volume_percentage.set_text(&percentage);
            imp.reset_volume_slider.set_value(*volume as f64);
            if ir.sink.muted {
                imp.reset_sink_mute
                    .set_icon_name("audio-volume-muted-symbolic");
            } else {
                imp.reset_sink_mute
                    .set_icon_name("audio-volume-high-symbolic");
            }
        });
    });
    true
}

pub fn input_stream_added_handler(sink_box: Arc<SinkBox>, ir: InputStreamAdded) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let sink_box_imp = sink_box.imp();
            let mut list = sink_box_imp.reset_input_stream_list.write().unwrap();
            let index = ir.stream.index;
            let input_stream = Arc::new(InputStreamEntry::new(sink_box.clone(), ir.stream));
            let entry = Arc::new(ListEntry::new(&*input_stream));
            entry.set_activatable(false);
            list.insert(index, (entry.clone(), input_stream.clone()));
            sink_box_imp.reset_input_streams.append(&*entry);
        });
    });
    true
}

pub fn input_stream_removed_handler(sink_box: Arc<SinkBox>, ir: InputStreamRemoved) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let sink_box_imp = sink_box.imp();
            let mut list = sink_box_imp.reset_input_stream_list.write().unwrap();
            let entry = list.remove(&ir.index);
            if entry.is_none() {
                return;
            }
            sink_box_imp.reset_input_streams.remove(&*entry.unwrap().0);
        });
    });
    true
}

pub fn input_stream_changed_handler(sink_box: Arc<SinkBox>, ir: InputStreamChanged) -> bool {
    let imp = sink_box.imp();
    let alias: String;
    {
        let sink_list = imp.reset_sink_list.read().unwrap();
        if let Some(alias_opt) = sink_list.get(&ir.stream.sink_index) {
            alias = alias_opt.2.clone();
        } else {
            alias = String::from("");
        }
    }
    let sink_box = sink_box.clone();
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let sink_box = sink_box.clone();
            let sink_box_imp = sink_box.imp();
            let entry: Arc<InputStreamEntry>;
            {
                let list = sink_box_imp.reset_input_stream_list.read().unwrap();
                let entry_opt = list.get(&ir.stream.index);
                if entry_opt.is_none() {
                    return;
                }
                entry = entry_opt.unwrap().1.clone();
            }
            let imp = entry.imp();
            if ir.stream.muted {
                imp.reset_sink_mute
                    .set_icon_name("audio-volume-muted-symbolic");
            } else {
                imp.reset_sink_mute
                    .set_icon_name("audio-volume-high-symbolic");
            }
            let name = ir.stream.application_name.clone() + ": " + ir.stream.name.as_str();
            imp.reset_sink_selection.set_title(name.as_str());
            let volume = ir.stream.volume.first().unwrap_or(&0_u32);
            let fraction = (*volume as f64 / 655.36).round();
            let percentage = (fraction).to_string() + "%";
            imp.reset_volume_percentage.set_text(&percentage);
            imp.reset_volume_slider.set_value(*volume as f64);
            let index = sink_box_imp.reset_model_index.read().unwrap();
            let model_list = sink_box_imp.reset_model_list.read().unwrap();
            for entry in 0..*index {
                if model_list.string(entry) == Some(alias.clone().into()) {
                    imp.reset_sink_selection.set_selected(entry);
                    break;
                }
            }
        });
    });
    true
}
