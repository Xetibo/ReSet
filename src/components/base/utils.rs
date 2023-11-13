use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use dbus::{blocking::Connection, Error};

#[derive(Default)]
pub struct Listeners {
    pub network_listener: AtomicBool,
    pub bluetooth_listener: AtomicBool,
    pub pulse_listener: AtomicBool,
}

impl Listeners {
    pub fn stop_network_listener(&self) {
        if !self.network_listener.load(Ordering::SeqCst) {
            return;
        }
        self.network_listener.store(false, Ordering::SeqCst);
        thread::spawn(|| {
            let conn = Connection::new_session().unwrap();
            let proxy = conn.with_proxy(
                "org.xetibo.ReSet",
                "/org/xetibo/ReSet",
                Duration::from_millis(1000),
            );
            let _: Result<(bool,), Error> =
                proxy.method_call("org.xetibo.ReSet", "StopNetworkListener", ());
        });
    }
}
