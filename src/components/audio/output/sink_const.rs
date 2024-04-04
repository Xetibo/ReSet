use crate::components::audio::audio_entry::{AudioIcons, DBusFunction};

pub const ICONS: AudioIcons = AudioIcons {
    muted: "audio-volume-muted-symbolic",
    active: "audio-volume-high-symbolic",
};

pub const SETVOLUME: DBusFunction = DBusFunction {
    function: "SetSinkVolume",
    error: "Failed to set sink volume",
};

pub const SETMUTE: DBusFunction = DBusFunction {
    function: "SetSinkMute",
    error: "Failed to mute sink",
};

pub const SETDEFAULT: DBusFunction = DBusFunction {
    function: "SetDefaultSink",
    error: "Failed to set default sink",
};

pub const GETDEFAULT: DBusFunction = DBusFunction {
    function: "GetDefaultSink",
    error: "Failed to get default sink",
};

pub const GETDEFAULTNAME: DBusFunction = DBusFunction {
    function: "GetDefaultSinkName",
    error: "Failed to get default sink name",
};

pub const GETOBJECTS: DBusFunction = DBusFunction {
    function: "ListSinks",
    error: "Failed to list sinks",
};

pub const GETSTREAMS: DBusFunction = DBusFunction {
    function: "ListInputStreams",
    error: "Failed to list input streams",
};

pub const SETSTREAMVOLUME: DBusFunction = DBusFunction {
    function: "SetInputStreamVolume",
    error: "Failed to set input stream volume",
};

pub const SETSTREAMMUTE: DBusFunction = DBusFunction {
    function: "SetInputStreamMute",
    error: "Failed to mute input stream",
};

pub const SETSTREAMOBJECT: DBusFunction = DBusFunction {
    function: "SetSinkOfInputStream",
    error: "Failed to set sink of input stream",
};
