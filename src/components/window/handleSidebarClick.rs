use std::sync::Arc;

use crate::components::audio::audioBox::AudioBox;
use crate::components::base::settingBox::SettingBox;
use crate::components::bluetooth::bluetoothBox::BluetoothBox;
use crate::components::wifi::wifiBox::{scanForWifi, show_stored_connections, WifiBox};
use adw::prelude::ButtonExt;
use glib::clone;
use glib::subclass::types::ObjectSubclassIsExt;
use gtk::{FlowBox, Label};

pub const HANDLE_CONNECTIVITY_CLICK: fn(FlowBox) = |resetMain: FlowBox| {
    let wifiBox = Arc::new(WifiBox::new());
    show_stored_connections(wifiBox.clone());
    scanForWifi(wifiBox.clone());
    let wifiBox = SettingBox::new(&*wifiBox, "WiFi");
    let bluetoothBox = SettingBox::new(&BluetoothBox::new(), "Bluetooth");
    resetMain.remove_all();
    resetMain.insert(&wifiBox, -1);
    resetMain.insert(&bluetoothBox, -1);
    resetMain.set_max_children_per_line(2);
};

pub const HANDLE_WIFI_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let wifiBox = Arc::new(WifiBox::new());
    scanForWifi(wifiBox.clone());
    let wifiBox = SettingBox::new(&*wifiBox, "WiFi");
    resetMain.remove_all();
    resetMain.insert(&wifiBox, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_BLUETOOTH_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let bluetoothBox = SettingBox::new(&BluetoothBox::new(), "Bluetooth");
    resetMain.remove_all();
    resetMain.insert(&bluetoothBox, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_VPN_CLICK: fn(FlowBox) = |resetMain: FlowBox| {
    let label = Label::new(Some("not implemented yet"));
    resetMain.remove_all();
    resetMain.insert(&label, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_AUDIO_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let audioBox = SettingBox::new(&AudioBox::new(), "Audio");
    resetMain.remove_all();
    resetMain.insert(&audioBox, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_VOLUME_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let audioBox = SettingBox::new(&AudioBox::new(), "Audio");
    resetMain.remove_all();
    resetMain.insert(&audioBox, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_MICROPHONE_CLICK: fn(FlowBox) = |resetMain: FlowBox| {
    let label = Label::new(Some("not implemented yet"));
    resetMain.remove_all();
    resetMain.insert(&label, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_PERIPHERALS_CLICK: fn(FlowBox) = |resetMain: FlowBox| {
    let label = Label::new(Some("not implemented yet"));
    resetMain.remove_all();
    resetMain.insert(&label, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_MONITOR_CLICK: fn(FlowBox) = |resetMain: FlowBox| {
    let label = Label::new(Some("not implemented yet"));
    resetMain.remove_all();
    resetMain.insert(&label, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_MOUSE_CLICK: fn(FlowBox) = |resetMain: FlowBox| {
    let label = Label::new(Some("not implemented yet"));
    resetMain.remove_all();
    resetMain.insert(&label, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_KEYBOARD_CLICK: fn(FlowBox) = |resetMain: FlowBox| {
    let label = Label::new(Some("not implemented yet"));
    resetMain.remove_all();
    resetMain.insert(&label, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_HOME: fn(FlowBox) =  |resetMain: FlowBox|   {
    resetMain.remove_all();
};
