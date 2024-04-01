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
use re_set_lib::audio::audio_structures::AudioObject;

use crate::components::base::error::ReSetError;
use crate::components::base::error_impl::ReSetErrorImpl;
use crate::components::base::list_entry::ListEntry;
use crate::components::utils::set_action_row_ellipsis;

use super::generic_audio_functions::{
    refresh_default_audio_object, set_default_audio_object, set_volume, toggle_audio_object_mute,
};

pub trait Audio<T: AudioObject, IMP: AudioImpl<T>>: IsClass + IsA<glib::Object> {
    fn entry_imp(&self) -> &IMP;
}

pub trait AudioBox<AudioBoxImpl> {
    fn box_imp(&self) -> &AudioBoxImpl;
}

pub type AudioEntryMap<T> = Arc<RwLock<HashMap<u32, (Arc<ListEntry>, Arc<T>, String)>>>;
#[allow(dead_code)]
pub type AudioStreamEntryMap<T> = Arc<RwLock<HashMap<u32, (Arc<ListEntry>, Arc<T>)>>>;
pub type AudioMap = Arc<RwLock<HashMap<String, (u32, String)>>>;

#[allow(dead_code)]
pub trait AudioBoxImpl<OBJ, ENTRY, STREAMENTRY> {
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
    fn default_audio_object(&self) -> Arc<RefCell<OBJ>>;
    fn audio_object_list(&self) -> &AudioEntryMap<ENTRY>;
    // fn audio_object_stream_list(&self) -> AudioStreamEntryMap<STREAMENTRY>;
    fn model_list(&self) -> Arc<RwLock<StringList>>;
    fn model_index(&self) -> Arc<RwLock<u32>>;
    fn source_map(&self) -> &AudioMap;
    fn volume_time_stamp(&self) -> &RefCell<Option<SystemTime>>;
}

pub trait AudioImpl<T: AudioObject> {
    fn name(&self) -> &TemplateChild<ActionRow>;
    fn selected_audio_object(&self) -> &TemplateChild<CheckButton>;
    fn mute(&self) -> &TemplateChild<Button>;
    fn volume_slider(&self) -> &TemplateChild<Scale>;
    fn volume_percentage(&self) -> &TemplateChild<Label>;
    fn audio_object(&self) -> Arc<RefCell<T>>;
    fn volume_time_stamp(&self) -> &RefCell<Option<SystemTime>>;
    fn set_volume_fn(&self) -> (&'static str, &'static str);
    fn set_audio_object_fn(&self) -> (&'static str, &'static str);
    fn set_mute_fn(&self) -> (&'static str, &'static str);
}

// pub trait AudioObject {
//     fn alias(&self) -> String;
//     fn name(&self) -> String;
//     fn volume(&self) -> Vec<u32>;
//     fn index(&self) -> u32;
//     fn channels(&self) -> u16;
//     fn muted(&self) -> bool;
//     fn toggle_muted(&mut self);
//     fn active() -> i32;
// }

pub fn new_entry<
    AObject: AudioObject + Arg + for<'z> Get<'z> + Send + Sync + 'static,
    ABox: AudioBox<BoxImpl> + ReSetErrorImpl + 'static,
    Entry: Audio<AObject, EntryImpl>,
    EntryImpl: AudioImpl<AObject>,
    BoxImpl: AudioBoxImpl<AObject, Entry, EntryImpl>,
>(
    is_default: bool,
    check_group: Arc<CheckButton>,
    audio_object: AObject,
    reset_box: Arc<ABox>,
) -> Arc<Entry> {
    let obj: Arc<Entry> = Arc::new(Object::builder().build());
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
                set_volume(
                    value,
                    index,
                    channels,
                    output_box_slider.clone(),
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
                    // TODO: make this generic as well
                    let result = set_default_audio_object::<ABox, AObject>(
                        name,
                        output_box_ref.clone(),
                        audio_object_fn,
                    );
                    if result.is_none() {
                        return;
                    }

                    refresh_default_audio_object::<ABox, AObject, Entry, EntryImpl, BoxImpl>(
                        result.unwrap(),
                        output_box_ref,
                        true,
                    );
                });
            }
        });
        imp.mute().connect_clicked(move |_| {
            let imp = mute_obj_ref.entry_imp();
            let audio_object = imp.audio_object().clone();
            let mut audio_object = audio_object.borrow_mut();
            audio_object.toggle_muted();
            if audio_object.muted() {
                imp.mute().set_icon_name("audio-volume-muted-symbolic");
            } else {
                imp.mute().set_icon_name("audio-volume-high-symbolic");
            }
            toggle_audio_object_mute::<ABox>(
                audio_object.index(),
                audio_object.muted(),
                output_box_ref.clone(),
                imp.set_mute_fn(),
            );
        });
        set_action_row_ellipsis(imp.name().get());
    }
    obj
}
