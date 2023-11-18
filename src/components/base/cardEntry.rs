use std::time::Duration;

use adw::glib;
use adw::glib::Object;
use dbus::blocking::Connection;
use dbus::Error;
use glib::subclass::types::ObjectSubclassIsExt;
use glib::{clone, Cast};
use gtk::{gio, StringObject};
use ReSet_Lib::audio::audio::Card;

use super::cardEntryImpl;

glib::wrapper! {
    pub struct CardEntry(ObjectSubclass<cardEntryImpl::CardEntry>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl CardEntry {
    pub fn new(card: Card) -> Self {
        let entry: Self = Object::builder().build();
        {
            let imp = entry.imp();
            let mut map = imp.resetCardMap.borrow_mut();
            imp.resetCardName.set_text(&card.name);
            let mut i: u32 = 0;
            let mut index: u32 = 0;
            for profile in card.profiles.iter() {
                if profile.name == card.active_profile {
                    index = i;
                }
                imp.resetCardList.append(&profile.description);
                map.insert(
                    profile.description.clone(),
                    (card.index, profile.name.clone()),
                );
                i += 1;
            }
            imp.resetCardDropdown.set_selected(index);
            imp.resetCardDropdown
                .connect_selected_notify(clone!(@weak imp => move |dropdown| {
                    let selected = dropdown.selected_item();
                    if selected.is_none() {
                        return;
                    }
                    let selected = selected.unwrap();
                    let selected = selected.downcast_ref::<StringObject>().unwrap();
                    let selected = selected.string().to_string();
                    let map = imp.resetCardMap.borrow();
                    let (device_index, profile_name) = map.get(&selected).unwrap();
                    set_card_profile_of_device(*device_index, profile_name.clone());
                }));
        }
        entry
    }
}

fn set_card_profile_of_device(device_index: u32, profile_name: String) -> bool {
    gio::spawn_blocking(move || {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.xetibo.ReSet",
            "/org/xetibo/ReSet",
            Duration::from_millis(1000),
        );
        let _: Result<(), Error> = proxy.method_call(
            "org.xetibo.ReSet",
            "SetCardProfileOfDevice",
            (device_index, profile_name),
        );
    });
    true
}
