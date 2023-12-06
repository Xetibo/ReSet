use gtk::prelude::FrameExt;
use std::sync::Arc;

use crate::components::base::setting_box::SettingBox;
use crate::components::base::utils::{start_audio_listener, Listeners};
use crate::components::bluetooth::bluetooth_box::{
    populate_conntected_bluetooth_devices, start_bluetooth_listener, BluetoothBox,
};
use crate::components::input::source_box::{populate_sources, SourceBox};
use crate::components::output::sink_box::{populate_sinks, SinkBox};
use crate::components::wifi::wifi_box::{
    scan_for_wifi, show_stored_connections, start_event_listener, WifiBox,
};
use gtk::prelude::WidgetExt;
use gtk::{Align, FlowBox, FlowBoxChild, Frame};

pub const HANDLE_CONNECTIVITY_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, reset_main: FlowBox| {
        listeners.stop_audio_listener();
        let wifi_box = WifiBox::new(listeners.clone());
        start_event_listener(listeners.clone(), wifi_box.clone());
        show_stored_connections(wifi_box.clone());
        scan_for_wifi(wifi_box.clone());
        let wifi_frame = wrap_in_flow_box_child(SettingBox::new(&*wifi_box));
        let bluetooth_box = BluetoothBox::new(listeners.clone());
        populate_conntected_bluetooth_devices(bluetooth_box.clone());
        start_bluetooth_listener(listeners.clone(), bluetooth_box.clone());
        let bluetooth_frame = wrap_in_flow_box_child(SettingBox::new(&*bluetooth_box));
        reset_main.remove_all();
        reset_main.insert(&wifi_frame, -1);
        reset_main.insert(&bluetooth_frame, -1);
        reset_main.set_max_children_per_line(2);
    };

pub const HANDLE_WIFI_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, reset_main: FlowBox| {
        listeners.stop_audio_listener();
        listeners.stop_bluetooth_listener();
        let wifi_box = WifiBox::new(listeners.clone());
        start_event_listener(listeners.clone(), wifi_box.clone());
        show_stored_connections(wifi_box.clone());
        scan_for_wifi(wifi_box.clone());
        let wifi_frame = wrap_in_flow_box_child(SettingBox::new(&*wifi_box));
        reset_main.remove_all();
        reset_main.insert(&wifi_frame, -1);
        reset_main.set_max_children_per_line(1);
    };

pub const HANDLE_BLUETOOTH_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, reset_main: FlowBox| {
        listeners.stop_network_listener();
        listeners.stop_audio_listener();
        let bluetooth_box = BluetoothBox::new(listeners.clone());
        start_bluetooth_listener(listeners.clone(), bluetooth_box.clone());
        populate_conntected_bluetooth_devices(bluetooth_box.clone());
        let bluetooth_frame = wrap_in_flow_box_child(SettingBox::new(&*bluetooth_box));
        reset_main.remove_all();
        reset_main.insert(&bluetooth_frame, -1);
        reset_main.set_max_children_per_line(1);
    };

pub const HANDLE_AUDIO_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, reset_main: FlowBox| {
        listeners.stop_network_listener();
        listeners.stop_bluetooth_listener();
        let audio_output = Arc::new(SinkBox::new());
        let audio_input = Arc::new(SourceBox::new());
        start_audio_listener(
            listeners.clone(),
            Some(audio_output.clone()),
            Some(audio_input.clone()),
        );
        populate_sinks(audio_output.clone());
        let audio_frame = wrap_in_flow_box_child(SettingBox::new(&*audio_output));
        populate_sources(audio_input.clone());
        let source_frame = wrap_in_flow_box_child(SettingBox::new(&*audio_input));
        reset_main.remove_all();
        reset_main.insert(&audio_frame, -1);
        reset_main.insert(&source_frame, -1);
        reset_main.set_max_children_per_line(2);
    };

pub const HANDLE_VOLUME_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, reset_main: FlowBox| {
        listeners.stop_network_listener();
        listeners.stop_bluetooth_listener();
        let audio_output = Arc::new(SinkBox::new());
        start_audio_listener(listeners.clone(), Some(audio_output.clone()), None);
        populate_sinks(audio_output.clone());
        let audio_frame = wrap_in_flow_box_child(SettingBox::new(&*audio_output));
        reset_main.remove_all();
        reset_main.insert(&audio_frame, -1);
        reset_main.set_max_children_per_line(1);
    };

pub const HANDLE_MICROPHONE_CLICK: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, reset_main: FlowBox| {
        listeners.stop_network_listener();
        listeners.stop_bluetooth_listener();
        let audio_input = Arc::new(SourceBox::new());
        start_audio_listener(listeners.clone(), None, Some(audio_input.clone()));
        populate_sources(audio_input.clone());
        let source_frame = wrap_in_flow_box_child(SettingBox::new(&*audio_input));
        reset_main.remove_all();
        reset_main.insert(&source_frame, -1);
        reset_main.set_max_children_per_line(1);
    };

pub const HANDLE_HOME: fn(Arc<Listeners>, FlowBox) =
    |listeners: Arc<Listeners>, reset_main: FlowBox| {
        listeners.stop_network_listener();
        listeners.stop_audio_listener();
        listeners.stop_bluetooth_listener();
        reset_main.remove_all();
    };

fn wrap_in_flow_box_child(widget: SettingBox) -> FlowBoxChild {
    let frame = Frame::new(None);
    frame.set_child(Some(&widget));
    frame.add_css_class("resetSettingFrame");
    FlowBoxChild::builder()
        .child(&frame)
        .halign(Align::Fill)
        .valign(Align::Start)
        .build()
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
