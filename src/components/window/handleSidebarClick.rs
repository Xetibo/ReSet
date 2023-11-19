use gtk::prelude::FrameExt;
use std::sync::Arc;

use crate::components::base::settingBox::SettingBox;
use crate::components::base::utils::{start_audio_listener, Listeners};
use crate::components::bluetooth::bluetoothBox::{start_bluetooth_listener, BluetoothBox};
use crate::components::input::sourceBox::{populate_sources, SourceBox};
use crate::components::output::sinkBox::{populate_sinks, SinkBox};
use crate::components::wifi::wifiBox::{scanForWifi, show_stored_connections, WifiBox};
use gtk::prelude::WidgetExt;
use gtk::{FlowBox, Frame, Label};

pub const HANDLE_CONNECTIVITY_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, resetMain: FlowBox| {
        listeners.stop_audio_listener();
        let wifiBox = Arc::new(WifiBox::new());
        show_stored_connections(wifiBox.clone());
        scanForWifi(listeners.clone(), wifiBox.clone());
        let wifiFrame = wrapInFrame(SettingBox::new(&*wifiBox));
        let bluetooth_box = Arc::new(BluetoothBox::new());
        start_bluetooth_listener(listeners.clone(), bluetooth_box.clone());
        let bluetoothFrame = wrapInFrame(SettingBox::new(&*bluetooth_box));
        resetMain.remove_all();
        resetMain.insert(&wifiFrame, -1);
        resetMain.insert(&bluetoothFrame, -1);
        resetMain.set_max_children_per_line(2);
    };

pub const HANDLE_WIFI_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, resetMain: FlowBox| {
        listeners.stop_audio_listener();
        listeners.stop_bluetooth_listener();
        let wifiBox = Arc::new(WifiBox::new());
        show_stored_connections(wifiBox.clone());
        scanForWifi(listeners.clone(), wifiBox.clone());
        let wifiFrame = wrapInFrame(SettingBox::new(&*wifiBox));
        resetMain.remove_all();
        resetMain.insert(&wifiFrame, -1);
        resetMain.set_max_children_per_line(1);
    };

pub const HANDLE_BLUETOOTH_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, resetMain: FlowBox| {
        listeners.stop_network_listener();
        listeners.stop_audio_listener();
        let bluetooth_box = Arc::new(BluetoothBox::new());
        start_bluetooth_listener(listeners.clone(), bluetooth_box.clone());
        let bluetoothFrame = wrapInFrame(SettingBox::new(&*bluetooth_box));
        resetMain.remove_all();
        resetMain.insert(&bluetoothFrame, -1);
        resetMain.set_max_children_per_line(1);
    };

pub const HANDLE_VPN_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, resetMain: FlowBox| {
        listeners.stop_network_listener();
        listeners.stop_bluetooth_listener();
        listeners.stop_audio_listener();
        let label = Label::new(Some("not implemented yet"));
        resetMain.remove_all();
        resetMain.insert(&label, -1);
        resetMain.set_max_children_per_line(1);
    };

pub const HANDLE_AUDIO_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, resetMain: FlowBox| {
        listeners.stop_network_listener();
        listeners.stop_bluetooth_listener();
        let audioOutput = Arc::new(SinkBox::new());
        populate_sinks(audioOutput.clone());
        let audioFrame = wrapInFrame(SettingBox::new(&*audioOutput));
        let audioInput = Arc::new(SourceBox::new());
        populate_sources(audioInput.clone());
        start_audio_listener(
            listeners.clone(),
            Some(audioOutput.clone()),
            Some(audioInput.clone()),
        );
        let sourceFrame = wrapInFrame(SettingBox::new(&*audioInput));
        resetMain.remove_all();
        resetMain.insert(&audioFrame, -1);
        resetMain.insert(&sourceFrame, -1);
        resetMain.set_max_children_per_line(2);
    };

pub const HANDLE_VOLUME_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, resetMain: FlowBox| {
        listeners.stop_network_listener();
        listeners.stop_bluetooth_listener();
        let audioOutput = Arc::new(SinkBox::new());
        populate_sinks(audioOutput.clone());
        start_audio_listener(listeners.clone(), Some(audioOutput.clone()), None);
        let audioFrame = wrapInFrame(SettingBox::new(&*audioOutput));
        resetMain.remove_all();
        resetMain.insert(&audioFrame, -1);
        resetMain.set_max_children_per_line(1);
    };

pub const HANDLE_MICROPHONE_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, resetMain: FlowBox| {
        listeners.stop_network_listener();
        listeners.stop_bluetooth_listener();
        let audioInput = Arc::new(SourceBox::new());
        populate_sources(audioInput.clone());
        start_audio_listener(listeners.clone(), None, Some(audioInput.clone()));
        let sourceFrame = wrapInFrame(SettingBox::new(&*audioInput));
        resetMain.remove_all();
        resetMain.insert(&sourceFrame, -1);
        resetMain.set_max_children_per_line(1);
    };

pub const HANDLE_PERIPHERALS_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, resetMain: FlowBox| {
        listeners.stop_network_listener();
        listeners.stop_audio_listener();
        listeners.stop_bluetooth_listener();
        let label = Label::new(Some("not implemented yet"));
        resetMain.remove_all();
        resetMain.insert(&label, -1);
        resetMain.set_max_children_per_line(1);
    };

pub const HANDLE_MONITOR_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, resetMain: FlowBox| {
        listeners.stop_network_listener();
        listeners.stop_audio_listener();
        listeners.stop_bluetooth_listener();
        let label = Label::new(Some("not implemented yet"));
        resetMain.remove_all();
        resetMain.insert(&label, -1);
        resetMain.set_max_children_per_line(1);
    };

pub const HANDLE_MOUSE_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, resetMain: FlowBox| {
        listeners.stop_network_listener();
        listeners.stop_audio_listener();
        listeners.stop_bluetooth_listener();
        let label = Label::new(Some("not implemented yet"));
        resetMain.remove_all();
        resetMain.insert(&label, -1);
        resetMain.set_max_children_per_line(1);
    };

pub const HANDLE_KEYBOARD_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, resetMain: FlowBox| {
        listeners.stop_network_listener();
        listeners.stop_audio_listener();
        listeners.stop_bluetooth_listener();
        let label = Label::new(Some("not implemented yet"));
        resetMain.remove_all();
        resetMain.insert(&label, -1);
        resetMain.set_max_children_per_line(1);
    };

pub const HANDLE_HOME: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, resetMain: FlowBox| {
        listeners.stop_network_listener();
        listeners.stop_audio_listener();
        listeners.stop_bluetooth_listener();
        resetMain.remove_all();
    };

fn wrapInFrame(widget: SettingBox) -> Frame {
    let frame = Frame::new(None);
    frame.set_child(Some(&widget));
    frame.add_css_class("resetSettingFrame");
    frame
}
