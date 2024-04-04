use std::sync::Arc;

use adw::{prelude::ComboRowExt, prelude::PreferencesGroupExt};
use dbus::{
    arg::{Arg, Get, ReadAll},
    blocking::Connection,
    message::SignalArgs,
    Path,
};
use glib::{object::IsA, Variant};
use gtk::{
    gio,
    prelude::{ActionableExt, BoxExt, ButtonExt, CheckButtonExt, ListBoxRowExt, RangeExt},
};
use re_set_lib::{
    audio::audio_structures::{Card, TAudioObject, TAudioStreamObject},
    signals::{TAudioEventRemoved, TAudioObjectEvent, TAudioStreamEvent},
};

use crate::components::{
    base::{card_entry::CardEntry, error_impl::ReSetErrorImpl, list_entry::ListEntry},
    utils::{create_dropdown_label_factory, set_combo_row_ellipsis, BASE, DBUS_PATH},
};

use super::{
    audio_box_handlers::{
        audio_stream_added_handler, audio_stream_changed_handler, audio_stream_removed_handler,
        dropdown_handler, mute_clicked_handler, object_added_handler, object_changed_handler,
        object_removed_handler, volume_slider_handler,
    },
    audio_const::GETCARDS,
    audio_entry::{
        new_entry, DBusFunction, TAudioBox, TAudioBoxImpl, TAudioEntry, TAudioEntryImpl,
        TAudioStream, TAudioStreamImpl,
    },
    audio_functions::new_stream_entry,
    audio_utils::audio_dbus_call,
};

pub fn setup_audio_box_callbacks<
    AudioObject: TAudioObject,
    StreamObject: TAudioStreamObject,
    AudioEntry: TAudioEntry<AudioEntryImpl>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl>,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
>(
    audio_box: &mut AudioBox,
) {
    let imp = audio_box.box_imp();
    let object_row = imp.audio_object_row();
    object_row.set_activatable(true);
    object_row.set_action_name(Some("navigation.push"));
    object_row.set_action_target_value(Some(&Variant::from("devices")));

    let cards_row = imp.cards_row();
    cards_row.set_activatable(true);
    cards_row.set_action_name(Some("navigation.push"));
    cards_row.set_action_target_value(Some(&Variant::from("profileConfiguration")));

    let stream_button = imp.audio_object_stream_button();
    stream_button.set_activatable(true);
    stream_button.set_action_name(Some("navigation.pop"));

    let cards_back_button = imp.cards_button();
    cards_back_button.set_activatable(true);
    cards_back_button.set_action_name(Some("navigation.pop"));

    let audio_object_dropdown = imp.audio_object_dropdown();
    audio_object_dropdown.set_factory(Some(&create_dropdown_label_factory()));
    set_combo_row_ellipsis(audio_object_dropdown.get());
}

pub fn populate_cards<
    AudioObject: TAudioObject,
    StreamObject: TAudioStreamObject,
    AudioEntry: TAudioEntry<AudioEntryImpl>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + ReSetErrorImpl + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
>(
    source_box: Arc<AudioBox>,
) {
    gio::spawn_blocking(move || {
        let source_box_ref = source_box.clone();
        let cards =
            audio_dbus_call::<AudioBox, (Vec<Card>,), ()>(source_box.clone(), (), &GETCARDS);
        if cards.is_none() {
            return;
        }
        let cards = cards.unwrap().0;
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let imp = source_box_ref.box_imp();
                for card in cards {
                    imp.cards().add(&CardEntry::new(card));
                }
            });
        });
    });
}

pub fn populate_streams<
    AudioObject: TAudioObject + Sync + Send + 'static,
    StreamObject: TAudioStreamObject + Arg + for<'z> Get<'z> + Sync + Send + 'static,
    AudioEntry: TAudioEntry<AudioEntryImpl>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl> + IsA<gtk::Widget>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + ReSetErrorImpl + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
>(
    audio_box: Arc<AudioBox>,
    function: &'static DBusFunction,
) {
    let audio_box_ref = audio_box.clone();
    gio::spawn_blocking(move || {
        let streams =
            audio_dbus_call::<AudioBox, (Vec<StreamObject>,), ()>(audio_box.clone(), (), function);
        if streams.is_none() {
            return;
        }
        let streams = streams.unwrap().0;
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let imp = audio_box_ref.box_imp();
                let mut list = imp.audio_object_stream_list().write().unwrap();
                for stream in streams {
                    let index = stream.index();
                    let stream = new_stream_entry::<
                        AudioObject,
                        StreamObject,
                        AudioEntry,
                        AudioEntryImpl,
                        AudioStream,
                        AudioStreamImpl,
                        AudioBox,
                        AudioBoxImpl,
                    >(audio_box.clone(), stream);
                    let stream_clone = stream.clone();
                    let entry = Arc::new(ListEntry::new(&*stream));
                    entry.set_activatable(false);
                    list.insert(index, (entry.clone(), stream_clone));
                    imp.audio_object_streams().append(&*entry);
                }
            });
        });
    });
}

