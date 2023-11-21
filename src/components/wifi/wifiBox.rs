use std::collections::HashMap;
use std::net::Shutdown::Read;
use std::sync::atomic::Ordering;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::components::base::listEntry::ListEntry;
use crate::components::base::utils::Listeners;
use adw::glib;
use adw::glib::Object;
use adw::prelude::{ListBoxRowExt, PreferencesGroupExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use dbus::arg::{AppendAll, ReadAll, RefArg};
use dbus::blocking::Connection;
use dbus::Error;
use dbus::Path;
use glib::ObjectExt;
use gtk::gio;
use gtk::glib::Variant;
use gtk::prelude::ActionableExt;
use ReSet_Lib::network::network::AccessPoint;
use ReSet_Lib::signals::AccessPointRemoved;
use ReSet_Lib::signals::{AccessPointAdded, GetVal};
use ReSet_Lib::utils::Events;

use crate::components::wifi::wifiBoxImpl;
use crate::components::wifi::wifiEntry::WifiEntry;

use super::savedWifiEntry::SavedWifiEntry;

use ReSet_Lib::network::connection::Connection as ResetConnection;

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
        selfImp.resetSavedNetworks.set_activatable(true);
        selfImp
            .resetSavedNetworks
            .set_action_name(Some("navigation.push"));
        selfImp
            .resetSavedNetworks
            .set_action_target_value(Some(&Variant::from("saved")));

        selfImp
            .resetAvailableNetworks
            .set_action_name(Some("navigation.pop"));
    }
}

pub fn scanForWifi(listeners: Arc<Listeners>, wifiBox: Arc<WifiBox>) {
    let wifibox_ref = wifiBox.clone();
    let wifibox_ref_listener = wifiBox.clone();
    let wifiEntries = wifiBox.imp().wifiEntries.clone();

    gio::spawn_blocking(move || {
        let accessPoints = get_access_points();
        let wifiEntriesListener = wifiEntries.clone();
        let wifiEntries = wifiEntries.clone();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let mut wifiEntries = wifiEntries.lock().unwrap();
                let selfImp = wifibox_ref.imp();
                for accessPoint in accessPoints {
                    let ssid = accessPoint.ssid.clone();
                    let entry = WifiEntry::new(accessPoint, selfImp);
                    wifiEntries.insert(ssid, entry.clone());
                    selfImp.resetWifiList.add(&*entry);
                }
            });
        });
        if listeners.network_listener.load(Ordering::SeqCst) {
            return;
        }
        listeners.network_listener.store(true, Ordering::SeqCst);
        dbus_start_network_events();
        let (sender, receiver): (
            Sender<Events<(AccessPoint,), (AccessPoint,)>>,
            Receiver<Events<(AccessPoint,), (AccessPoint,)>>,
        ) = channel();
        let sender_ref = Arc::new(sender);
        let res = start_event_listener::<
            (AccessPoint,),
            (AccessPoint,),
            AccessPointAdded,
            AccessPointRemoved,
        >(listeners.clone(), sender_ref);
        if res.is_err() {
            println!("Could not connect listener");
        }
        loop {
            let wifiEntriesListener = wifiEntriesListener.clone();
            if listeners
                .network_listener
                .load(std::sync::atomic::Ordering::SeqCst)
                == false
            {
                break;
            }
            let res = receiver.recv();
            if res.is_ok() {
                let access_point = res.unwrap();
                match access_point {
                    Events::AddedEvent(access_point) => {
                        let wifiEntriesListener = wifiEntriesListener.clone();
                        let wifiBoxImpl = wifibox_ref_listener.clone();
                        glib::spawn_future(async move {
                            glib::idle_add_once(move || {
                                let mut wifiEntries = wifiEntriesListener.lock().unwrap();
                                let ssid = access_point.0.ssid.clone();
                                if wifiEntries.get(&ssid).is_some() {
                                    return;
                                }
                                let entry = WifiEntry::new(access_point.0, wifiBoxImpl.imp());
                                wifiEntries.insert(ssid, entry.clone());
                                wifiBoxImpl.imp().resetWifiList.add(&*entry);
                            });
                        });
                    }
                    Events::RemovedEvent(access_point) => {
                        let wifiBoxImpl = wifibox_ref_listener.clone();
                        glib::spawn_future(async move {
                            glib::idle_add_once(move || {
                                let mut wifiEntries = wifiEntriesListener.lock().unwrap();
                                let entry = wifiEntries.remove(&access_point.0.ssid);
                                if entry.is_none() {
                                    return;
                                }
                                wifiBoxImpl.imp().resetWifiList.remove(&*entry.unwrap());
                            });
                        });
                    }
                };
            } else {
                println!("no message there :)");
            }
        }
    });
}

