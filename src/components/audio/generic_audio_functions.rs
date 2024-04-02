use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use adw::{prelude::ComboRowExt, prelude::PreferencesRowExt};
use glib::{object::Cast, Object, Propagation};
use gtk::{
    prelude::{ButtonExt, CheckButtonExt, RangeExt},
    StringObject,
};
use re_set_lib::audio::audio_structures::{TAudioObject, TAudioStreamObject};

use crate::components::{
    base::error_impl::ReSetErrorImpl,
    utils::{create_dropdown_label_factory, set_combo_row_ellipsis},
};

use super::{
    generic_entry::{
        TAudioBox, TAudioBoxImpl, TAudioEntry, TAudioEntryImpl, TAudioStream, TAudioStreamImpl,
    },
    generic_utils::audio_dbus_call,
};

pub fn refresh_default_audio_object<
    AudioObject: TAudioObject + Send + Sync + 'static,
    StreamObject: TAudioStreamObject + Send + Sync + 'static,
    AudioEntry: TAudioEntry<AudioEntryImpl>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + Send + Sync + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
>(
    new_audio_object: AudioObject,
    reset_box: Arc<AudioBox>,
    entry: bool,
) {
    let volume = *new_audio_object.volume().first().unwrap_or(&0_u32);
    let fraction = (volume as f64 / 655.36).round();
    let percentage = (fraction).to_string() + "%";
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let imp = reset_box.box_imp();
            if !entry {
                let list = imp.audio_object_list();
                let list = list.read().unwrap();
                let entry = list.get(&new_audio_object.index());
                if entry.is_none() {
                    return;
                }
                let entry_imp = entry.unwrap().1.entry_imp();
                entry_imp.selected_audio_object().set_active(true);
            } else {
                let index = imp.model_index();
                let index = index.read().unwrap();
                let model_list = imp.model_list();
                let model_list = model_list.read().unwrap();
                for entry in 0..*index {
                    if model_list.string(entry) == Some(new_audio_object.alias().clone().into()) {
                        imp.audio_object_dropdown().set_selected(entry);
                        break;
                    }
                }
            }
            imp.volume_percentage().set_text(&percentage);
            imp.volume_slider().set_value(volume as f64);
            let icons = imp.icons();
            if new_audio_object.muted() {
                imp.audio_object_mute().set_icon_name(icons.muted);
            } else {
                imp.audio_object_mute().set_icon_name(icons.active);
            }
            imp.default_audio_object().replace(new_audio_object);
        });
    });
}

pub fn new_stream_entry<
    AudioObject: TAudioObject + Send + Sync + 'static,
    StreamObject: TAudioStreamObject + Send + Sync + 'static,
    AudioEntry: TAudioEntry<AudioEntryImpl>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + ReSetErrorImpl + Send + Sync + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