pub fn refresh_default_audio_object<
    AudioObject: TAudioObject + Sync + Send + 'static,
    StreamObject: TAudioStreamObject + Arg + for<'z> Get<'z> + Sync + Send + 'static,
    AudioEntry: TAudioEntry<AudioEntryImpl>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl> + IsA<gtk::Widget>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + ReSetErrorImpl + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
>(
    new_audio_object: AudioObject,
    audio_box: Arc<AudioBox>,
    entry: bool,
) {
    let volume = *new_audio_object.volume().first().unwrap_or(&0_u32);
    let fraction = (volume as f64 / 655.36).round();
    let percentage = (fraction).to_string() + "%";
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let imp = audio_box.box_imp();
            if !entry {
                let list = imp.audio_object_list().read().unwrap();
                let entry = list.get(&new_audio_object.index());
                if entry.is_none() {
                    return;
                }
                let entry_imp = entry.unwrap().1.entry_imp();
                entry_imp.selected_audio_object().set_active(true);
            } else {
                let model_list = imp.model_list();
                let model_list = model_list.read().unwrap();
                for entry in 0..*imp.model_index().read().unwrap() {
                    if model_list.string(entry) == Some(new_audio_object.alias().clone().into()) {
                        imp.audio_object_dropdown().set_selected(entry);
                        break;
                    }
                }
            }
            imp.volume_percentage().set_text(&percentage);
            imp.volume_slider().set_value(volume as f64);
            let icons = imp.icons();
            let mute_button = imp.audio_object_mute();
            if new_audio_object.muted() {
                mute_button.set_icon_name(icons.muted);
            } else {
                mute_button.set_icon_name(icons.active);
            }
            imp.default_audio_object().replace(new_audio_object);
        });
    });
}

pub fn populate_audio_object_information<
    AudioObject: TAudioObject + Sync + Send + 'static + Arg + for<'z> Get<'z>,
    StreamObject: TAudioStreamObject + Arg + for<'z> Get<'z> + Sync + Send + 'static,
    AudioEntry: TAudioEntry<AudioEntryImpl> + IsA<gtk::Widget>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl> + IsA<gtk::Widget>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + ReSetErrorImpl + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
>(
    audio_box: Arc<AudioBox>,
    audio_objects: Vec<AudioObject>,
    dropdown_function: &'static DBusFunction,
    change_volume_function: &'static DBusFunction,
    mute_function: &'static DBusFunction,
) {
    glib::spawn_future(async move {
        glib::idle_add_once(move || {
            let source_box_ref_slider = audio_box.clone();
            let source_box_ref_toggle = audio_box.clone();
            let source_box_ref_mute = audio_box.clone();
            let imp = audio_box.box_imp();
            let default_sink = imp.default_audio_object().clone();
            let source = default_sink.borrow();

            let icons = imp.icons();
            let mute_button = imp.audio_object_mute();
            if source.muted() {
                mute_button.set_icon_name(icons.muted);
            } else {
                mute_button.set_icon_name(icons.active);
            }

            let volume = source.volume();
            let volume = volume.first().unwrap_or(&0_u32);
            let fraction = (*volume as f64 / 655.36).round();
            let percentage = (fraction).to_string() + "%";
            imp.volume_percentage().set_text(&percentage);
            imp.volume_slider().set_value(*volume as f64);
            let list = imp.audio_object_list();
            let mut list = list.write().unwrap();
            for source in audio_objects {
                let index = source.index();
                let alias = source.alias().clone();
                let mut is_default = false;
                if imp.default_audio_object().borrow().name() == source.name() {
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
                    imp.default_check_button().clone(),
                    source,
                    audio_box.clone(),
                );
                let source_clone = source_entry.clone();
                let entry = Arc::new(ListEntry::new(&*source_entry));
                entry.set_activatable(false);
                list.insert(index, (entry.clone(), source_clone, alias));
                imp.audio_objects().append(&*entry);
            }
            let list = imp.model_list();
            let list = list.read().unwrap();
            imp.audio_object_dropdown().set_model(Some(&*list));
            let name = imp.default_audio_object();
            let name = name.borrow();

            let index = imp.model_index();
            let index = index.read().unwrap();
            let model_list = imp.model_list();
            let model_list = model_list.read().unwrap();
            for entry in 0..*index {
                if model_list.string(entry) == Some(name.alias().clone().into()) {
                    imp.audio_object_dropdown().set_selected(entry);
                    break;
                }
            }
            imp.audio_object_dropdown()
                .connect_selected_notify(move |dropdown| {
                    dropdown_handler(source_box_ref_toggle.clone(), dropdown, dropdown_function);
                });
            imp.volume_slider()
                .connect_change_value(move |_, _, value| {
                    volume_slider_handler(
                        source_box_ref_slider.clone(),
                        value,
                        change_volume_function,
                    )
                });

            imp.audio_object_mute().connect_clicked(move |_| {
                mute_clicked_handler(source_box_ref_mute.clone(), mute_function);
            });
        });
    });
}

