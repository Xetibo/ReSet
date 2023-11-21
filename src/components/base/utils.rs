use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use dbus::{
    arg::{self, Append},
    blocking::Connection,
    Error,
};
use gtk::gio;
use ReSet_Lib::{
    audio::audio::{InputStream, OutputStream, Sink, Source},
    signals::GetVal,
};

use crate::components::{
    input::sourceBox::{start_input_box_listener, SourceBox},
    output::sinkBox::{start_output_box_listener, SinkBox},
};

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

    pub fn stop_audio_listener(&self) {
        self.pulse_listener.store(false, Ordering::SeqCst);
    }

    pub fn stop_bluetooth_listener(&self) {
        self.bluetooth_listener.store(false, Ordering::SeqCst);
    }
}

#[derive(Debug)]
pub struct SinkAdded {
    pub sink: Sink,
}

impl arg::AppendAll for SinkAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        self.sink.append_by_ref(i);
    }
}

impl arg::ReadAll for SinkAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SinkAdded { sink: i.read()? })
    }
}

impl dbus::message::SignalArgs for SinkAdded {
    const NAME: &'static str = "SinkAdded";
    const INTERFACE: &'static str = "org.xetibo.ReSet";
}

impl GetVal<(Sink,)> for SinkAdded {
    fn get_value(&self) -> (Sink,) {
        (self.sink.clone(),)
    }
}

#[derive(Debug)]
pub struct SinkChanged {
    pub sink: Sink,
}

impl arg::AppendAll for SinkChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        self.sink.append_by_ref(i);
    }
}

impl arg::ReadAll for SinkChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SinkChanged { sink: i.read()? })
    }
}

impl dbus::message::SignalArgs for SinkChanged {
    const NAME: &'static str = "SinkChanged";
    const INTERFACE: &'static str = "org.xetibo.ReSet";
}

impl GetVal<(Sink,)> for SinkChanged {
    fn get_value(&self) -> (Sink,) {
        (self.sink.clone(),)
    }
}

#[derive(Debug)]
pub struct SinkRemoved {
    pub index: u32,
}

impl arg::AppendAll for SinkRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        self.index.append_by_ref(i);
    }
}

impl arg::ReadAll for SinkRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SinkRemoved { index: i.read()? })
    }
}

impl dbus::message::SignalArgs for SinkRemoved {
    const NAME: &'static str = "SinkRemoved";
    const INTERFACE: &'static str = "org.xetibo.ReSet";
}

impl GetVal<(u32,)> for SinkRemoved {
    fn get_value(&self) -> (u32,) {
        (self.index.clone(),)
    }
}

#[derive(Debug)]
pub struct InputStreamAdded {
    pub stream: InputStream,
}

impl arg::AppendAll for InputStreamAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        self.stream.append_by_ref(i);
    }
}

impl arg::ReadAll for InputStreamAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(InputStreamAdded { stream: i.read()? })
    }
}

impl dbus::message::SignalArgs for InputStreamAdded {
    const NAME: &'static str = "InputStreamAdded";
    const INTERFACE: &'static str = "org.xetibo.ReSet";
}

impl GetVal<(InputStream,)> for InputStreamAdded {
    fn get_value(&self) -> (InputStream,) {
        (self.stream.clone(),)
    }
}

#[derive(Debug)]
pub struct InputStreamChanged {
    pub stream: InputStream,
}

impl arg::AppendAll for InputStreamChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        self.stream.append_by_ref(i);
    }
}

impl arg::ReadAll for InputStreamChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(InputStreamChanged { stream: i.read()? })
    }
}

impl dbus::message::SignalArgs for InputStreamChanged {
    const NAME: &'static str = "InputStreamChanged";
    const INTERFACE: &'static str = "org.xetibo.ReSet";
}

#[derive(Debug)]
pub struct InputStreamRemoved {
    pub index: u32,
}

impl arg::AppendAll for InputStreamRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        self.index.append_by_ref(i);
    }
}

impl arg::ReadAll for InputStreamRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(InputStreamRemoved { index: i.read()? })
    }
}

impl dbus::message::SignalArgs for InputStreamRemoved {
    const NAME: &'static str = "InputStreamRemoved";
    const INTERFACE: &'static str = "org.xetibo.ReSet";
}

impl GetVal<(u32,)> for InputStreamRemoved {
    fn get_value(&self) -> (u32,) {
        (self.index.clone(),)
    }
}

#[derive(Debug)]
pub struct SourceAdded {
    pub source: Source,
}

impl arg::AppendAll for SourceAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        self.source.append_by_ref(i);
    }
}

impl arg::ReadAll for SourceAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SourceAdded { source: i.read()? })
    }
}