pub fn show_stored_connections(wifiBox: Arc<WifiBox>) {
    let wifibox_ref = wifiBox.clone();
    gio::spawn_blocking(move || {
        let connections = get_stored_connections();
        glib::spawn_future(async move {
            glib::idle_add_once(move || {
                let selfImp = wifibox_ref.imp();
                for connection in connections {
                    // TODO include button for settings
                    let name =
                        &String::from_utf8(connection.1).unwrap_or_else(|_| String::from(""));
                    let entry = SavedWifiEntry::new(name, connection.0);
                    selfImp.resetStoredWifiList.add(&entry);
                }
            });
        });
    });
}

pub fn dbus_start_network_events() {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let _: Result<(), Error> = proxy.method_call("org.xetibo.ReSet", "StartNetworkListener", ());
}

pub fn get_access_points() -> Vec<AccessPoint> {
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

pub fn get_stored_connections() -> Vec<(Path<'static>, Vec<u8>)> {
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
    connections
}

pub fn getConnectionSettings(path: Path<'static>) -> ResetConnection {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let res: Result<
        (HashMap<String, HashMap<String, dbus::arg::Variant<Box<dyn RefArg>>>>,),
        Error,
    > = proxy.method_call("org.xetibo.ReSet", "GetConnectionSettings", (path,));
    if res.is_err() {
        ResetConnection::default();
    }
    let (res,) = res.unwrap();
    let res = ResetConnection::convert_from_propmap(res);
    if res.is_err() {
        ResetConnection::default();
    }
    res.unwrap()
}

// temporary, testing this with lib is pain
//

pub fn start_event_listener<
    AddedType: ReadAll + AppendAll + Send + Sync + 'static,
    RemovedType: ReadAll + AppendAll + Send + Sync + 'static,
    AddedEvent: ReadAll + AppendAll + dbus::message::SignalArgs + GetVal<AddedType>,
    RemovedEvent: ReadAll + AppendAll + dbus::message::SignalArgs + GetVal<RemovedType>,
>(
    listeners: Arc<Listeners>,
    sender: Arc<Sender<Events<AddedType, RemovedType>>>,
) -> Result<(), dbus::Error> {
    thread::spawn(move || {
        let added_sender = sender.clone();
        let removed_sender = sender.clone();
        let conn = Connection::new_session().unwrap();
        let mr = AddedEvent::match_rule(
            Some(&"org.xetibo.ReSet".into()),
            Some(&Path::from("/org/xetibo/ReSet")),
        )
        .static_clone();
        let mrb = RemovedEvent::match_rule(
            Some(&"org.xetibo.ReSet".into()),
            Some(&Path::from("/org/xetibo/ReSet")),
        )
        .static_clone();
        let res = conn.add_match(mr, move |ir: AddedEvent, _, _| {
            println!("received added event");
            let res = added_sender.send(Events::AddedEvent(ir.get_value()));
            if res.is_err() {
                println!("fail on sending added");
                return false;
            }
            true
        });
        if res.is_err() {
            println!("fail on add");
            return Err(dbus::Error::new_custom(
                "SignalMatchFailed",
                "Failed to match signal on ReSet.",
            ));
        }
        let res = conn.add_match(mrb, move |ir: RemovedEvent, _, _| {
            println!("received removed event");
            let res = removed_sender.send(Events::RemovedEvent(ir.get_value()));
            if res.is_err() {
                println!("fail on sending removed");
                return false;
            }
            true
        });
        if res.is_err() {
            println!("fail on remove");
            return Err(dbus::Error::new_custom(
                "SignalMatchFailed",
                "Failed to match signal on ReSet.",
            ));
        }
        listeners.network_listener.store(true, Ordering::SeqCst);
        println!("starting thread listener");
        loop {
            let _ = conn.process(Duration::from_millis(1000))?;
            if !listeners.network_listener.load(Ordering::SeqCst) {
                println!("stopping thread listener");
                break;
            }
            thread::sleep(Duration::from_millis(1000));
        }
        Ok(())
    });
    Ok(())
}
