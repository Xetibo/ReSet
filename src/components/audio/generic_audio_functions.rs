use std::sync::Arc;

use adw::traits::ComboRowExt;
use gtk::prelude::{ButtonExt, CheckButtonExt, RangeExt};
use re_set_lib::audio::audio_structures::{TAudioObject, TAudioStreamObject};

use super::generic_entry::{
    TAudioBox, TAudioBoxImpl, TAudioEntry, TAudioEntryImpl, TAudioStream, TAudioStreamImpl,
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
