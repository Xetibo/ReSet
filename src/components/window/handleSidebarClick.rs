use gtk::prelude::FrameExt;
use std::sync::Arc;

use crate::components::output::audioBox::AudioBox;
use crate::components::base::settingBox::SettingBox;
use crate::components::bluetooth::bluetoothBox::BluetoothBox;
use crate::components::wifi::wifiBox::{scanForWifi, show_stored_connections, WifiBox};
use gtk::{FlowBox, Frame, Label,};
use gtk::prelude::WidgetExt;
use crate::components::input::sourceBox;
use crate::components::input::sourceBox::SourceBox;

pub const HANDLE_CONNECTIVITY_CLICK: fn(FlowBox) = |resetMain: FlowBox| {
    let wifiBox = Arc::new(WifiBox::new());
    show_stored_connections(wifiBox.clone());
    scanForWifi(wifiBox.clone());
    let wifiFrame = wrapInFrame(SettingBox::new(&*wifiBox));
    let bluetoothFrame = wrapInFrame(SettingBox::new(&BluetoothBox::new()));
    resetMain.remove_all();
    resetMain.insert(&wifiFrame, -1);
    resetMain.insert(&bluetoothFrame, -1);
    resetMain.set_max_children_per_line(2);
};

pub const HANDLE_WIFI_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let wifiBox = Arc::new(WifiBox::new());
    scanForWifi(wifiBox.clone());
    let wifiFrame = wrapInFrame(SettingBox::new(&*wifiBox));
    resetMain.remove_all();
    resetMain.insert(&wifiFrame, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_BLUETOOTH_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let bluetoothFrame = wrapInFrame(SettingBox::new(&BluetoothBox::new()));
    resetMain.remove_all();
    resetMain.insert(&bluetoothFrame, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_VPN_CLICK: fn(FlowBox) = |resetMain: FlowBox| {
    let label = Label::new(Some("not implemented yet"));
    resetMain.remove_all();
    resetMain.insert(&label, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_AUDIO_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let audioFrame = wrapInFrame(SettingBox::new(&AudioBox::new()));
    let sourceFrame = wrapInFrame(SettingBox::new(&SourceBox::new()));
    resetMain.remove_all();
    resetMain.insert(&audioFrame, -1);
    resetMain.insert(&sourceFrame, -1);
    resetMain.set_max_children_per_line(2);
};

pub const HANDLE_VOLUME_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let audioFrame = wrapInFrame(SettingBox::new(&AudioBox::new()));
    resetMain.remove_all();
    resetMain.insert(&audioFrame, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_MICROPHONE_CLICK: fn(FlowBox) = |resetMain: FlowBox| {
    let sourceFrame = wrapInFrame(SettingBox::new(&SourceBox::new()));
    resetMain.remove_all();
    resetMain.insert(&sourceFrame, -1);
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

fn wrapInFrame(widget: SettingBox) -> Frame {
    let frame = Frame::new(None);
    frame.set_child(Some(&widget));
    frame.add_css_class("resetSettingFrame");
    frame
}
