use crate::components::audio::audio_entry::{AudioIcons, DBusFunction};

pub const ICONS: AudioIcons = AudioIcons {
    muted: "microphone-disabled-symbolic",
    active: "audio-input-microphone-symbolic",
};

pub const SETVOLUME: DBusFunction = DBusFunction {
    function: "SetSourceVolume",
    error: "Failed to set source volume",
};

pub const SETMUTE: DBusFunction = DBusFunction {
    function: "SetSourceMute",
    error: "Failed to mute source",
};

pub const SETDEFAULT: DBusFunction = DBusFunction {
    function: "SetDefaultSource",
    error: "Failed to set default source",
};

pub const GETDEFAULT: DBusFunction = DBusFunction {
    function: "GetDefaultSource",
    error: "Failed to get default source",
};

pub const GETDEFAULTNAME: DBusFunction = DBusFunction {
    function: "GetDefaultSourceName",
    error: "Failed to get default source name",
};

pub const GETOBJECTS: DBusFunction = DBusFunction {
    function: "ListSources",
    error: "Failed to list sources",
};

pub const GETSTREAMS: DBusFunction = DBusFunction {
    function: "ListOutputStreams",
    error: "Failed to list output streams",
};

pub const SETSTREAMVOLUME: DBusFunction = DBusFunction {
    function: "SetOutputStreamVolume",
    error: "Failed to set output stream volume",
};

pub const SETSTREAMMUTE: DBusFunction = DBusFunction {
    function: "SetOutputStreamMute",
    error: "Failed to mute output stream",
};

pub const SETSTREAMOBJECT: DBusFunction = DBusFunction {
    function: "SetSourceOfOutputStream",
    error: "Failed to set source of output stream",
};

pub const DUMMY: &str = "Monitor of Dummy Output";
