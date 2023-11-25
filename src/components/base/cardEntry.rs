use std::time::Duration;

use adw::glib;
use adw::glib::Object;
use adw::prelude::{ComboRowExt, PreferencesRowExt};
use dbus::blocking::Connection;
use dbus::Error;
use glib::{Cast, clone, ObjectExt};
use glib::subclass::types::ObjectSubclassIsExt;
use gtk::{Align, gio, SignalListItemFactory, StringList, StringObject};
use gtk::prelude::{GObjectPropertyExpressionExt, ListItemExt, WidgetExt};
use ReSet_Lib::audio::audio::Card;

use super::cardEntryImpl;

glib::wrapper! {
    pub struct CardEntry(ObjectSubclass<cardEntryImpl::CardEntry>)
    @extends adw::ComboRow, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable, adw::PreferencesRow;
}

impl CardEntry {
    pub fn new(card: Card) -> Self {
        let entry: CardEntry = Object::builder().build();
        {
            let imp = entry.imp();
            let mut map = imp.resetCardMap.borrow_mut();
            entry.set_title(&card.name);
            let mut i: u32 = 0;
            let mut index: u32 = 0;
            let list = StringList::new(&[]);
            for profile in card.profiles.iter() {
                if profile.name == card.active_profile {
                    index = i;
                }
                list.append(&profile.description);
                map.insert(
                    profile.description.clone(),
                    (card.index, profile.name.clone()),
                );
                i += 1;
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
                    let map = imp.resetCardMap.borrow();
                    let (device_index, profile_name) = map.get(&selected).unwrap();
                    set_card_profile_of_device(*device_index, profile_name.clone());
                }));

            let factory = &SignalListItemFactory::new();
            factory.connect_setup(|_, item| {
                let item = item.downcast_ref::<gtk::ListItem>().unwrap();
                let label = gtk::Label::new(None);
                label.set_halign(Align::Start);
                item.property_expression("item")
                    .chain_property::<StringObject>("string")
                    .bind(&label, "label", gtk::Widget::NONE);
                item.set_child(Some(&label));
            });
            entry.set_factory(Some(factory));
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
