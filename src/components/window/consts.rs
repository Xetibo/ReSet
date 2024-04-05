use crate::components::plugin::function::ReSetSidebarInfo;

use super::handle_sidebar_click::{
    HANDLE_AUDIO_CLICK, HANDLE_BLUETOOTH_CLICK, HANDLE_CONNECTIVITY_CLICK, HANDLE_MICROPHONE_CLICK,
    HANDLE_VOLUME_CLICK, HANDLE_WIFI_CLICK,
};

pub const CONNECTIVITY_SIDEBAR: ReSetSidebarInfo = ReSetSidebarInfo {
    name: "Connectivity",
    icon_name: "network-wired-symbolic",
    parent: None,
    click_event: HANDLE_CONNECTIVITY_CLICK,
};

pub const WIFI_SIDEBAR: ReSetSidebarInfo = ReSetSidebarInfo {
    name: "WiFi",
    icon_name: "network-wireless-symbolic",
    parent: Some("Connectivity"),
    click_event: HANDLE_WIFI_CLICK,
};

pub const BLUETOOTH_SIDEBAR: ReSetSidebarInfo = ReSetSidebarInfo {
    name: "Bluetooth",
    icon_name: "bluetooth-symbolic",
    parent: Some("Connectivity"),
    click_event: HANDLE_BLUETOOTH_CLICK,
};

pub const AUDIO_SIDEBAR: ReSetSidebarInfo = ReSetSidebarInfo {
    name: "Audio",
    icon_name: "audio-headset-symbolic",
    parent: None,
    click_event: HANDLE_AUDIO_CLICK,
};

pub const SINK_SIDEBAR: ReSetSidebarInfo = ReSetSidebarInfo {
    name: "Output",
    icon_name: "audio-volume-high-symbolic",
    parent: Some("Audio"),
    click_event: HANDLE_VOLUME_CLICK,
};

pub const SOURCE_SIDEBAR: ReSetSidebarInfo = ReSetSidebarInfo {
    name: "Input",
    icon_name: "audio-input-microphone-symbolic",
    parent: Some("Audio"),
    click_event: HANDLE_MICROPHONE_CLICK,
};
