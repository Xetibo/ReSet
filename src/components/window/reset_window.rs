use adw::glib::clone;
use adw::subclass::prelude::ObjectSubclassIsExt;
use adw::BreakpointCondition;
use glib::Object;
use gtk::prelude::*;
use gtk::{gio, glib, Application, ListBoxRow, Orientation};

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
        let obj: Self = Object::builder().property("application", app).build();
        let imp = obj.imp();
        (HANDLE_HOME)(
            imp.listeners.clone(),
            imp.reset_main.get(),
            imp.position.clone(),
        );
        obj
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

        // sidebarEntries.push((
        //     SidebarEntry::new(
        //         "Peripherals",
        //         "preferences-system-devices-symbolic",
        //         Categories::Peripherals,
        //         false,
        //         HANDLE_PERIPHERALS_CLICK,
        //     ),
        //     peripheralsList,
        // ));

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
                .build();
            let separator_row = ListBoxRow::new();
            separator_row.set_child(Some(&separator));
            separator_row.set_selectable(false);
            separator_row.set_activatable(false);
            self_imp.reset_sidebar_list.append(&separator_row);
        }
    }

    pub fn setup_popover_buttons(&self) {
        let self_imp = self.imp();
        self_imp
            .reset_about_button
            .connect_clicked(clone!(@ weak self as window => move |_| {
                    let dialog = adw::AboutWindow::builder()
                        .application_name("ReSet")
                        .application_icon("ReSet")
                        .developer_name("Xetibo")
                        .license("GPL-3.0")
                        .license_type(gtk::License::Gpl30)
                        .website("https://github.com/Xetibo/ReSet")
                        .issue_url("https://github.com/Xetibo/ReSet/issues")
                        .version("0.0.1")
                        .transient_for(&window)
                        .modal(true)
                        .copyright("© 2022-2023 Xetibo")
                        .developers(vec!["DashieTM".to_string(), "Takotori".to_string()])
                        .designers(vec!["DashieTM".to_string(), "Takotori".to_string()])
                        .build();
                window.imp().reset_popover_menu.popdown();
                dialog.present();
            }));
        self_imp
            .reset_preference_button
            .connect_clicked(clone!(@weak self as window => move |_| {
                let preferences = adw::PreferencesWindow::builder().build();
                window.imp().reset_popover_menu.popdown();
                preferences.present();
            }));
        self_imp
            .reset_shortcuts_button
            .connect_clicked(clone!(@weak self as window => move |_| {
                let shortcuts = gtk::ShortcutsWindow::builder().build();
                window.imp().reset_popover_menu.popdown();
                shortcuts.present();
            }));
    }
}
