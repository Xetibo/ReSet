use std::collections::HashMap;
use std::sync::RwLock;
use std::time::Duration;
use std::{cell::RefCell, sync::Arc, time::SystemTime};

use adw::prelude::{ButtonExt, CheckButtonExt, PreferencesRowExt, RangeExt};
use adw::{ActionRow, ComboRow, PreferencesGroup};
use dbus::arg::{Arg, Get};
use glib::Propagation;
use glib::{
    object::{IsA, IsClass},
    Object,
};
use gtk::{gio, Button, CheckButton, Label, Scale, StringList, TemplateChild};
use re_set_lib::audio::audio_structures::{TAudioObject, TAudioStreamObject};

use crate::components::base::error::ReSetError;
use crate::components::base::error_impl::ReSetErrorImpl;
use crate::components::base::list_entry::ListEntry;
use crate::components::utils::set_action_row_ellipsis;

use super::generic_audio_functions::refresh_default_audio_object;
use super::generic_utils::audio_dbus_call;

pub type AudioEntryMap<T> = Arc<RwLock<HashMap<u32, (Arc<ListEntry>, Arc<T>, String)>>>;
pub type AudioStreamEntryMap<T> = Arc<RwLock<HashMap<u32, (Arc<ListEntry>, Arc<T>)>>>;
pub type AudioMap = Arc<RwLock<HashMap<String, (u32, String)>>>;

pub trait TAudioBox<AudioBoxImpl> {
    fn box_imp(&self) -> &AudioBoxImpl;
}

#[allow(dead_code)]
pub trait TAudioBoxImpl<AObject, ENTRY, STREAMENTRY> {
    fn audio_object_row(&self) -> &TemplateChild<ActionRow>;
    fn cards_row(&self) -> &TemplateChild<ActionRow>;
    fn audio_object_dropdown(&self) -> &TemplateChild<ComboRow>;
    fn audio_object_mute(&self) -> &TemplateChild<Button>;
    fn volume_slider(&self) -> &TemplateChild<Scale>;
    fn volume_percentage(&self) -> &TemplateChild<Label>;
    fn audio_objects(&self) -> &TemplateChild<gtk::Box>;
    fn audio_object_stream_button(&self) -> &TemplateChild<ActionRow>;
    fn audio_object_streams(&self) -> &TemplateChild<gtk::Box>;
    fn cards_button(&self) -> &TemplateChild<ActionRow>;
    fn cards(&self) -> &TemplateChild<PreferencesGroup>;
    fn error(&self) -> &TemplateChild<ReSetError>;
    fn default_check_button(&self) -> Arc<CheckButton>;
    fn default_audio_object(&self) -> Arc<RefCell<AObject>>;
    fn audio_object_list(&self) -> &AudioEntryMap<ENTRY>;
    fn audio_object_stream_list(&self) -> &AudioStreamEntryMap<STREAMENTRY>;
    fn model_list(&self) -> Arc<RwLock<StringList>>;
    fn model_index(&self) -> Arc<RwLock<u32>>;
    fn source_map(&self) -> &AudioMap;
    fn volume_time_stamp(&self) -> &RefCell<Option<SystemTime>>;
    fn icons(&self) -> &AudioIcons;
}

pub trait TAudioEntry<TAudioEntryImpl>: IsClass + IsA<glib::Object> {
    fn entry_imp(&self) -> &TAudioEntryImpl;
}

pub trait TAudioEntryImpl<AudioObject: TAudioObject> {
    fn name(&self) -> &TemplateChild<ActionRow>;
    fn selected_audio_object(&self) -> &TemplateChild<CheckButton>;
    fn mute(&self) -> &TemplateChild<Button>;
    fn volume_slider(&self) -> &TemplateChild<Scale>;
    fn volume_percentage(&self) -> &TemplateChild<Label>;
    fn audio_object(&self) -> Arc<RefCell<AudioObject>>;
    fn volume_time_stamp(&self) -> &RefCell<Option<SystemTime>>;
    fn set_volume_fn(&self) -> &'static DBusFunction;
    fn set_audio_object_fn(&self) -> &'static DBusFunction;
    fn set_mute_fn(&self) -> &'static DBusFunction;
    fn icons(&self) -> &AudioIcons;
}

pub trait TAudioStream<TAudioStreamImpl>: IsClass + IsA<glib::Object> {
    fn entry_imp(&self) -> &TAudioStreamImpl;
}

