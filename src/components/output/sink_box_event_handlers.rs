use std::sync::Arc;

use adw::prelude::{ComboRowExt, PreferencesRowExt};
use glib::subclass::types::ObjectSubclassIsExt;
use gtk::prelude::{BoxExt, ButtonExt, CheckButtonExt, ListBoxRowExt, RangeExt};
use re_set_lib::signals::{
    InputStreamAdded, InputStreamChanged, InputStreamRemoved, SinkAdded, SinkChanged, SinkRemoved,
};

use crate::components::base::list_entry::ListEntry;

use super::{input_stream_entry::InputStreamEntry, sink_box::SinkBox, sink_entry::SinkEntry};

pub fn sink_added_handler(output_box: Arc<SinkBox>, ir: SinkAdded) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let output_box_imp = output_box.imp();
            let sink_index = ir.sink.index;
            let alias = ir.sink.alias.clone();
            let name = ir.sink.name.clone();
            let mut is_default = false;

            if output_box_imp.reset_default_sink.borrow().name == ir.sink.name {
                is_default = true;
            }
            let sink_entry = Arc::new(SinkEntry::new(
                is_default,
                output_box_imp.reset_default_check_button.clone(),
                ir.sink,
                output_box.clone(),
            ));
            let sink_clone = sink_entry.clone();
            let entry = Arc::new(ListEntry::new(&*sink_entry));
            entry.set_activatable(false);
            let mut list = output_box_imp.reset_sink_list.write().unwrap();
            list.insert(sink_index, (entry.clone(), sink_clone, alias.clone()));
            output_box_imp.reset_sinks.append(&*entry);
            let mut map = output_box_imp.reset_sink_map.write().unwrap();
            let mut index = output_box_imp.reset_model_index.write().unwrap();
            let model_list = output_box_imp.reset_model_list.write().unwrap();
            if model_list.string(*index - 1) == Some("Dummy Output".into()) {
                model_list.append(&alias);
                model_list.remove(*index - 1);
                map.insert(alias, (sink_index, name));
                output_box_imp.reset_sink_dropdown.set_selected(0);
            } else {
                model_list.append(&alias);
                map.insert(alias.clone(), (sink_index, name));
                if alias == "Dummy Output" {
                    output_box_imp.reset_sink_dropdown.set_selected(0);
                }
                *index += 1;
            }
        });
    });
    true
}

pub fn sink_removed_handler(output_box: Arc<SinkBox>, ir: SinkRemoved) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let output_box_imp = output_box.imp();

            let entry: Option<(Arc<ListEntry>, Arc<SinkEntry>, String)>;
            {
                let mut list = output_box_imp.reset_sink_list.write().unwrap();
                entry = list.remove(&ir.index);
                if entry.is_none() {
                    return;
                }
            }
            output_box_imp
                .reset_sinks
                .remove(&*entry.clone().unwrap().0);
            let alias = entry.unwrap().2;
            let mut index = output_box_imp.reset_model_index.write().unwrap();
            let model_list = output_box_imp.reset_model_list.write().unwrap();

            // add dummy entry when no other devices are available
            if *index == 1 {
                model_list.append("Dummy Output");
            }

            let mut map = output_box_imp.reset_sink_map.write().unwrap();
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

pub fn sink_changed_handler(
    output_box: Arc<SinkBox>,
    ir: SinkChanged,
    default_sink: String,
) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let output_box_imp = output_box.imp();
            let is_default = ir.sink.name == default_sink;
            let volume = ir.sink.volume.first().unwrap_or(&0_u32);
            let fraction = (*volume as f64 / 655.36).round();
            let percentage = (fraction).to_string() + "%";

            let list = output_box_imp.reset_sink_list.read().unwrap();
            let entry = list.get(&ir.sink.index);
            if entry.is_none() {
                return;
            }
            let imp = entry.unwrap().1.imp();
            if is_default {
                output_box_imp.reset_volume_percentage.set_text(&percentage);
                output_box_imp.reset_volume_slider.set_value(*volume as f64);
                output_box_imp.reset_default_sink.replace(ir.sink.clone());
                if ir.sink.muted {
                    output_box_imp
                        .reset_sink_mute
                        .set_icon_name("audio-volume-muted-symbolic");
                } else {
                    output_box_imp
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

pub fn input_stream_added_handler(output_box: Arc<SinkBox>, ir: InputStreamAdded) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let output_box_imp = output_box.imp();
            let mut list = output_box_imp.reset_input_stream_list.write().unwrap();
            let index = ir.stream.index;
            let input_stream = Arc::new(InputStreamEntry::new(output_box.clone(), ir.stream));
            let entry = Arc::new(ListEntry::new(&*input_stream));
            entry.set_activatable(false);
            list.insert(index, (entry.clone(), input_stream.clone()));
            output_box_imp.reset_input_streams.append(&*entry);
        });
    });
    true
}

pub fn input_stream_removed_handler(output_box: Arc<SinkBox>, ir: InputStreamRemoved) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let output_box_imp = output_box.imp();
            let mut list = output_box_imp.reset_input_stream_list.write().unwrap();
            let entry = list.remove(&ir.index);
            if entry.is_none() {
                return;
            }
            output_box_imp
                .reset_input_streams
                .remove(&*entry.unwrap().0);
        });
    });
    true
}

pub fn input_stream_changed_handler(output_box: Arc<SinkBox>, ir: InputStreamChanged) -> bool {
    let imp = output_box.imp();
    let alias: String;
    {
        let sink_list = imp.reset_sink_list.read().unwrap();
        if let Some(alias_opt) = sink_list.get(&ir.stream.sink_index) {
            alias = alias_opt.2.clone();
        } else {
            alias = String::from("");
        }
    }
    let sink_box = output_box.clone();
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let output_box = sink_box.clone();
            let output_box_imp = output_box.imp();
            let entry: Arc<InputStreamEntry>;
            {
                let list = output_box_imp.reset_input_stream_list.read().unwrap();
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
            let index = output_box_imp.reset_model_index.read().unwrap();
            let model_list = output_box_imp.reset_model_list.read().unwrap();
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
