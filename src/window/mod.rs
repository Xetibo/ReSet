#![allow(non_snake_case)]

use adw::BreakpointCondition;
use adw::glib::clone;
use adw::subclass::prelude::ObjectSubclassIsExt;
use glib::Object;
use gtk::{Application, FlowBox, gio, glib};
use gtk::prelude::*;

use crate::window::handleSidebarClick::{HANDLE_AUDIO_CLICK, HANDLE_BLUETOOTH_CLICK, HANDLE_CONNECTIVITY_CLICK, HANDLE_MICROPHONE_CLICK, HANDLE_VOLUME_CLICK, HANDLE_VPN_CLICK, HANDLE_WIFI_CLICK};
use crate::window::sidebarEntry::{Categories, SidebarAction};

mod window;
mod sidebarEntry;
mod handleSidebarClick;

glib::wrapper! {
    pub struct Window(ObjectSubclass<window::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

glib::wrapper! {
    pub struct SidebarEntry(ObjectSubclass<sidebarEntry::SidebarEntry>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

#[allow(non_snake_case)]
impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setupCallback(&self) {
        let selfImp = self.imp();

        selfImp.resetSearchEntry.connect_search_changed(clone!(@ weak self as window => move |_| {
                window.filterList();
            }));

        selfImp.resetSideBarToggle.connect_clicked(clone!(@ weak self as window => move |_| {
                window.toggleSidebar();
            }));

        selfImp.resetSidebarList.connect_row_activated(clone!(@ weak selfImp as flowbox => move |x, y| {
            let result = y.downcast_ref::<SidebarEntry>().unwrap();
            let clickEvent = result.imp().onClickEvent.borrow().onClickEvent;
            (clickEvent)(flowbox.resetMain.get());
        }));
    }

    fn handleDynamicSidebar(&self) {
        let selfImp = self.imp();
        selfImp.resetSidebarBreakpoint.set_condition(BreakpointCondition::parse("max-width: 500sp").as_ref().ok());
        selfImp.resetSidebarBreakpoint.add_setter(&Object::from(selfImp.resetOverlaySplitView.get()),
                                                  "collapsed",
                                                  &true.to_value());
        selfImp.resetSidebarBreakpoint.add_setter(&Object::from(selfImp.resetSideBarToggle.get()),
                                                  "visible",
                                                  &true.to_value());
    }

    fn filterList(&self) {
        let text = self.imp().resetSearchEntry.text().to_string();
        for (mainEntry, subEntries) in self.imp().sidebarEntries.borrow().iter() {
            if text == "" {
                mainEntry.set_visible(true);
                for subEntry in subEntries {
                    subEntry.set_visible(true);
                }
                continue;
            }
            if mainEntry.imp().name.borrow().to_lowercase().contains(&text.to_lowercase()) {
                mainEntry.set_visible(true);
            } else {
                mainEntry.set_visible(false);
            }
            for subEntry in subEntries {
                if subEntry.imp().name.borrow().to_lowercase().contains(&text.to_lowercase()) {
                    subEntry.set_visible(true);
                    mainEntry.set_visible(true);
                } else {
                    subEntry.set_visible(false);
                }
            }
        }
    }

    fn toggleSidebar(&self) {
        if self.imp().resetOverlaySplitView.shows_sidebar() {
            self.imp().resetOverlaySplitView.set_show_sidebar(false);
        } else {
            self.imp().resetOverlaySplitView.set_show_sidebar(true);
        }
    }

    fn setupSidebarEntries(&self) {
        let selfImp = self.imp();
        let mut sidebarEntries = selfImp.sidebarEntries.borrow_mut();

        let connectivityList = vec![SidebarEntry::new("WiFi",
                                                          "network-wireless-symbolic",
                                                          Categories::Connectivity,
                                                          true,
                                                          HANDLE_WIFI_CLICK),
                                        SidebarEntry::new("Bluetooth",
                                                          "bluetooth-symbolic",
                                                          Categories::Connectivity,
                                                          true,
                                                          HANDLE_BLUETOOTH_CLICK),
                                        SidebarEntry::new("VPN",
                                                          "network-vpn-symbolic",
                                                          Categories::Connectivity,
                                                          true,
                                                          HANDLE_VPN_CLICK)];

        sidebarEntries.push((SidebarEntry::new("Connectivity",
                                               "network-wired-symbolic",
                                               Categories::Connectivity,
                                               false,
                                               HANDLE_CONNECTIVITY_CLICK), connectivityList));

        let audioList = vec![SidebarEntry::new("Volume",
                                                   "audio-volume-high-symbolic",
                                                   Categories::Audio,
                                                   true,
                                                   HANDLE_VOLUME_CLICK),
                                 SidebarEntry::new("Microphone",
                                                   "audio-input-microphone-symbolic",
                                                   Categories::Audio,
                                                   true,
                                                   HANDLE_MICROPHONE_CLICK)];

        sidebarEntries.push((SidebarEntry::new("Audio",
                                               "audio-headset-symbolic",
                                               Categories::Audio,
                                               false,
                                               HANDLE_AUDIO_CLICK), audioList));


        for (mainEntry, subEntries) in sidebarEntries.iter() {
            selfImp.resetSidebarList.append(mainEntry);
            for subEntry in subEntries {
                selfImp.resetSidebarList.append(subEntry);
            }
        }
    }
}

impl SidebarEntry {
    pub fn new(entryName: &str, iconName: &str, category: Categories, isSubcategory: bool, clickEvent: fn(FlowBox)) -> Self {
        let entry: SidebarEntry = Object::builder().build();
        let entryImp = entry.imp();
        entryImp.resetSidebarLabel.get().set_text(entryName);
        entryImp.resetSidebarImage.set_from_icon_name(Some(iconName));
        entryImp.category.set(category);
        entryImp.isSubcategory.set(isSubcategory);
        {
            let mut name = entryImp.name.borrow_mut();
            *name = String::from(entryName);
            let mut action = entryImp.onClickEvent.borrow_mut();
            *action = SidebarAction { onClickEvent: clickEvent };
        }
        Self::setMargin(&entry);
        entry
    }

    fn setMargin(entry: &SidebarEntry) {
        if entry.imp().isSubcategory.get() {
            let option = entry.child().unwrap();
            option.set_margin_start(30);
        }
    }
}