impl dbus::message::SignalArgs for SourceAdded {
    const NAME: &'static str = "SourceAdded";
    const INTERFACE: &'static str = "org.xetibo.ReSet";
}

impl GetVal<(Source,)> for SourceAdded {
    fn get_value(&self) -> (Source,) {
        (self.source.clone(),)
    }
}

#[derive(Debug)]
pub struct SourceChanged {
    pub source: Source,
}

impl arg::AppendAll for SourceChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        self.source.append_by_ref(i);
    }
}

impl arg::ReadAll for SourceChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SourceChanged { source: i.read()? })
    }
}

impl dbus::message::SignalArgs for SourceChanged {
    const NAME: &'static str = "SourceChanged";
    const INTERFACE: &'static str = "org.xetibo.ReSet";
}

impl GetVal<(Source,)> for SourceChanged {
    fn get_value(&self) -> (Source,) {
        (self.source.clone(),)
    }
}

#[derive(Debug)]
pub struct SourceRemoved {
    pub index: u32,
}

impl arg::AppendAll for SourceRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        self.index.append_by_ref(i);
    }
}

impl arg::ReadAll for SourceRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SourceRemoved { index: i.read()? })
    }
}

impl dbus::message::SignalArgs for SourceRemoved {
    const NAME: &'static str = "SourceRemoved";
    const INTERFACE: &'static str = "org.xetibo.ReSet";
}

impl GetVal<(u32,)> for SourceRemoved {
    fn get_value(&self) -> (u32,) {
        (self.index.clone(),)
    }
}

#[derive(Debug)]
pub struct OutputStreamAdded {
    pub stream: OutputStream,
}

impl arg::AppendAll for OutputStreamAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        self.stream.append_by_ref(i);
    }
}

impl arg::ReadAll for OutputStreamAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OutputStreamAdded { stream: i.read()? })
    }
}

impl dbus::message::SignalArgs for OutputStreamAdded {
    const NAME: &'static str = "OutputStreamAdded";
    const INTERFACE: &'static str = "org.xetibo.ReSet";
}

impl GetVal<(OutputStream,)> for OutputStreamAdded {
    fn get_value(&self) -> (OutputStream,) {
        (self.stream.clone(),)
    }
}

#[derive(Debug)]
pub struct OutputStreamChanged {
    pub stream: OutputStream,
}

impl arg::AppendAll for OutputStreamChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        self.stream.append_by_ref(i);
    }
}

impl arg::ReadAll for OutputStreamChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OutputStreamChanged { stream: i.read()? })
    }
}

impl dbus::message::SignalArgs for OutputStreamChanged {
    const NAME: &'static str = "OutputStreamChanged";
    const INTERFACE: &'static str = "org.xetibo.ReSet";
}

#[derive(Debug)]
pub struct OutputStreamRemoved {
    pub index: u32,
}

impl arg::AppendAll for OutputStreamRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        self.index.append_by_ref(i);
    }
}

impl arg::ReadAll for OutputStreamRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OutputStreamRemoved { index: i.read()? })
    }
}

impl dbus::message::SignalArgs for OutputStreamRemoved {
    const NAME: &'static str = "OutputStreamRemoved";
    const INTERFACE: &'static str = "org.xetibo.ReSet";
}

impl GetVal<(u32,)> for OutputStreamRemoved {
    fn get_value(&self) -> (u32,) {
        (self.index.clone(),)
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

        let mut conn = start_dbus_audio_listener(conn);

        if sink_box.is_some() {
            conn = start_output_box_listener(conn, sink_box.unwrap());
        }
        if source_box.is_some() {
            conn = start_input_box_listener(conn, source_box.unwrap());
        }

        listeners.pulse_listener.store(true, Ordering::SeqCst);
        println!("starting audio listener");
        loop {
            let _ = conn.process(Duration::from_millis(1000));
            if !listeners.pulse_listener.load(Ordering::SeqCst) {
                println!("stopping audio listener");
                stop_dbus_audio_listener(conn);
                break;
            }
            // thread::sleep(Duration::from_millis(1000));
            // TODO is this really how we should do this?
        }
    });
}

fn start_dbus_audio_listener(conn: Connection) -> Connection {
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let _: Result<(), Error> = proxy.method_call("org.xetibo.ReSet", "StartAudioListener", ());
    conn
}

fn stop_dbus_audio_listener(conn: Connection) {
    let proxy = conn.with_proxy(
        "org.xetibo.ReSet",
        "/org/xetibo/ReSet",
        Duration::from_millis(1000),
    );
    let _: Result<(), Error> = proxy.method_call("org.xetibo.ReSet", "StopAudioListener", ());
}