>(
    audio_box: Arc<AudioBox>,
    stream: StreamObject,
) -> Arc<AudioStream> {
    let obj: Arc<AudioStream> = Arc::new(Object::builder().build());
    // TODO use event callback for progress bar -> this is the "im speaking" indicator
    let output_box_mute_ref = audio_box.clone();
    let output_box_volume_ref = audio_box.clone();
    let output_box_sink_ref = audio_box.clone();
    let entry_mute_ref = obj.clone();
    let entry_volume_ref = obj.clone();
    let entry_sink_ref = obj.clone();
    {
        let index = stream.audio_object_index();
        let box_imp = audio_box.box_imp();
        let imp = obj.entry_imp();
        let icons = box_imp.icons();
        if stream.muted() {
            imp.audio_object_mute().set_icon_name(icons.muted);
        } else {
            imp.audio_object_mute().set_icon_name(icons.active);
        }
        let name = stream.application_name().clone() + ": " + stream.name().as_str();
        imp.audio_object_selection().set_title(name.as_str());
        imp.audio_object_selection()
            .set_factory(Some(&create_dropdown_label_factory()));
        set_combo_row_ellipsis(imp.audio_object_selection().get());
        let volume = stream.volume();
        let volume = volume.first().unwrap_or(&0_u32);
        let fraction = (*volume as f64 / 655.36).round();
        let percentage = (fraction).to_string() + "%";
        imp.volume_percentage().set_text(&percentage);
        imp.volume_slider().set_value(*volume as f64);
        imp.stream_object().replace(stream);
        {
            let sink = box_imp.default_audio_object();
            let sink = sink.borrow();
            imp.associated_audio_object()
                .replace((sink.index(), sink.name().clone()));
        }
        imp.volume_slider()
            .connect_change_value(move |_, _, value| {
                let imp = entry_volume_ref.entry_imp();
                let fraction = (value / 655.36).round();
                let percentage = (fraction).to_string() + "%";
                imp.volume_percentage().set_text(&percentage);
                let stream = imp.stream_object();
                let mut stream_opt = stream.try_borrow();
                while stream_opt.is_err() {
                    stream_opt = stream.try_borrow();
                }
                let stream = stream_opt.unwrap();
                let index = stream.index();
                let channels = stream.channels();
                {
                    let mut time = imp.volume_time_stamp().borrow_mut();
                    if time.is_some()
                        && time.unwrap().elapsed().unwrap() < Duration::from_millis(50)
                    {
                        return Propagation::Proceed;
                    }
                    *time = Some(SystemTime::now());
                }
                audio_dbus_call::<AudioBox, (), (u32, u16, u32)>(
                    output_box_volume_ref.clone(),
                    (index, channels, value as u32),
                    imp.set_volume_fn(),
                );
                Propagation::Proceed
            });
        {
            let list = box_imp.model_list();
            let list = list.read().unwrap();
            imp.audio_object_selection().set_model(Some(&*list));
            let sink_list = box_imp.audio_object_list().read().unwrap();
            let name = sink_list.get(&index);
            let index = box_imp.model_index();
            let index = index.read().unwrap();
            if let Some(name) = name {
                for entry in 0..*index {
                    if list.string(entry) == Some(name.2.clone().into()) {
                        imp.audio_object_selection().set_selected(entry);
                        break;
                    }
                }
            } else {
                let name = box_imp.default_audio_object();
                let mut name_opt = name.try_borrow();
                while name_opt.is_err() {
                    name_opt = name.try_borrow();
                }
                let name = name_opt.unwrap();
                for entry in 0..*index {
                    if list.string(entry) == Some(name.alias().into()) {
                        imp.audio_object_selection().set_selected(entry);
                        break;
                    }
                }
            }
        }
        imp.audio_object_selection()
            .connect_selected_notify(move |dropdown| {
                let imp = entry_sink_ref.entry_imp();
                let box_imp = output_box_sink_ref.box_imp();
                let selected = dropdown.selected_item();
                if selected.is_none() {
                    return;
                }
                let selected = selected.unwrap();
                let selected = selected.downcast_ref::<StringObject>().unwrap();
                let selected = selected.string().to_string();
                let sink = box_imp.source_map().read().unwrap();
                let sink = sink.get(&selected);
                if sink.is_none() {
                    return;
                }
                let stream = imp.stream_object();
                let mut stream_opt = stream.try_borrow();
                while stream_opt.is_err() {
                    stream_opt = stream.try_borrow();
                }
                let stream = stream_opt.unwrap();
                let sink = sink.unwrap().0;
                audio_dbus_call::<AudioBox, (), (u32, u32)>(
                    output_box_sink_ref.clone(),
                    (stream.index(), sink),
                    imp.set_audio_object_fn(),
                );
            });
        imp.audio_object_mute().connect_clicked(move |_| {
            let imp = entry_mute_ref.entry_imp();
            let stream = imp.stream_object().clone();
            let mut stream_opt = stream.try_borrow_mut();
            while stream_opt.is_err() {
                stream_opt = stream.try_borrow_mut();
            }
            let mut stream = stream_opt.unwrap();
            stream.toggle_muted();
            let icons = imp.icons();
            let muted = stream.muted();
            if muted {
                imp.audio_object_mute().set_icon_name(icons.muted);
            } else {
                imp.audio_object_mute().set_icon_name(icons.active);
            }
            audio_dbus_call::<AudioBox, (), (u32, bool)>(
                output_box_mute_ref.clone(),
                (stream.index(), muted),
                imp.set_mute_fn(),
            );
        });
    }
    obj
}