pub fn start_audio_box_listener<
    AudioObject: TAudioObject,
    StreamObject: TAudioStreamObject,
    AudioEntry: TAudioEntry<AudioEntryImpl> + IsA<gtk::Widget>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl> + IsA<gtk::Widget>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + ReSetErrorImpl + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
    ObjectAdded: TAudioObjectEvent<AudioObject> + ReadAll + SignalArgs,
    ObjectChanged: TAudioObjectEvent<AudioObject> + ReadAll + SignalArgs,
    ObjectRemoved: TAudioEventRemoved + ReadAll + SignalArgs,
    StreamAdded: TAudioStreamEvent<StreamObject> + ReadAll + SignalArgs,
    StreamChanged: TAudioStreamEvent<StreamObject> + ReadAll + SignalArgs,
    StreamRemoved: TAudioEventRemoved + ReadAll + SignalArgs,
>(
    conn: Connection,
    source_box: Arc<AudioBox>,
    get_default_name_function: &'static DBusFunction,
) -> Connection {
    // TODO: make the failed logs generically sound -> deynamic output for both
    let object_added =
        ObjectAdded::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();
    let object_changed =
        ObjectChanged::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();
    let object_removed =
        ObjectRemoved::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();
    let stream_added =
        StreamAdded::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();
    let stream_changed =
        StreamChanged::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();
    let stream_removed =
        StreamRemoved::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();

    let object_added_box = source_box.clone();
    let object_removed_box = source_box.clone();
    let object_changed_box = source_box.clone();
    let stream_added_box = source_box.clone();
    let stream_removed_box = source_box.clone();
    let stream_changed_box = source_box.clone();

    let res = conn.add_match(object_added, move |ir: ObjectAdded, _, _| {
        object_added_handler::<
            AudioObject,
            StreamObject,
            AudioEntry,
            AudioEntryImpl,
            AudioStream,
            AudioStreamImpl,
            AudioBox,
            AudioBoxImpl,
            ObjectAdded,
        >(object_added_box.clone(), ir)
    });
    if res.is_err() {
        // TODO: handle this with the log/error macro
        println!("fail on source add event");
        return conn;
    }

    let res = conn.add_match(object_changed, move |ir: ObjectChanged, _, _| {
        // source_changed_handler(source_changed_box.clone(), ir)
        object_changed_handler::<
            AudioObject,
            StreamObject,
            AudioEntry,
            AudioEntryImpl,
            AudioStream,
            AudioStreamImpl,
            AudioBox,
            AudioBoxImpl,
            ObjectChanged,
        >(object_changed_box.clone(), ir, get_default_name_function)
    });
    if res.is_err() {
        println!("fail on source change event");
        return conn;
    }

    let res = conn.add_match(object_removed, move |ir: ObjectRemoved, _, _| {
        // source_removed_handler(source_removed_box.clone(), ir)
        object_removed_handler::<
            AudioObject,
            StreamObject,
            AudioEntry,
            AudioEntryImpl,
            AudioStream,
            AudioStreamImpl,
            AudioBox,
            AudioBoxImpl,
            ObjectRemoved,
        >(object_removed_box.clone(), ir)
    });
    if res.is_err() {
        println!("fail on source remove event");
        return conn;
    }

    let res = conn.add_match(stream_added, move |ir: StreamAdded, _, _| {
        audio_stream_added_handler::<
            AudioObject,
            StreamObject,
            AudioEntry,
            AudioEntryImpl,
            AudioStream,
            AudioStreamImpl,
            AudioBox,
            AudioBoxImpl,
            StreamAdded,
        >(stream_added_box.clone(), ir)
    });
    if res.is_err() {
        println!("fail on output stream add event");
        return conn;
    }

    let res = conn.add_match(stream_changed, move |ir: StreamChanged, _, _| {
        audio_stream_changed_handler::<
            AudioObject,
            StreamObject,
            AudioEntry,
            AudioEntryImpl,
            AudioStream,
            AudioStreamImpl,
            AudioBox,
            AudioBoxImpl,
            StreamChanged,
        >(stream_changed_box.clone(), ir)
    });
    if res.is_err() {
        println!("fail on output stream change event");
        return conn;
    }

    let res = conn.add_match(stream_removed, move |ir: StreamRemoved, _, _| {
        audio_stream_removed_handler::<
            AudioObject,
            StreamObject,
            AudioEntry,
            AudioEntryImpl,
            AudioStream,
            AudioStreamImpl,
            AudioBox,
            AudioBoxImpl,
            StreamRemoved,
        >(stream_removed_box.clone(), ir)
    });
    if res.is_err() {
        println!("fail on output stream remove event");
        return conn;
    }

    conn
}
