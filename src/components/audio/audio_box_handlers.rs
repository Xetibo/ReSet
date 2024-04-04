use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use adw::prelude::{ComboRowExt, PreferencesRowExt};
use dbus::arg::{Arg, Get};
use glib::{
    object::{Cast, IsA},
    ControlFlow, Propagation,
};
use gtk::{
    gio,
    prelude::{BoxExt, ButtonExt, CheckButtonExt, ListBoxRowExt, RangeExt},
    StringObject,
};
use re_set_lib::{
    audio::audio_structures::{TAudioObject, TAudioStreamObject},
    signals::{TAudioEventRemoved, TAudioObjectEvent, TAudioStreamEvent},
};

use crate::components::base::{error_impl::ReSetErrorImpl, list_entry::ListEntry};

use super::{
    audio_box_utils::{
        populate_audio_object_information, populate_cards, populate_streams,
        refresh_default_audio_object,
    },
    audio_entry::{
        new_entry, DBusFunction, TAudioBox, TAudioBoxImpl, TAudioEntry, TAudioEntryImpl,
        TAudioStream, TAudioStreamImpl,
    },
    audio_functions::new_stream_entry,
    audio_utils::audio_dbus_call,
};

pub fn mute_clicked_handler<
    AudioObject: TAudioObject,
    StreamObject: TAudioStreamObject,
    AudioEntry: TAudioEntry<AudioEntryImpl>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + ReSetErrorImpl + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
>(
    audio_box: Arc<AudioBox>,
    function: &'static DBusFunction,
) {
    let imp = audio_box.box_imp();
    let source = imp.default_audio_object();
    let mut source = source.borrow_mut();
    source.toggle_muted();
    let icons = imp.icons();
    let mute_button = imp.audio_object_mute();
    if source.muted() {
        mute_button.set_icon_name(icons.muted);
    } else {
        mute_button.set_icon_name(icons.active);
    }
    audio_dbus_call::<AudioBox, (), (u32, bool)>(
        audio_box.clone(),
        (source.index(), source.muted()),
        function,
    );
}

pub fn volume_slider_handler<
    AudioObject: TAudioObject,
    StreamObject: TAudioStreamObject,
    AudioEntry: TAudioEntry<AudioEntryImpl>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + ReSetErrorImpl + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
>(
    audio_box: Arc<AudioBox>,
    value: f64,
    function: &'static DBusFunction,
) -> Propagation {
    let imp = audio_box.box_imp();
    let fraction = (value / 655.36).round();
    let percentage = (fraction).to_string() + "%";
    imp.volume_percentage().set_text(&percentage);
    let source = imp.default_audio_object();
    let source = source.borrow();
    let index = source.index();
    let channels = source.channels();
    {
        let mut time = imp.volume_time_stamp().borrow_mut();
        if time.is_some() && time.unwrap().elapsed().unwrap() < Duration::from_millis(50) {
            return Propagation::Proceed;
        }
        *time = Some(SystemTime::now());
    }
    audio_dbus_call::<AudioBox, (), (u32, u16, u32)>(
        audio_box.clone(),
        (index, channels, value as u32),
        function,
    );
    Propagation::Proceed
}

pub fn dropdown_handler<
    AudioObject: TAudioObject + Send + Sync,
    StreamObject: TAudioStreamObject + Send + Sync,
    AudioEntry: TAudioEntry<AudioEntryImpl>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl> + IsA<gtk::Widget>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + ReSetErrorImpl + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
>(
    audio_box: Arc<AudioBox>,
    dropdown: &adw::ComboRow,
    function: &'static DBusFunction,
) -> ControlFlow {
    let source_box_imp = audio_box.box_imp();
    let source_box_ref = audio_box.clone();
    let selected = dropdown.selected_item();
    if selected.is_none() {
        return ControlFlow::Break;
    }
    let selected = selected.unwrap();
    let selected = selected.downcast_ref::<StringObject>().unwrap();
    let selected = selected.string().to_string();
    let source_map = source_box_imp.source_map();
    let source_map = source_map.read().unwrap();
    let source = source_map.get(&selected);
    if source.is_none() {
        return ControlFlow::Break;
    }
    let source = Arc::new(source.unwrap().1.clone());
    gio::spawn_blocking(move || {
        let result = audio_dbus_call::<AudioBox, (AudioObject,), (&String,)>(
            source_box_ref.clone(),
            (&source,),
            function,
        );
        if result.is_none() {
            return ControlFlow::Break;
        }
        refresh_default_audio_object::<
            AudioObject,
            StreamObject,
            AudioEntry,
            AudioEntryImpl,
            AudioStream,
            AudioStreamImpl,
            AudioBox,
            AudioBoxImpl,
        >(result.unwrap().0, source_box_ref.clone(), false);
        ControlFlow::Continue
    });
    ControlFlow::Continue
}