pub trait TAudioStreamImpl<AudioObject: TAudioObject, StreamObject: TAudioStreamObject> {
    fn audio_object_selection(&self) -> &TemplateChild<ComboRow>;
    fn audio_object_mute(&self) -> &TemplateChild<Button>;
    fn volume_slider(&self) -> &TemplateChild<Scale>;
    fn volume_percentage(&self) -> &TemplateChild<Label>;
    fn stream_object(&self) -> Arc<RefCell<StreamObject>>;
    fn associated_audio_object(&self) -> Arc<RefCell<(u32, String)>>;
    fn volume_time_stamp(&self) -> &RefCell<Option<SystemTime>>;
    fn set_volume_fn(&self) -> &'static DBusFunction;
    fn set_audio_object_fn(&self) -> &'static DBusFunction;
    fn set_mute_fn(&self) -> &'static DBusFunction;
    fn icons(&self) -> &AudioIcons;
}

pub struct AudioIcons {
    pub muted: &'static str,
    pub active: &'static str,
}

pub struct DBusFunction {
    pub function: &'static str,
    pub error: &'static str,
}

pub fn new_entry<
    AudioObject: TAudioObject + Arg + for<'z> Get<'z> + Send + Sync + 'static,
    StreamObject: TAudioStreamObject + Arg + for<'z> Get<'z> + Send + Sync + 'static,
    AudioEntry: TAudioEntry<AudioEntryImpl>,
    AudioEntryImpl: TAudioEntryImpl<AudioObject>,
    AudioStream: TAudioStream<AudioStreamImpl>,
    AudioStreamImpl: TAudioStreamImpl<AudioObject, StreamObject>,
    AudioBox: TAudioBox<AudioBoxImpl> + ReSetErrorImpl + 'static,
    AudioBoxImpl: TAudioBoxImpl<AudioObject, AudioEntry, AudioStream>,
>(
    is_default: bool,
    check_group: Arc<CheckButton>,
    audio_object: AudioObject,
    reset_box: Arc<AudioBox>,
) -> Arc<AudioEntry> {
    let obj: Arc<AudioEntry> = Arc::new(Object::builder().build());
    // TODO use event callback for progress bar -> this is the "im speaking" indicator
    {
        let imp = obj.entry_imp();
        let slider_obj_ref = obj.clone();
        let mute_obj_ref = obj.clone();
        imp.name().set_title(audio_object.alias().clone().as_str());
        let name = Arc::new(audio_object.name().clone());
        let volume = audio_object.volume();
        let volume = volume.first().unwrap_or(&0_u32);
        let fraction = (*volume as f64 / 655.36).round();
        let percentage = (fraction).to_string() + "%";
        let output_box_slider = reset_box.clone();
        let output_box_ref = reset_box.clone();
        imp.volume_percentage().set_text(&percentage);
        imp.volume_slider().set_value(*volume as f64);
        imp.audio_object().replace(audio_object);
        imp.volume_slider()
            .connect_change_value(move |_, _, value| {
                let imp = slider_obj_ref.entry_imp();
                let fraction = (value / 655.36).round();
                let percentage = (fraction).to_string() + "%";
                imp.volume_percentage().set_text(&percentage);
                let sink = imp.audio_object();
                let sink = sink.borrow();
                let index = sink.index();
                let channels = sink.channels();
                {
                    let time = imp.volume_time_stamp();
                    let mut time = time.borrow_mut();
                    if time.is_some()
                        && time.unwrap().elapsed().unwrap() < Duration::from_millis(50)
                    {
                        return Propagation::Proceed;
                    }
                    *time = Some(SystemTime::now());
                }
                audio_dbus_call::<AudioBox, (), (u32, u16, u32)>(
                    output_box_slider.clone(),
                    (index, channels, value as u32),
                    imp.set_volume_fn(),
                );
                Propagation::Proceed
            });
        imp.selected_audio_object().set_group(Some(&*check_group));
        if is_default {
            imp.selected_audio_object().set_active(true);
        } else {
            imp.selected_audio_object().set_active(false);
        }

        let audio_object_fn = imp.set_audio_object_fn();
        imp.selected_audio_object().connect_toggled(move |button| {
            let output_box_ref = reset_box.clone();
            if button.is_active() {
                let name = name.clone();
                gio::spawn_blocking(move || {
                    let result = audio_dbus_call::<AudioBox, (AudioObject,), (&String,)>(
                        output_box_ref.clone(),
                        (&name,),
                        audio_object_fn,
                    );
                    if result.is_none() {
                        return;
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
                    >(result.unwrap().0, output_box_ref, true);
                });
            }
        });
        imp.mute().connect_clicked(move |_| {
            let imp = mute_obj_ref.entry_imp();
            let audio_object = imp.audio_object().clone();
            let mut audio_object = audio_object.borrow_mut();
            audio_object.toggle_muted();
            let icons = imp.icons();
            if audio_object.muted() {
                imp.mute().set_icon_name(icons.muted);
            } else {
                imp.mute().set_icon_name(icons.active);
            }
            audio_dbus_call::<AudioBox, (), (u32, bool)>(
                output_box_ref.clone(),
                (audio_object.index(), audio_object.muted()),
                imp.set_mute_fn(),
            );
        });
        set_action_row_ellipsis(imp.name().get());
    }
    obj
}
