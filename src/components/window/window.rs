use adw::BreakpointCondition;
use adw::glib::clone;
use adw::subclass::prelude::ObjectSubclassIsExt;
use glib::{closure_local, Object};
use gtk::{Application, gio, glib, ListBoxRow, Orientation};
use gtk::prelude::*;

use crate::components::breadcrumb::breadcrumb;
use crate::components::breadcrumb::breadcrumb::Breadcrumb;
use crate::components::wifi::wifiBox::WifiBox;
use crate::components::window::handleSidebarClick::*;
use crate::components::window::sidebarEntry::SidebarEntry;
use crate::components::window::sidebarEntryImpl::Categories;
use crate::components::window::windowImpl;

glib::wrapper! {
    pub struct Window(ObjectSubclass<windowImpl::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}

#[allow(non_snake_case)]
impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    pub fn setupCallback(&self) {
        let selfImp = self.imp();


        selfImp.resetPath.connect_closure("max-number-reached", false, closure_local!(move |x : Breadcrumb, y : i32| {
            print!("askdfj");
        }));

        let sadf:i32 = 2312;
        Breadcrumb::new().emit_by_name::<()>("max-number-reached", &[&sadf]);

        let breadcrumb= Breadcrumb::new();
        breadcrumb.emit_by_name::<()>("max-number-reached", &[&sadf]);



        selfImp
            .resetSearchEntry
            .connect_search_changed(clone!(@ weak self as window => move |_| {
                window.filterList();
            }));

        selfImp
            .resetSideBarToggle
            .connect_clicked(clone!(@ weak self as window => move |_| {
                window.toggleSidebar();
            }));

        selfImp.resetSidebarList.connect_row_activated(
            clone!(@ weak selfImp as flowbox => move |_, y| {
                let result = y.downcast_ref::<SidebarEntry>().unwrap();
                let clickEvent = result.imp().onClickEvent.borrow().onClickEvent;
                (clickEvent)(flowbox.listeners.clone(), flowbox.resetMain.get(), flowbox.resetPath.get());
            }),
        );

        selfImp
            .resetClose
            .connect_clicked(clone!(@ weak self as window => move |_| {
                window.close();
            }));
    }

    pub fn handleDynamicSidebar(&self) {
        let selfImp = self.imp();
        selfImp
            .resetSidebarBreakpoint
            .set_condition(BreakpointCondition::parse("max-width: 700sp").as_ref().ok());
        selfImp.resetSidebarBreakpoint.add_setter(
            &Object::from(selfImp.resetOverlaySplitView.get()),
            "collapsed",
            &true.to_value(),
        );
        selfImp.resetSidebarBreakpoint.add_setter(
            &Object::from(selfImp.resetSideBarToggle.get()),
            "visible",
            &true.to_value(),
        );
    }

    pub fn filterList(&self) {
        let text = self.imp().resetSearchEntry.text().to_string();
        for (mainEntry, subEntries) in self.imp().sidebarEntries.borrow().iter() {
            if text == "" {
                mainEntry.set_visible(true);
                for subEntry in subEntries {
                    subEntry.set_visible(true);
                }
                continue;
            }
            if mainEntry
                .imp()
                .name
                .borrow()
                .to_lowercase()
                .contains(&text.to_lowercase())
            {
                mainEntry.set_visible(true);
            } else {
                mainEntry.set_visible(false);
            }
            for subEntry in subEntries {
                if subEntry
                    .imp()
                    .name
                    .borrow()
                    .to_lowercase()
                    .contains(&text.to_lowercase())
                {
                    subEntry.set_visible(true);
                    mainEntry.set_visible(true);
                } else {
                    subEntry.set_visible(false);
                }
            }
        }
    }

    pub fn toggleSidebar(&self) {
        if self.imp().resetOverlaySplitView.shows_sidebar() {
            self.imp().resetOverlaySplitView.set_show_sidebar(false);
        } else {
            self.imp().resetOverlaySplitView.set_show_sidebar(true);
        }
    }

    pub fn setupSidebarEntries(&self) {
        let selfImp = self.imp();
        let mut sidebarEntries = selfImp.sidebarEntries.borrow_mut();

        let connectivityList = vec![
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
            SidebarEntry::new(
                "VPN",
                "network-vpn-symbolic",
                Categories::Connectivity,
                true,
                HANDLE_VPN_CLICK,
            ),
        ];

        sidebarEntries.push((
            SidebarEntry::new(
                "Connectivity",
                "network-wired-symbolic",
                Categories::Connectivity,
                false,
                HANDLE_CONNECTIVITY_CLICK,
            ),
            connectivityList,
        ));

        let audioList = vec![
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

        sidebarEntries.push((
            SidebarEntry::new(
                "Audio",
                "audio-headset-symbolic",
                Categories::Audio,
                false,
                HANDLE_AUDIO_CLICK,
            ),
            audioList,
        ));

        let peripheralsList = vec![
            SidebarEntry::new(
                "Displays",
                "video-display-symbolic",
                Categories::Peripherals,
                true,
                HANDLE_MONITOR_CLICK,
            ),
            SidebarEntry::new(
                "Mouse",
                "input-mouse-symbolic",
                Categories::Peripherals,
                true,
                HANDLE_MOUSE_CLICK,
            ),
            SidebarEntry::new(
                "Keyboard",
                "input-keyboard-symbolic",
                Categories::Peripherals,
                true,
                HANDLE_KEYBOARD_CLICK,
            ),
        ];

        sidebarEntries.push((
            SidebarEntry::new(
                "Peripherals",
                "preferences-system-devices-symbolic",
                Categories::Peripherals,
                false,
                HANDLE_PERIPHERALS_CLICK,
            ),
            peripheralsList,
        ));

        for (mainEntry, subEntries) in sidebarEntries.iter() {
            selfImp.resetSidebarList.append(mainEntry);
            for subEntry in subEntries {
                selfImp.resetSidebarList.append(subEntry);
            }
            let separator = ListBoxRow::new();
            separator.set_child(Some(&gtk::Separator::new(Orientation::Horizontal)));
            separator.set_selectable(false);
            separator.set_activatable(false);
            selfImp.resetSidebarList.append(&separator);
        }
    }

    pub fn setupPopoverButtons(&self) {
        let selfImp = self.imp();
        selfImp
            .resetAboutButton
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
                window.imp().resetPopoverMenu.popdown();
                dialog.present();
            }));
        selfImp
            .resetPreferenceButton
            .connect_clicked(clone!(@weak self as window => move |_| {
                let preferences = adw::PreferencesWindow::builder().build();
                window.imp().resetPopoverMenu.popdown();
                preferences.present();
            }));
        selfImp
            .resetShortcutsButton
            .connect_clicked(clone!(@weak self as window => move |_| {
                let shortcuts = gtk::ShortcutsWindow::builder().build();
                window.imp().resetPopoverMenu.popdown();
                shortcuts.present();
            }));
    }
}