pub fn populate_audio_objects<
    AudioObject: TAudioObject + Arg + for<'z> Get<'z> + Send + Sync + 'static,
    StreamObject: TAudioStreamObject + Send + Sync + for<'z> Get<'z> + Arg + 'static,
    AudioEntry: TAudioEntry<AudioEntryImpl> + IsA<gtk::Widget>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl> + IsA<gtk::Widget>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + ReSetErrorImpl + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
>(
    audio_box: Arc<AudioBox>,
    audio_objects_function: &'static DBusFunction,
    default_audio_object_function: &'static DBusFunction,
    set_default_audio_object_function: &'static DBusFunction,
    get_audio_streams_function: &'static DBusFunction,
    set_audio_object_mute_function: &'static DBusFunction,
    set_audio_object_volume_function: &'static DBusFunction,
) {
    gio::spawn_blocking(move || {
        let sources = audio_dbus_call::<AudioBox, (Vec<AudioObject>,), ()>(
            audio_box.clone(),
            (),
            audio_objects_function,
        );
        if sources.is_none() {
            return;
        }
        let audio_objects = sources.unwrap().0;
        {
            let imp = audio_box.box_imp();
            let list = imp.model_list();
            let list = list.write().unwrap();
            let map = imp.source_map();
            let mut map = map.write().unwrap();
            let model_index = imp.model_index();
            let mut model_index = model_index.write().unwrap();

            let audio_object = audio_dbus_call::<AudioBox, (AudioObject,), ()>(
                audio_box.clone(),
                (),
                default_audio_object_function,
            );
            if let Some(audio_object) = audio_object {
                imp.default_audio_object().replace(audio_object.0);
            }

            for audio_object in audio_objects.iter() {
                let alias = audio_object.alias();
                list.append(&alias);
                map.insert(
                    alias.clone(),
                    (audio_object.index(), audio_object.name().clone()),
                );
                *model_index += 1;
            }
        }
        populate_streams::<
            AudioObject,
            StreamObject,
            AudioEntry,
            AudioEntryImpl,
            AudioStream,
            AudioStreamImpl,
            AudioBox,
            AudioBoxImpl,
        >(audio_box.clone(), get_audio_streams_function);
        populate_cards::<
            AudioObject,
            StreamObject,
            AudioEntry,
            AudioEntryImpl,
            AudioStream,
            AudioStreamImpl,
            AudioBox,
            AudioBoxImpl,
        >(audio_box.clone());
        populate_audio_object_information::<
            AudioObject,
            StreamObject,
            AudioEntry,
            AudioEntryImpl,
            AudioStream,
            AudioStreamImpl,
            AudioBox,
            AudioBoxImpl,
        >(
            audio_box,
            audio_objects,
            set_default_audio_object_function,
            set_audio_object_volume_function,
            set_audio_object_mute_function,
        );
    });
}

pub fn object_added_handler<
    AudioObject: TAudioObject,
    StreamObject: TAudioStreamObject,
    AudioEntry: TAudioEntry<AudioEntryImpl> + IsA<gtk::Widget>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl> + IsA<gtk::Widget>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + ReSetErrorImpl + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
    Event: TAudioObjectEvent<AudioObject>,
