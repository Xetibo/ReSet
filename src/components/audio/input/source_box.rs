use re_set_lib::signals::{
    OutputStreamAdded, OutputStreamChanged, OutputStreamRemoved, SourceAdded, SourceChanged,
    SourceRemoved,
};
use std::sync::Arc;

use adw::glib::Object;
use adw::prelude::{ComboRowExt, ListBoxRowExt};
use dbus::blocking::Connection;
use dbus::message::SignalArgs;
use dbus::Path;
use glib::subclass::prelude::ObjectSubclassIsExt;
use glib::Variant;
use gtk::gio;
use gtk::prelude::ActionableExt;

use crate::components::base::error::{self};
use crate::components::base::error_impl::ReSetErrorImpl;
use crate::components::audio::input::source_box_impl;
use crate::components::utils::{
    create_dropdown_label_factory, set_combo_row_ellipsis, BASE, DBUS_PATH,
};

use super::source_box_handlers::{
    output_stream_added_handler, output_stream_changed_handler, output_stream_removed_handler,
    source_added_handler, source_changed_handler, source_removed_handler,
};
use super::source_box_utils::{
    get_default_source, get_sources, populate_cards, populate_outputstreams,
    populate_source_information,
};

glib::wrapper! {
    pub struct SourceBox(ObjectSubclass<source_box_impl::SourceBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for SourceBox {}
unsafe impl Sync for SourceBox {}

impl ReSetErrorImpl for SourceBox {
    fn error(&self) -> &gtk::subclass::prelude::TemplateChild<error::ReSetError> {
        &self.imp().error
    }
}

impl SourceBox {
    pub fn new() -> Self {
        let obj: Self = Object::builder().build();
        {
            let imp = obj.imp();
            let mut model_index = imp.reset_model_index.write().unwrap();
            *model_index = 0;
        }
        obj
    }

    pub fn setup_callbacks(&self) {
        let self_imp = self.imp();
        self_imp.reset_source_row.set_activatable(true);
        self_imp
            .reset_source_row
            .set_action_name(Some("navigation.push"));
        self_imp
            .reset_source_row
            .set_action_target_value(Some(&Variant::from("sources")));
        self_imp.reset_cards_row.set_activatable(true);
        self_imp
            .reset_cards_row
            .set_action_name(Some("navigation.push"));
        self_imp
            .reset_cards_row
            .set_action_target_value(Some(&Variant::from("profileConfiguration")));

        self_imp.reset_output_stream_button.set_activatable(true);
        self_imp
            .reset_output_stream_button
            .set_action_name(Some("navigation.pop"));

        self_imp.reset_input_cards_back_button.set_activatable(true);
        self_imp
            .reset_input_cards_back_button
            .set_action_name(Some("navigation.pop"));

        self_imp
            .reset_source_dropdown
            .set_factory(Some(&create_dropdown_label_factory()));
        set_combo_row_ellipsis(self_imp.reset_source_dropdown.get());
    }
}

impl Default for SourceBox {
    fn default() -> Self {
        Self::new()
    }
}

pub fn populate_sources(source_box: Arc<SourceBox>) {
    gio::spawn_blocking(move || {
        let sources = get_sources(source_box.clone());
        {
            let source_box_imp = source_box.imp();
            let list = source_box_imp.reset_model_list.write().unwrap();
            let mut map = source_box_imp.reset_source_map.write().unwrap();
            let mut model_index = source_box_imp.reset_model_index.write().unwrap();
            source_box_imp
                .reset_default_source
                .replace(get_default_source(source_box.clone()));
            for source in sources.iter() {
                list.append(&source.alias);
                map.insert(source.alias.clone(), (source.index, source.name.clone()));
                *model_index += 1;
            }
        }

        populate_outputstreams(source_box.clone());
        populate_cards(source_box.clone());
        populate_source_information(source_box, sources);
    });
}

pub fn start_source_box_listener(conn: Connection, source_box: Arc<SourceBox>) -> Connection {
    let source_added =
        SourceAdded::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();
    let source_removed =
        SourceRemoved::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();
    let source_changed =
        SourceChanged::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH))).static_clone();
    let output_stream_added =
        OutputStreamAdded::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH)))
            .static_clone();
    let output_stream_removed =
        OutputStreamRemoved::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH)))
            .static_clone();
    let output_stream_changed =
        OutputStreamChanged::match_rule(Some(&BASE.into()), Some(&Path::from(DBUS_PATH)))
            .static_clone();

    let source_added_box = source_box.clone();
    let source_removed_box = source_box.clone();
    let source_changed_box = source_box.clone();
    let output_stream_added_box = source_box.clone();
    let output_stream_removed_box = source_box.clone();
    let output_stream_changed_box = source_box.clone();

    let res = conn.add_match(source_added, move |ir: SourceAdded, _, _| {
        source_added_handler(source_added_box.clone(), ir)
    });
    if res.is_err() {
        // TODO: handle this with the log/error macro
        println!("fail on source add event");
        return conn;
    }

    let res = conn.add_match(source_removed, move |ir: SourceRemoved, _, _| {
        source_removed_handler(source_removed_box.clone(), ir)
    });
    if res.is_err() {
        println!("fail on source remove event");
        return conn;
    }

    let res = conn.add_match(source_changed, move |ir: SourceChanged, _, _| {
        source_changed_handler(source_changed_box.clone(), ir)
    });
    if res.is_err() {
        println!("fail on source change event");
        return conn;
    }

    let res = conn.add_match(output_stream_added, move |ir: OutputStreamAdded, _, _| {
        output_stream_added_handler(output_stream_added_box.clone(), ir)
    });
    if res.is_err() {
        println!("fail on output stream add event");
        return conn;
    }

    let res = conn.add_match(
        output_stream_changed,
        move |ir: OutputStreamChanged, _, _| {
            output_stream_changed_handler(output_stream_changed_box.clone(), ir)
        },
    );
    if res.is_err() {
        println!("fail on output stream change event");
        return conn;
    }

    let res = conn.add_match(
        output_stream_removed,
        move |ir: OutputStreamRemoved, _, _| {
            output_stream_removed_handler(output_stream_removed_box.clone(), ir)
        },
    );
    if res.is_err() {
        println!("fail on output stream remove event");
        return conn;
    }

    conn
}
