use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

use dbus::{
    arg::{self, Append},
    blocking::Connection,
    Error,
};
use ReSet_Lib::{
    audio::audio::{InputStream, Sink},
    signals::GetVal,
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
