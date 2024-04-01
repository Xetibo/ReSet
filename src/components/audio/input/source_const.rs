use crate::components::audio::generic_entry::{AudioIcons, DBusFunction};

pub const ICONS: AudioIcons = AudioIcons {
    muted: "audio-input-microphone-symbolic",
    active: "microphone-disabled-symbolic",
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
