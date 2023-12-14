use adw::glib::clone;
use adw::subclass::prelude::ObjectSubclassIsExt;
use adw::BreakpointCondition;
use glib::Object;
use gtk::gio::ActionEntry;
use gtk::{gio, glib, Application, ListBoxRow, Orientation, AccessibleRole};
use gtk::{prelude::*, DirectionType};

use crate::components::window::handle_sidebar_click::*;
use crate::components::window::reset_window_impl;
use crate::components::window::sidebar_entry::SidebarEntry;
use crate::components::window::sidebar_entry_impl::Categories;

glib::wrapper! {
    pub struct ReSetWindow(ObjectSubclass<reset_window_impl::ReSetWindow>)
        @extends adw::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

unsafe impl Send for ReSetWindow {}

unsafe impl Sync for ReSetWindow {}

impl ReSetWindow {
    pub fn new(app: &Application) -> Self {
        app.set_accels_for_action("win.search", &["<Ctrl>F"]);
        app.set_accels_for_action("win.close", &["<Ctrl>Q"]);
        app.set_accels_for_action("win.about", &["<Ctrl>A"]);
        // implemented when a proper movement method is found
        // app.set_accels_for_action("win.up", &["<Ctrl>K"]);
        // app.set_accels_for_action("win.right", &["<Ctrl>L"]);
        // app.set_accels_for_action("win.down", &["<Ctrl>J"]);
        // app.set_accels_for_action("win.left", &["<Ctrl>H"]);
        Object::builder().property("application", app).build()
    }

    pub fn setup_shortcuts(&self) {
        let search_action = ActionEntry::builder("search")
            .activate(move |window: &Self, _, _| {
                window.imp().reset_search_entry.grab_focus();
            })
            .build();

        let close_action = ActionEntry::builder("close")
            .activate(move |window: &Self, _, _| {
                window.close();
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
                    .version("0.0.1")
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
        ]);
    }

    pub fn setup_callback(&self) {
        let self_imp = self.imp();

        self_imp.reset_search_entry.connect_search_changed(
            clone!(@ weak self as window => move |_| {
                window.filter_list();
            }),
        );

        self_imp
            .reset_sidebar_toggle
            .connect_clicked(clone!(@ weak self as window => move |_| {
                window.toggle_sidebar();
            }));

        self_imp.reset_sidebar_list.connect_row_activated(
            clone!(@ weak self_imp as flowbox => move |_, y| {
                let result = y.downcast_ref::<SidebarEntry>().unwrap();
                let click_event = result.imp().on_click_event.borrow().on_click_event;
                (click_event)(flowbox.listeners.clone(), flowbox.reset_main.get(), flowbox.position.clone());
            }),
        );

        self_imp
            .reset_close
            .connect_clicked(clone!(@ weak self as window => move |_| {
                window.close();
            }));
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
        let mut sidebar_entries = self_imp.sidebar_entries.borrow_mut();

        let connectivity_list = vec![
            SidebarEntry::new(
                "WiFi",
                "network-wireless-symbolic",
                Categories::Connectivity,
                true,
                HANDLE_WIFI_CLICK,
            ),
            SidebarEntry::new(
                "Bluetooth",
                "bluetooth-symbolic",
                Categories::Connectivity,
                true,
                HANDLE_BLUETOOTH_CLICK,
            ),
            // uncommented when VPN is implemented
            // SidebarEntry::new(
            //     "VPN",
            //     "network-vpn-symbolic",
            //     Categories::Connectivity,
            //     true,
            //     HANDLE_VPN_CLICK,
            // ),
        ];

        sidebar_entries.push((
            SidebarEntry::new(
                "Connectivity",
                "network-wired-symbolic",
                Categories::Connectivity,
                false,
                HANDLE_CONNECTIVITY_CLICK,
            ),
            connectivity_list,
        ));

        let audio_list = vec![
            SidebarEntry::new(
                "Output",
                "audio-volume-high-symbolic",
                Categories::Audio,
                true,
                HANDLE_VOLUME_CLICK,
            ),
            SidebarEntry::new(
                "Input",
                "audio-input-microphone-symbolic",
                Categories::Audio,
                true,
                HANDLE_MICROPHONE_CLICK,
            ),
        ];

        sidebar_entries.push((
            SidebarEntry::new(
                "Audio",
                "audio-headset-symbolic",
                Categories::Audio,
                false,
                HANDLE_AUDIO_CLICK,
            ),
            audio_list,
        ));

        // uncommented when implemented
        // let peripheralsList = vec![
        //     SidebarEntry::new(
        //         "Displays",
        //         "video-display-symbolic",
        //         Categories::Peripherals,
        //         true,
        //         HANDLE_MONITOR_CLICK,
        //     ),
        //     SidebarEntry::new(
        //         "Mouse",
        //         "input-mouse-symbolic",
        //         Categories::Peripherals,
        //         true,
        //         HANDLE_MOUSE_CLICK,
        //     ),
        //     SidebarEntry::new(
        //         "Keyboard",
        //         "input-keyboard-symbolic",
        //         Categories::Peripherals,
        //         true,
        //         HANDLE_KEYBOARD_CLICK,
        //     ),
        // ];

        // let home = SidebarEntry::new(
        //     "Home",
        //     "preferences-system-devices-symbolic",
        //     Categories::Peripherals,
        //     false,
        //     HANDLE_VOLUME_CLICK,
        // );
        //
        // sidebar_entries.push((home, Vec::new()));

        (HANDLE_VOLUME_CLICK)(
            self_imp.listeners.clone(),
            self_imp.reset_main.clone(),
            self_imp.position.clone(),
        );

        self_imp
            .reset_sidebar_list
            .connect_row_activated(clone!(@ weak self_imp => move |_, _| {
                self_imp.reset_search_entry.set_text("");
            }));

        for (main_entry, sub_entries) in sidebar_entries.iter() {
            self_imp.reset_sidebar_list.append(main_entry);
            for sub_entry in sub_entries {
                self_imp.reset_sidebar_list.append(sub_entry);
            }
            let separator = gtk::Separator::builder()
                .margin_bottom(3)
                .margin_top(3)
                .orientation(Orientation::Horizontal)
                .accessible_role(AccessibleRole::Separator)
                .can_focus(false)
                .build();
            let separator_row = ListBoxRow::builder()
                .child(&separator)
                .selectable(false)
                .activatable(false)
                .can_target(false)
                .focusable(false)
                .accessible_role(AccessibleRole::Separator)
                .build();
            // TODO how to simply skip this ?
            self_imp.reset_sidebar_list.append(&separator_row);
        }
    }
}
