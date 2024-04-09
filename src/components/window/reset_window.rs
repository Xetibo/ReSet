use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use adw::glib::clone;
use adw::subclass::prelude::ObjectSubclassIsExt;
use adw::BreakpointCondition;
use glib::Object;
use gtk::gio::ActionEntry;
use gtk::{
    gio, AccessibleRole, Align, Application, FlowBox, FlowBoxChild, Frame, ListBoxRow, Orientation,
    StateFlags,
};
use gtk::{prelude::*, DirectionType};
use re_set_lib::utils::plugin_setup::FRONTEND_PLUGINS;

use crate::components::base::setting_box::SettingBox;
use crate::components::base::utils::{Listeners, Position};
use crate::components::plugin::function::PluginSidebarInfo;
use crate::components::utils::get_capabilities;
use crate::components::window::handle_sidebar_click::*;
use crate::components::window::reset_window_impl;
use crate::components::window::sidebar_entry::SidebarEntry;

use super::consts::{
    AUDIO_SIDEBAR, BLUETOOTH_SIDEBAR, CONNECTIVITY_SIDEBAR, SINK_SIDEBAR, SOURCE_SIDEBAR,
    WIFI_SIDEBAR,
};

glib::wrapper! {
    pub struct ReSetWindow(ObjectSubclass<reset_window_impl::ReSetWindow>)
        @extends adw::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

unsafe impl Send for ReSetWindow {}

unsafe impl Sync for ReSetWindow {}

impl ReSetWindow {
    pub fn new(app: &Application) -> Rc<Self> {
        app.set_accels_for_action("win.search", &["<Ctrl>F"]);
        app.set_accels_for_action("win.close", &["<Ctrl>Q"]);
        app.set_accels_for_action("win.about", &["<Ctrl>H"]);
        // implemented when a proper movement method is found
        // app.set_accels_for_action("win.up", &["<Ctrl>K"]);
        // app.set_accels_for_action("win.right", &["<Ctrl>L"]);
        // app.set_accels_for_action("win.down", &["<Ctrl>J"]);
        // app.set_accels_for_action("win.left", &["<Ctrl>H"]);
        let mut window: Rc<Self> = Rc::new(Object::builder().property("application", app).build());
        window = setup_callback(window);
        window
    }

    pub fn handle_dynamic_sidebar(&self) {
        let self_imp = self.imp();
        self_imp
            .reset_sidebar_breakpoint
            .set_condition(BreakpointCondition::parse("max-width: 860sp").as_ref().ok());
        self_imp.reset_sidebar_breakpoint.add_setter(
            &Object::from(self_imp.reset_overlay_split_view.get()),
            "collapsed",
            &true.to_value(),
        );
        self_imp.reset_sidebar_breakpoint.add_setter(
            &Object::from(self_imp.reset_sidebar_toggle.get()),
            "visible",
            &true.to_value(),
        );
    }

    pub fn filter_list(&self) {
        let text = self.imp().reset_search_entry.text().to_string();
        for (main_entry, sub_entriess) in self.imp().sidebar_entries.borrow().iter() {
            if text.is_empty() {
                main_entry.set_visible(true);
                for sub_entry in sub_entriess {
                    sub_entry.set_visible(true);
                }
                continue;
            }
            if main_entry
                .imp()
                .name
                .borrow()
                .to_lowercase()
                .contains(&text.to_lowercase())
            {
                main_entry.set_visible(true);
            } else {
                main_entry.set_visible(false);
            }
            for sub_entry in sub_entriess {
                if sub_entry
                    .imp()
                    .name
                    .borrow()
                    .to_lowercase()
                    .contains(&text.to_lowercase())
                {
                    sub_entry.set_visible(true);
                    main_entry.set_visible(true);
                } else {
                    sub_entry.set_visible(false);
                }
            }
        }
    }

    pub fn toggle_sidebar(&self) {
        if self.imp().reset_overlay_split_view.shows_sidebar() {
            self.imp().reset_overlay_split_view.set_show_sidebar(false);
        } else {
            self.imp().reset_overlay_split_view.set_show_sidebar(true);
        }
    }

    pub fn setup_sidebar_entries(&self) {
        let self_imp = self.imp();
        let capabilities = get_capabilities();
        let wifi = capabilities.contains(&"WiFi".to_string());
        let bluetooth = capabilities.contains(&"Bluetooth".to_string());
        let audio = capabilities.contains(&"Audio".to_string());
        self_imp.capabilities.set(wifi, bluetooth, audio);

        let mut sidebar_list = Vec::new();

        if wifi || bluetooth {
            sidebar_list.push(CONNECTIVITY_SIDEBAR);
        }
        if wifi {
            sidebar_list.push(WIFI_SIDEBAR);
        };
        if bluetooth {
            sidebar_list.push(BLUETOOTH_SIDEBAR);
        };
        if audio {
            sidebar_list.push(AUDIO_SIDEBAR);
            sidebar_list.push(SINK_SIDEBAR);
            sidebar_list.push(SOURCE_SIDEBAR);
        }

        let mut plugin_sidebar_list = vec![];
        unsafe {
            for plugin in FRONTEND_PLUGINS.iter() {
                let plugin_capabilities = &plugin.capabilities;
                let (sidebar_info, plugin_boxes) = (plugin.frontend_data)();
                let listeners = self_imp.listeners.clone();
                (plugin.frontend_startup)();

                let mut found = false;
                    dbg!(&capabilities);
                for capability in plugin_capabilities {
                    dbg!(&capability);
                    if capabilities.contains(&capability.to_string()) {
                        found = true;
                        break;
                    }
                }
                if !found {
                    continue;
                }
                let event = Rc::new(
                    move |reset_main: FlowBox,
                          position: Rc<RefCell<Position>>,
                          boxes: Vec<gtk::Box>| {
                        if handle_init(
                            listeners.clone(),
                            position,
                            Position::Custom(String::from(sidebar_info.name)),
                        ) {
                            return;
                        }
                        reset_main.remove_all();
                        for plugin_box in &boxes {
                            let frame =
                                wrap_in_flow_box_child(SettingBox::new(&plugin_box.clone()));
                            reset_main.insert(&frame, -1);
                        }
                        reset_main.set_max_children_per_line(boxes.len() as u32);
                    },
                );

                plugin_sidebar_list.push(PluginSidebarInfo {
                    name: sidebar_info.name,
                    icon_name: sidebar_info.icon_name,
                    parent: sidebar_info.parent,
                    click_event: event,
                    plugin_boxes,
                });
            }
        }

        HANDLE_VOLUME_CLICK(
            &self_imp.capabilities,
            self_imp.listeners.clone(),
            self_imp.reset_main.clone(),
            self_imp.position.clone(),
        );

        self_imp
            .reset_sidebar_list
            .connect_row_activated(clone!(@ weak self_imp => move |_, _| {
                self_imp.reset_search_entry.set_text("");
            }));
        // TODO: refactor this
        let mut i = 0;
        for info in sidebar_list {
            if info.parent.is_none() && i != 0 {
                self_imp.reset_sidebar_list.insert(&create_separator(), i);
                i += 1;
            }
            let entry = SidebarEntry::new(&info);
            self_imp.reset_sidebar_list.insert(&entry, i);
            i += 1;
        }

        for info in plugin_sidebar_list {
            if info.parent.is_none() && i != 0 {
                self_imp.reset_sidebar_list.insert(&create_separator(), i);
                i += 1;
            }
            let entry = SidebarEntry::new(&info);
            self_imp.reset_sidebar_list.insert(&entry, i);
            i += 1;
        }
    }

    pub fn setup_shortcuts(&self) {
        let search_action = ActionEntry::builder("search")
            .activate(move |window: &Self, _, _| {
                let imp = window.imp();
                if !imp.reset_overlay_split_view.shows_sidebar() {
                    imp.reset_overlay_split_view.set_show_sidebar(true);
                }
                window.imp().reset_search_entry.grab_focus();
            })
            .build();

        let close_action = ActionEntry::builder("close")
            .activate(move |window: &Self, _, _| {
                window.close();
            })
            .build();

        let error_popup_action = ActionEntry::builder("show_error")
            .activate(move |window: &Self, _, _| {
                window.imp().error_popup.popup();
            })
            .build();

        let error_popdown_action = ActionEntry::builder("hide_error")
            .activate(move |window: &Self, _, _| {
                window.imp().error_popup.popdown();
            })
            .build();

        let vim_up = ActionEntry::builder("up")
            .activate(move |window: &Self, _, _| {
                window.child_focus(DirectionType::Up);
            })
            .build();

        let vim_right = ActionEntry::builder("right")
            .activate(move |window: &Self, _, _| {
                window.child_focus(DirectionType::Right);
            })
            .build();

        let vim_down = ActionEntry::builder("down")
            .activate(move |window: &Self, _, _| {
                window.child_focus(DirectionType::Down);
            })
            .build();

        let vim_left = ActionEntry::builder("left")
            .activate(move |window: &Self, _, _| {
                window.child_focus(DirectionType::Left);
            })
            .build();

        // let clear_initial = ActionEntry::builder("clear_initial")
        //     .activate(move |window: &Self, _, _| {
        //         let imp = window.imp();
        //         for (_, subentries) in imp.sidebar_entries.borrow().iter() {
        //             for subentry in subentries {
        //                 if &*subentry.imp().name.borrow() == "Output" {
        //                     subentry.set_state_flags(StateFlags::SELECTED, false);
        //                 }
        //             }
        //         }
        //     })
        //     .build();

        let about_action = ActionEntry::builder("about")
            .activate(move |window: &ReSetWindow, _, _| {
                let dialog = adw::AboutWindow::builder()
                    .application_name("ReSet")
                    .application_icon("ReSet")
                    .developer_name("Xetibo")
                    .license("GPL-3.0")
                    .license_type(gtk::License::Gpl30)
                    .website("https://github.com/Xetibo/ReSet")
                    .issue_url("https://github.com/Xetibo/ReSet/issues")
                    .version("1.0.0")
                    .transient_for(window)
                    .modal(true)
                    .copyright("© 2022-2023 Xetibo")
                    .developers(vec!["DashieTM".to_string(), "Takotori".to_string()])
                    .designers(vec!["DashieTM".to_string(), "Takotori".to_string()])
                    .build();
                // window.imp().reset_popover_menu.popdown();
                dialog.present();
            })
            .build();

        self.add_action_entries([
            search_action,
            close_action,
            about_action,
            vim_up,
            vim_right,
            vim_down,
            vim_left,
            error_popup_action,
            error_popdown_action,
        ]);
    }
}

fn setup_callback(window: Rc<ReSetWindow>) -> Rc<ReSetWindow> {
    let self_imp = window.imp();
    let activated_ref = window.clone();
    let search_ref = window.clone();
    let toggle_ref = window.clone();
    let close_ref = window.clone();

    self_imp
        .reset_search_entry
        .connect_search_changed(move |_| {
            search_ref.filter_list();
        });

    self_imp.reset_sidebar_toggle.connect_clicked(move |_| {
        toggle_ref.toggle_sidebar();
    });

    self_imp
        .reset_sidebar_list
        .connect_row_activated(move |_, y| {
            let imp = activated_ref.imp();
            let result = y.downcast_ref::<SidebarEntry>().unwrap();
            {
                let mut default_entry = imp.default_entry.borrow_mut();
                if default_entry.is_some() {
                    default_entry
                        .clone()
                        .unwrap()
                        .set_state_flags(StateFlags::NORMAL, true);
                    *default_entry = None;
                }
            }
            let click_event = result.imp().on_click_event.borrow();
            if let Some(event) = click_event.on_click_event {
                event(
                    &imp.capabilities,
                    imp.listeners.clone(),
                    imp.reset_main.get(),
                    imp.position.clone(),
                );
            } else {
                let event = click_event.on_plugin_click_event.clone();
                event(
                    imp.reset_main.get(),
                    imp.position.clone(),
                    result.imp().plugin_boxes.borrow().clone(),
                );
            }
        });

    self_imp.reset_close.connect_clicked(move |_| {
        close_ref.close();
    });
    window
}

pub fn create_separator() -> ListBoxRow {
    let separator: gtk::Separator = gtk::Separator::builder()
        .margin_bottom(3)
        .margin_top(3)
        .orientation(Orientation::Horizontal)
        .accessible_role(AccessibleRole::Separator)
        .can_focus(false)
        .build();
    ListBoxRow::builder()
        .child(&separator)
        .selectable(false)
        .activatable(false)
        .can_target(false)
        .accessible_role(AccessibleRole::Separator)
        .build()
}

fn handle_init(
    listeners: Arc<Listeners>,
    position: Rc<RefCell<Position>>,
    clicked_position: Position,
) -> bool {
    {
        let mut pos_borrow = position.borrow_mut();
        if *pos_borrow == clicked_position {
            return true;
        }
        *pos_borrow = clicked_position;
    }
    listeners.stop_network_listener();
    listeners.stop_audio_listener();
    listeners.stop_bluetooth_listener();
    false
}

fn wrap_in_flow_box_child(widget: SettingBox) -> FlowBoxChild {
    let frame = Frame::new(None);
    frame.set_child(Some(&widget));
    frame.add_css_class("resetSettingFrame");
    FlowBoxChild::builder()
        .child(&frame)
        .halign(Align::Fill)
        .valign(Align::Start)
        .build()
}
