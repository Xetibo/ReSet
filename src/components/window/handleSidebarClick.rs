use gtk::prelude::FrameExt;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::components::base::settingBox::SettingBox;
use crate::components::base::utils::{start_audio_listener, Listeners};
use crate::components::bluetooth::bluetoothBox::{
    populate_conntected_bluetooth_devices, start_bluetooth_listener, BluetoothBox,
};
use crate::components::input::sourceBox::{populate_sources, SourceBox};
use crate::components::output::sinkBox::{populate_sinks, SinkBox};
use crate::components::wifi::wifiBox::{
    scanForWifi, show_stored_connections, start_event_listener, WifiBox,
};
use gtk::prelude::WidgetExt;
use gtk::{FlowBox, Frame};

pub const HANDLE_CONNECTIVITY_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, resetMain: FlowBox| {
        listeners.stop_audio_listener();
        let wifiBox = WifiBox::new(listeners.clone());
        start_event_listener(listeners.clone(), wifiBox.clone());
        show_stored_connections(wifiBox.clone());
        scanForWifi(wifiBox.clone());
        let wifiFrame = wrapInFrame(SettingBox::new(&*wifiBox));
        let bluetooth_box = BluetoothBox::new(listeners.clone());
        populate_conntected_bluetooth_devices(bluetooth_box.clone());
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
        let wifiBox = WifiBox::new(listeners.clone());
        start_event_listener(listeners.clone(), wifiBox.clone());
        show_stored_connections(wifiBox.clone());
        scanForWifi(wifiBox.clone());
        let wifiFrame = wrapInFrame(SettingBox::new(&*wifiBox));
        resetMain.remove_all();
        resetMain.insert(&wifiFrame, -1);
        resetMain.set_max_children_per_line(1);
    };

pub const HANDLE_BLUETOOTH_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, resetMain: FlowBox| {
        listeners.stop_network_listener();
        listeners.stop_audio_listener();
        let bluetooth_box = BluetoothBox::new(listeners.clone());
        start_bluetooth_listener(listeners.clone(), bluetooth_box.clone());
        populate_conntected_bluetooth_devices(bluetooth_box.clone());
        let bluetoothFrame = wrapInFrame(SettingBox::new(&*bluetooth_box));
        resetMain.remove_all();
        resetMain.insert(&bluetoothFrame, -1);
        resetMain.set_max_children_per_line(1);
    };

pub const HANDLE_AUDIO_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, resetMain: FlowBox| {
        listeners.stop_network_listener();
        listeners.stop_bluetooth_listener();
        let audioOutput = Arc::new(SinkBox::new());
        let audioInput = Arc::new(SourceBox::new());
        start_audio_listener(
            listeners.clone(),
            Some(audioOutput.clone()),
            Some(audioInput.clone()),
        );
        populate_sinks(audioOutput.clone());
        let audioFrame = wrapInFrame(SettingBox::new(&*audioOutput));
        populate_sources(audioInput.clone());
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
        start_audio_listener(listeners.clone(), Some(audioOutput.clone()), None);
        while !listeners.pulse_listener.load(Ordering::SeqCst) {
            std::hint::spin_loop()
        }
        populate_sinks(audioOutput.clone());
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
        start_audio_listener(listeners.clone(), None, Some(audioInput.clone()));
        populate_sources(audioInput.clone());
        let sourceFrame = wrapInFrame(SettingBox::new(&*audioInput));
        resetMain.remove_all();
        resetMain.insert(&sourceFrame, -1);
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

// for future implementations
// pub const HANDLE_VPN_CLICK: fn(Arc<Listeners>, FlowBox) =
//     |listeners: Arc<Listeners>, resetMain: FlowBox| {
//         listeners.stop_network_listener();
//         listeners.stop_bluetooth_listener();
//         listeners.stop_audio_listener();
//         let label = Label::new(Some("not implemented yet"));
//         resetMain.remove_all();
//         resetMain.insert(&label, -1);
//         resetMain.set_max_children_per_line(1);
//     };
//
// pub const HANDLE_PERIPHERALS_CLICK: fn(Arc<Listeners>, FlowBox) =
//     |listeners: Arc<Listeners>, resetMain: FlowBox| {
//         listeners.stop_network_listener();
//         listeners.stop_audio_listener();
//         listeners.stop_bluetooth_listener();
//         let label = Label::new(Some("not implemented yet"));
//         resetMain.remove_all();
//         resetMain.insert(&label, -1);
//         resetMain.set_max_children_per_line(1);
//     };
//
// pub const HANDLE_MONITOR_CLICK: fn(Arc<Listeners>, FlowBox) =
//     |listeners: Arc<Listeners>, resetMain: FlowBox| {
//         listeners.stop_network_listener();
//         listeners.stop_audio_listener();
//         listeners.stop_bluetooth_listener();
//         let label = Label::new(Some("not implemented yet"));
//         resetMain.remove_all();
//         resetMain.insert(&label, -1);
//         resetMain.set_max_children_per_line(1);
//     };
//
// pub const HANDLE_MOUSE_CLICK: fn(Arc<Listeners>, FlowBox) =
//     |listeners: Arc<Listeners>, resetMain: FlowBox| {
//         listeners.stop_network_listener();
//         listeners.stop_audio_listener();
//         listeners.stop_bluetooth_listener();
//         let label = Label::new(Some("not implemented yet"));
//         resetMain.remove_all();
//         resetMain.insert(&label, -1);
//         resetMain.set_max_children_per_line(1);
//     };
//
// pub const HANDLE_KEYBOARD_CLICK: fn(Arc<Listeners>, FlowBox) =
//     |listeners: Arc<Listeners>, resetMain: FlowBox| {
//         listeners.stop_network_listener();
//         listeners.stop_audio_listener();
//         listeners.stop_bluetooth_listener();
//         let label = Label::new(Some("not implemented yet"));
//         resetMain.remove_all();
//         resetMain.insert(&label, -1);
//         resetMain.set_max_children_per_line(1);
//     };
//
