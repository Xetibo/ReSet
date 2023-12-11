use std::time::Duration;

use adw::glib;
use adw::glib::Object;
use adw::prelude::{ComboRowExt, PreferencesRowExt};
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::types::ObjectSubclassIsExt;
use glib::{clone, Cast};
use gtk::{gio, StringList, StringObject};

use components::utils::create_dropdown_label_factory;
use re_set_lib::audio::audio_structures::Card;

use crate::components;

use super::card_entry_impl;

glib::wrapper! {
    pub struct CardEntry(ObjectSubclass<card_entry_impl::CardEntry>)
    @extends adw::ComboRow, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable, adw::PreferencesRow;
}

impl CardEntry {
    pub fn new(card: Card) -> Self {
        let entry: CardEntry = Object::builder().build();
        {
            let imp = entry.imp();
            let mut map = imp.reset_card_map.borrow_mut();
            entry.set_title(&card.name);
            let mut index: u32 = 0;
            let list = StringList::new(&[]);
            for (i, profile) in (0_u32..).zip(card.profiles.iter()) {
                if profile.name == card.active_profile {
                    index = i;
                }
                list.append(&profile.description);
                map.insert(
                    profile.description.clone(),
                    (card.index, profile.name.clone()),
                );
            }
            entry.set_model(Some(&list));
            entry.set_selected(index);
            entry.set_use_subtitle(true);
            entry.connect_selected_notify(clone!(@weak imp => move |dropdown| {
                let selected = dropdown.selected_item();
                if selected.is_none() {
                    return;
                }
                let selected = selected.unwrap();
                let selected = selected.downcast_ref::<StringObject>().unwrap();
                let selected = selected.string().to_string();
                let map = imp.reset_card_map.borrow();
                let (device_index, profile_name) = map.get(&selected).unwrap();
                set_card_profile_of_device(*device_index, profile_name.clone());
            }));
            entry.set_factory(Some(&create_dropdown_label_factory()));
        }
        entry
    }
}

fn set_card_profile_of_device(device_index: u32, profile_name: String) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSetDaemon",
            "/org/Xetibo/ReSetDaemon",
            Duration::from_millis(1000),
        );
        let _: Result<(), Error> = proxy.method_call(
            "org.Xetibo.ReSetAudio",
            "SetCardProfileOfDevice",
            (device_index, profile_name),
        );
    });
    true
}
