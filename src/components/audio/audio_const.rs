use super::{audio_entry::DBusFunction};

pub const GETCARDS: DBusFunction = DBusFunction {
    function: "ListCards",
    error: "Failed to get list profiles",
};
