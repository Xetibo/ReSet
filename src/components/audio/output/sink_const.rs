use crate::components::audio::generic_entry::{AudioIcons, DBusFunction};

pub const ICONS: AudioIcons = AudioIcons {
    muted: "audio-volume-high-symbolic",
    active: "audio-volume-muted-symbolic",
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
