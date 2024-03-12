use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::components::base::error_impl::show_error;
use crate::components::utils::set_action_row_ellipsis;
use adw::glib::Object;
use adw::prelude::{ButtonExt, CheckButtonExt, PreferencesRowExt, RangeExt};
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::types::ObjectSubclassIsExt;
use glib::{clone, Propagation};
use gtk::{gio, CheckButton};
use re_set_lib::audio::audio_structures::Source;

use crate::components::utils::{AUDIO, BASE, DBUS_PATH};

use super::source_box::SourceBox;
use super::source_box_utils::refresh_default_source;
use super::source_entry_impl;

glib::wrapper! {
    pub struct SourceEntry(ObjectSubclass<source_entry_impl::SourceEntry>)
    @extends adw::PreferencesGroup, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for SourceEntry {}
unsafe impl Sync for SourceEntry {}

impl SourceEntry {
    pub fn new(
        is_default: bool,
        check_group: Arc<CheckButton>,
        source: Source,
        input_box: Arc<SourceBox>,
    ) -> Self {
        let obj: Self = Object::builder().build();
        // TODO use event callback for progress bar -> this is the "im speaking" indicator
        {
            let imp = obj.imp();
            imp.reset_source_name
                .set_title(source.alias.clone().as_str());
            let name = Arc::new(source.name.clone());
            let volume = source.volume.first().unwrap_or(&0_u32);
            let fraction = (*volume as f64 / 655.36).round();
            let percentage = (fraction).to_string() + "%";
            let input_box_slider = input_box.clone();
            let input_box_ref = input_box.clone();
            imp.reset_volume_percentage.set_text(&percentage);
            imp.reset_volume_slider.set_value(*volume as f64);
            imp.source.replace(source);
            imp.reset_volume_slider.connect_change_value(
                clone!(@weak imp => @default-return Propagation::Stop, move |_, _, value| {
                    let fraction = (value / 655.36).round();
                    let percentage = (fraction).to_string() + "%";
                    imp.reset_volume_percentage.set_text(&percentage);
                    let source = imp.source.borrow();
                    let index = source.index;
                    let channels = source.channels;
                    {
                        let mut time = imp.volume_time_stamp.borrow_mut();
                        if time.is_some()
                            && time.unwrap().elapsed().unwrap() < Duration::from_millis(50)
                        {
                            return Propagation::Proceed;
                        }
                        *time = Some(SystemTime::now());
                    }
                    set_source_volume(value, index, channels, input_box_slider.clone());
                    Propagation::Proceed
                }),
            );
            imp.reset_selected_source.set_group(Some(&*check_group));
            if is_default {
                imp.reset_selected_source.set_active(true);
            } else {
                imp.reset_selected_source.set_active(false);
            }
            imp.reset_selected_source.connect_toggled(move |button| {
                let input_box = input_box.clone();
                if button.is_active() {
                    let name = name.clone();
                    gio::spawn_blocking(move || {
                        let result = set_default_source(name, input_box.clone());
                        if result.is_none() {
                            return;
                        }
                        refresh_default_source(result.unwrap(), input_box, true);
                    });
                }
            });
            imp.reset_source_mute
                .connect_clicked(clone!(@weak imp => move |_| {
                    let mut source = imp.source.borrow_mut();
                    source.muted = !source.muted;
                    if source.muted {
                        imp.reset_source_mute
                           .set_icon_name("microphone-disabled-symbolic");
                    } else {
                        imp.reset_source_mute
                           .set_icon_name("audio-input-microphone-symbolic");
                    }
                    toggle_source_mute(source.index, source.muted, input_box_ref.clone());
                }));
            set_action_row_ellipsis(imp.reset_source_name.get());
        }
        obj
    }
}

pub fn set_source_volume(value: f64, index: u32, channels: u16, input_box: Arc<SourceBox>) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
        let res: Result<(), Error> =
            proxy.method_call(AUDIO, "SetSourceVolume", (index, channels, value as u32));
        if res.is_err() {
            // TODO: also log this with LOG/ERROR
            show_error::<SourceBox>(input_box.clone(), "Failed to set source volume");
        }
    });
    true
}

pub fn toggle_source_mute(index: u32, muted: bool, input_box: Arc<SourceBox>) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
        let res: Result<(), Error> = proxy.method_call(AUDIO, "SetSourceMute", (index, muted));
        if res.is_err() {
            show_error::<SourceBox>(input_box.clone(), "Failed to mute source");
        }
    });
    true
}

pub fn set_default_source(name: Arc<String>, input_box: Arc<SourceBox>) -> Option<Source> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<(Source,), Error> =
        proxy.method_call(AUDIO, "SetDefaultSource", (name.as_str(),));
    if res.is_err() {
        show_error::<SourceBox>(input_box.clone(), "Failed to set default source");
        return None;
    }
    Some(res.unwrap().0)
}