>(
    audio_box: Arc<AudioBox>,
    ir: Event,
) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let audio_box = audio_box.clone();
            let source_box_imp = audio_box.box_imp();
            let object = ir.object_ref();
            let object_index = object.index();
            let alias = object.alias().clone();
            let name = object.name().clone();
            let mut is_default = false;
            if source_box_imp.default_audio_object().borrow().name() == object.name() {
                is_default = true;
            }
            let source_entry = new_entry::<
                AudioObject,
                StreamObject,
                AudioEntry,
                AudioEntryImpl,
                AudioStream,
                AudioStreamImpl,
                AudioBox,
                AudioBoxImpl,
            >(
                is_default,
                source_box_imp.default_check_button().clone(),
                ir.object(),
                audio_box.clone(),
            );
            let source_clone = source_entry.clone();
            let entry = Arc::new(ListEntry::new(&*source_entry));
            entry.set_activatable(false);
            let list = source_box_imp.audio_object_list();
            let mut list = list.write().unwrap();
            list.insert(object_index, (entry.clone(), source_clone, alias.clone()));
            source_box_imp.audio_objects().append(&*entry);
            let map = source_box_imp.source_map();
            let mut map = map.write().unwrap();
            let index = source_box_imp.model_index();
            let mut index = index.write().unwrap();
            let model_list = source_box_imp.model_list();
            let model_list = model_list.write().unwrap();
            // TODO: make this work generic!
            if model_list.string(*index - 1) == Some("Monitor of Dummy Output".into()) {
                model_list.append(&alias);
                model_list.remove(*index - 1);
                map.insert(alias, (object_index, name));
                source_box_imp.audio_object_dropdown().set_selected(0);
            } else {
                model_list.append(&alias);
                map.insert(alias.clone(), (object_index, name));
                // TODO: make this work generic!
                if alias == "Monitor of Dummy Output" {
                    source_box_imp.audio_object_dropdown().set_selected(0);
                }
                *index += 1;
            }
        });
    });
    true
}

pub fn object_changed_handler<
    AudioObject: TAudioObject,
    StreamObject: TAudioStreamObject,
    AudioEntry: TAudioEntry<AudioEntryImpl> + IsA<gtk::Widget>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl> + IsA<gtk::Widget>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + ReSetErrorImpl + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
    Event: TAudioObjectEvent<AudioObject>,
>(
    audio_box: Arc<AudioBox>,
    ir: Event,
    function: &'static DBusFunction,
) -> bool {
    let source = audio_dbus_call::<AudioBox, (String,), ()>(audio_box.clone(), (), function);
    if source.is_none() {
        return false;
    }
    let default_source = source.unwrap().0;
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let audio_box = audio_box.clone();
            let box_imp = audio_box.box_imp();
            let object = ir.object_ref();
            let is_default = object.name() == default_source;
            let volume = object.volume();
            let volume = volume.first().unwrap_or(&0_u32);
            let fraction = (*volume as f64 / 655.36).round();
            let percentage = (fraction).to_string() + "%";

            let list = box_imp.audio_object_list();
            let list = list.read().unwrap();
            let entry = list.get(&object.index());
            if entry.is_none() {
                return;
            }
            let imp = entry.unwrap().1.entry_imp();
            if is_default {
                box_imp.volume_percentage().set_text(&percentage);
                box_imp.volume_slider().set_value(*volume as f64);
                box_imp.default_audio_object().replace(ir.object());
                let icons = imp.icons();
                let mute_button = imp.mute();
                if object.muted() {
                    mute_button.set_icon_name(icons.muted);
                } else {
                    mute_button.set_icon_name(icons.active);
                }
                imp.selected_audio_object().set_active(true);
            } else {
                imp.selected_audio_object().set_active(false);
            }
            imp.name().set_title(object.alias().as_str());
            imp.volume_percentage().set_text(&percentage);
            imp.volume_slider().set_value(*volume as f64);
            let mute_button = imp.mute();
            let icons = imp.icons();
            if object.muted() {
                mute_button.set_icon_name(icons.muted);
            } else {
                mute_button.set_icon_name(icons.active);
            }
        });
    });
    true
}

pub fn object_removed_handler<
    AudioObject: TAudioObject,
    StreamObject: TAudioStreamObject,
    AudioEntry: TAudioEntry<AudioEntryImpl> + IsA<gtk::Widget>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl> + IsA<gtk::Widget>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + ReSetErrorImpl + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
    Event: TAudioEventRemoved,
