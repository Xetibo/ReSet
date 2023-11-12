use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{atomic::AtomicBool, Arc, Weak};
use std::thread;
use std::time::Duration;

use crate::components::base::listEntry::ListEntry;
use adw::glib;
use adw::glib::Object;
use adw::prelude::{BoxExt, ButtonExt, ListBoxRowExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::blocking::Connection;
use dbus::Error;
use dbus::Path;
use gtk::glib::{clone, Variant};
use gtk::prelude::ActionableExt;
use gtk::{Button, Label, Orientation};
use ReSet_Lib::network::network::{AccessPoint, WifiStrength};
use ReSet_Lib::signals::{
    AccessPointAdded, AccessPointRemoved, BluetoothDeviceAdded, BluetoothDeviceRemoved,
};
use ReSet_Lib::utils::Events;

use crate::components::wifi::wifiBoxImpl;
use crate::components::wifi::wifiEntry::WifiEntry;

use super::savedWifiEntry::SavedWifiEntry;

glib::wrapper! {
    pub struct WifiBox(ObjectSubclass<wifiBoxImpl::WifiBox>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

unsafe impl Send for WifiBox {}
unsafe impl Sync for WifiBox {}

impl WifiBox {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn setupCallbacks(&self) {
        let selfImp = self.imp();

        selfImp
            .resetSavedNetworks
            .set_action_name(Some("navigation.push"));
        selfImp
            .resetSavedNetworks
            .set_action_target_value(Some(&Variant::from("saved")));
    }

    // pub fn donotdisturb() {
    //     thread::spawn(|| {
    //         let conn = Connection::new_session().unwrap();
    //         let proxy = conn.with_proxy(
    //             "org.freedesktop.Notifications",
    //             "/org/freedesktop/Notifications",
    //             Duration::from_millis(1000),
    //         );
    //         let _: Result<(), Error> =
    //             proxy.method_call("org.freedesktop.Notifications", "DoNotDisturb", ());
    //     });
    // }
}

pub fn scanForWifi(wifiBox: Arc<WifiBox>) {
    let wifibox_ref = wifiBox.clone();
    let wifiEntries = wifiBox.imp().wifiEntries.clone();

    glib::spawn_future_local(async move {
        let accessPoints = get_access_points().await;
        let wifiEntries = wifiEntries.clone();
        {
            let mut wifiEntries = wifiEntries.lock().unwrap();
            for accessPoint in accessPoints {
                wifiEntries.push(ListEntry::new(&*WifiEntry::new(accessPoint)));
            }
        }
        glib::MainContext::default().spawn_local(async move {
            glib::idle_add_once(move || {
                let wifiEntries = wifiEntries.lock().unwrap();
                let selfImp = wifibox_ref.imp();
                for wifiEntry in wifiEntries.iter() {
                    selfImp.resetWifiList.append(wifiEntry);
                }
            });
        });
        let (sender, receiver): (
            Sender<Events<(AccessPoint,), (Path<'static>,)>>,
            Receiver<Events<(AccessPoint,), (Path<'static>,)>>,
        ) = channel();
        let sender_ref = Arc::new(sender);
        let listener_active = Arc::new(AtomicBool::new(false));
        ReSet_Lib::utils::start_event_listener::<
            (AccessPoint,),
            (Path<'static>,),
            AccessPointAdded,
            AccessPointRemoved,
        >(listener_active, sender_ref);
        // handle receiver...
        let res = receiver.try_recv();
        if res.is_ok() {
            let access_point = res.unwrap();
            match access_point {
                Events::AddedEvent(access_point) => {
                    dbg!(access_point);
                }
                _ => (),
            };
        } else {
            println!("no message there :)");
        }
    });
}

pub fn show_stored_connections(wifiBox: Arc<WifiBox>) {
    let wifibox_ref = wifiBox.clone();
    let wifiEntries = wifiBox.imp().savedWifiEntries.clone();

    glib::spawn_future_local(async move {
        let connections = get_stored_connections().await;
        let wifiEntries = wifiEntries.clone();
        {
            let mut wifiEntries = wifiEntries.lock().unwrap();
            for connection in connections {
                // TODO include button for settings
                let name = &String::from_utf8(connection.1).unwrap_or_else(|_| String::from(""));
                let entry = ListEntry::new(&SavedWifiEntry::new(name, connection.0));
                entry.set_activatable(false);
                wifiEntries.push(entry);
            }
        }
        glib::MainContext::default().spawn_local(async move {
            glib::idle_add_once(move || {
                let wifiEntries = wifiEntries.lock().unwrap();
                let selfImp = wifibox_ref.imp();
                for wifiEntry in wifiEntries.iter() {
                    selfImp.resetStoredWifiList.append(wifiEntry);
                }
            });
        });
    });
}

pub async fn get_access_points() -> Vec<AccessPoint> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(Vec<AccessPoint>,), Error> =
        proxy.method_call("org.xetibo.ReSet", "ListAccessPoints", ());
    if res.is_err() {
        return Vec::new();
    }
    let (accessPoints,) = res.unwrap();
    accessPoints
}

pub async fn get_stored_connections() -> Vec<(Path<'static>, Vec<u8>)> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<(Vec<(Path<'static>, Vec<u8>)>,), Error> =
        proxy.method_call("org.xetibo.ReSet", "ListStoredConnections", ());
    if res.is_err() {
        println!("we got error...");
        return Vec::new();
    }
    let (connections,) = res.unwrap();
    dbg!(connections.clone());
    connections
}
