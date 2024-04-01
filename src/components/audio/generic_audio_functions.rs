use std::{sync::Arc, time::Duration};

use adw::traits::ComboRowExt;
use dbus::{
    arg::{Arg, Get},
    blocking::Connection,
    Error,
};
use gtk::{
    gio,
    prelude::{ButtonExt, CheckButtonExt, RangeExt},
};
use re_set_lib::audio::audio_structures::AudioObject;

use crate::components::{
    base::error_impl::{show_error, ReSetErrorImpl},
    utils::{AUDIO, BASE, DBUS_PATH},
};

use super::generic_entry::{Audio, AudioBox, AudioBoxImpl, AudioImpl, DBusFunction};

pub fn set_volume<T: ReSetErrorImpl + 'static>(
    value: f64,
    index: u32,
    channels: u16,
    reset_box: Arc<T>,
    function: &'static DBusFunction,
) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
        let res: Result<(), Error> =
            proxy.method_call(AUDIO, function.function, (index, channels, value as u32));
        if res.is_err() {
            // TODO: also log this with LOG/ERROR
            show_error::<T>(reset_box.clone(), function.error);
        }
    });
    true
}

pub fn toggle_audio_object_mute<T: ReSetErrorImpl + 'static>(
    index: u32,
    muted: bool,
    input_box: Arc<T>,
    function: &'static DBusFunction,
) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
        let res: Result<(), Error> = proxy.method_call(AUDIO, function.function, (index, muted));
        if res.is_err() {
            // TODO: also log this with LOG/ERROR
            show_error::<T>(input_box.clone(), function.error);
        }
    });
    true
}

pub fn set_default_audio_object<T, R>(
    name: Arc<String>,
    input_box: Arc<T>,
    function: &'static DBusFunction,
) -> Option<R>
where
    T: ReSetErrorImpl + 'static,
    R: Arg + for<'z> Get<'z>,
{
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(R,), Error> = proxy.method_call(AUDIO, function.function, (name.as_str(),));
    if res.is_err() {
        show_error::<T>(input_box.clone(), function.error);
        return None;
    }
    Some(res.unwrap().0)
}

pub fn refresh_default_audio_object<
    A: AudioBox<BoxImpl> + Send + Sync + 'static,
    OBJ: AudioObject + Send + Sync + 'static,
    Entry: Audio<OBJ, EntryImpl>,
    EntryImpl: AudioImpl<OBJ>,
    BoxImpl: AudioBoxImpl<OBJ, Entry, EntryImpl>,
>(
    new_audio_object: OBJ,
    reset_box: Arc<A>,
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