>(
    audio_box: Arc<AudioBox>,
    ir: Event,
) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let audio_box = audio_box.clone();
            let box_imp = audio_box.box_imp();
            let entry: Option<(Arc<ListEntry>, Arc<AudioEntry>, String)>;
            {
                let list = box_imp.audio_object_list();
                let mut list = list.write().unwrap();
                entry = list.remove(&ir.index());
                if entry.is_none() {
                    return;
                }
            }
            box_imp.audio_objects().remove(&*entry.clone().unwrap().0);
            let map = box_imp.source_map();
            let mut map = map.write().unwrap();
            let alias = entry.unwrap().2;
            map.remove(&alias);
            let index = box_imp.model_index();
            let mut index = index.write().unwrap();
            let model_list = box_imp.model_list();
            let model_list = model_list.write().unwrap();

            if *index == 1 {
                // TODO: ensure dummy output and input are mentioned
                model_list.append("Dummy");
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

pub fn audio_stream_added_handler<
    AudioObject: TAudioObject,
    StreamObject: TAudioStreamObject,
    AudioEntry: TAudioEntry<AudioEntryImpl> + IsA<gtk::Widget>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl> + IsA<gtk::Widget>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + ReSetErrorImpl + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
    Event: TAudioStreamEvent<StreamObject>,
>(
    audio_box: Arc<AudioBox>,
    ir: Event,
) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let audio_box = audio_box.clone();
            let imp = audio_box.box_imp();
            let list = imp.audio_object_stream_list();
            let mut list = list.write().unwrap();
            let index = ir.stream_ref().index();
            let stream = new_stream_entry::<
                AudioObject,
                StreamObject,
                AudioEntry,
                AudioEntryImpl,
                AudioStream,
                AudioStreamImpl,
                AudioBox,
                AudioBoxImpl,
            >(audio_box.clone(), ir.stream());
            let entry = Arc::new(ListEntry::new(&*stream));
            entry.set_activatable(false);
            list.insert(index, (entry.clone(), stream.clone()));
            imp.audio_object_streams().append(&*entry);
        });
    });
    true
}

pub fn audio_stream_changed_handler<
    AudioObject: TAudioObject,
    StreamObject: TAudioStreamObject,
    AudioEntry: TAudioEntry<AudioEntryImpl> + IsA<gtk::Widget>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl> + IsA<gtk::Widget>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + ReSetErrorImpl + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
    Event: TAudioStreamEvent<StreamObject>,
>(
    audio_box: Arc<AudioBox>,
    ir: Event,
) -> bool {
    let imp = audio_box.box_imp();
    let alias: String;
    {
        let stream = ir.stream_ref();
        let object_list = imp.audio_object_list();
        let object_list = object_list.read().unwrap();
        if let Some(alias_opt) = object_list.get(&stream.audio_object_index()) {
            alias = alias_opt.2.clone();
        } else {
            alias = String::from("");
        }
    }
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let audio_box = audio_box.clone();
            let box_imp = audio_box.box_imp();
            let entry: Arc<AudioStream>;
            let stream = ir.stream_ref();
            {
                let list = box_imp.audio_object_stream_list();
                let list = list.read().unwrap();
                let entry_opt = list.get(&stream.index());
                if entry_opt.is_none() {
                    return;
                }
                entry = entry_opt.unwrap().1.clone();
            }
            let imp = entry.entry_imp();
            let mute_button = imp.audio_object_mute();
            let icons = imp.icons();
            if stream.muted() {
                mute_button.set_icon_name(icons.muted);
            } else {
                mute_button.set_icon_name(icons.active);
            }
            let name = stream.application_name().clone() + ": " + stream.name().as_str();
            imp.audio_object_selection().set_title(name.as_str());
            let volume = stream.volume();
            let volume = volume.first().unwrap_or(&0_u32);
            let fraction = (*volume as f64 / 655.36).round();
            let percentage = (fraction).to_string() + "%";
            imp.volume_percentage().set_text(&percentage);
            imp.volume_slider().set_value(*volume as f64);
            let index = box_imp.model_index();
            let index = index.read().unwrap();
            let model_list = box_imp.model_list();
            let model_list = model_list.read().unwrap();
            for entry in 0..*index {
                if model_list.string(entry) == Some(alias.clone().into()) {
                    imp.audio_object_selection().set_selected(entry);
                    break;
                }
            }
        });
    });
    true
}

pub fn audio_stream_removed_handler<
    AudioObject: TAudioObject,
    StreamObject: TAudioStreamObject,
    AudioEntry: TAudioEntry<AudioEntryImpl> + IsA<gtk::Widget>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl> + IsA<gtk::Widget>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + ReSetErrorImpl + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
    Event: TAudioEventRemoved,
>(
    audio_box: Arc<AudioBox>,
    ir: Event,
) -> bool {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let imp = audio_box.box_imp();
            let list = imp.audio_object_stream_list();
            let mut list = list.write().unwrap();
            let entry = list.remove(&ir.index());
            if entry.is_none() {
                return;
            }
            imp.audio_object_streams().remove(&*entry.unwrap().0);
        });
    });
    true
}
