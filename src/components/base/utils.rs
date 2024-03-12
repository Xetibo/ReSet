use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use dbus::{blocking::Connection, Error};
use gtk::gio;

use crate::components::{
    audio::input::source_box::{start_source_box_listener, SourceBox},
    audio::output::sink_box::{start_sink_box_listener, SinkBox},
    utils::{BASE, DBUS_PATH, WIRELESS},
};

#[derive(Default, PartialEq, Eq)]
pub enum Position {
    Connectivity,
    Wifi,
    Bluetooth,
    Audio,
    AudioOutput,
    AudioInput,
    #[default]
    Home,
}

#[derive(Default)]
pub struct Listeners {
    pub wifi_disabled: AtomicBool,
    pub wifi_listener: AtomicBool,
    pub bluetooth_listener: AtomicBool,
    pub bluetooth_scan_requested: AtomicBool,
    pub pulse_listener: AtomicBool,
}

impl Listeners {
    pub fn stop_network_listener(&self) {
        if !self.wifi_listener.load(Ordering::SeqCst) {
            return;
        }
        self.wifi_listener.store(false, Ordering::SeqCst);
        thread::spawn(|| {
            let conn = Connection::new_session().unwrap();
            let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
            let _: Result<(bool,), Error> = proxy.method_call(WIRELESS, "StopNetworkListener", ());
        });
    }

    pub fn stop_audio_listener(&self) {
        self.pulse_listener.store(false, Ordering::SeqCst);
    }

    pub fn stop_bluetooth_listener(&self) {
        self.bluetooth_listener.store(false, Ordering::SeqCst);
    }
}

pub fn start_audio_listener(
    listeners: Arc<Listeners>,
    sink_box: Option<Arc<SinkBox>>,
    source_box: Option<Arc<SourceBox>>,
) {
    gio::spawn_blocking(move || {
        let mut conn = Connection::new_session().unwrap();
        if listeners.pulse_listener.load(Ordering::SeqCst) {
            return;
        }

        if let Some(sink_box) = sink_box {
            conn = start_sink_box_listener(conn, sink_box);
        }
        if let Some(source_box) = source_box {
            conn = start_source_box_listener(conn, source_box);
        }

        listeners.pulse_listener.store(true, Ordering::SeqCst);

        loop {
            let _ = conn.process(Duration::from_millis(1000));
            if !listeners.pulse_listener.load(Ordering::SeqCst) {
                break;
            }
        }
    });
}
