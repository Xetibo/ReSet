use gtk::prelude::FrameExt;
use std::cell::RefCell;
use std::hint::spin_loop;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::components::audio::input::source_box::{populate_sources, SourceBox};
use crate::components::audio::output::sink_box::{populate_sinks, SinkBox};
use crate::components::base::setting_box::SettingBox;
use crate::components::base::utils::{start_audio_listener, Listeners, Position};
use crate::components::bluetooth::bluetooth_box::{
    populate_connected_bluetooth_devices, start_bluetooth_listener, BluetoothBox,
};
use crate::components::wifi::wifi_box::{
    scan_for_wifi, show_stored_connections, start_event_listener, WifiBox,
};
use gtk::prelude::WidgetExt;
use gtk::{Align, FlowBox, FlowBoxChild, Frame};

pub const HANDLE_CONNECTIVITY_CLICK: fn(Arc<Listeners>, FlowBox, Rc<RefCell<Position>>) =
    |listeners: Arc<Listeners>, reset_main: FlowBox, position: Rc<RefCell<Position>>| {
        if handle_init(listeners.clone(), position, Position::Connectivity) {
            return;
        }
        let wifi_box = WifiBox::new(listeners.clone());
        start_event_listener(listeners.clone(), wifi_box.clone());
        show_stored_connections(wifi_box.clone());
        scan_for_wifi(wifi_box.clone());
        let wifi_frame = wrap_in_flow_box_child(SettingBox::new(&*wifi_box));
        let bluetooth_box = BluetoothBox::new(listeners.clone());
        populate_connected_bluetooth_devices(listeners, bluetooth_box.clone());
        // start_bluetooth_listener(listeners, bluetooth_box.clone());
        let bluetooth_frame = wrap_in_flow_box_child(SettingBox::new(&*bluetooth_box));
        reset_main.remove_all();
        reset_main.insert(&wifi_frame, -1);
        reset_main.insert(&bluetooth_frame, -1);
        reset_main.set_max_children_per_line(2);
    };

pub const HANDLE_WIFI_CLICK: fn(Arc<Listeners>, FlowBox, Rc<RefCell<Position>>) =
    |listeners: Arc<Listeners>, reset_main: FlowBox, position: Rc<RefCell<Position>>| {
        if handle_init(listeners.clone(), position, Position::Wifi) {
            return;
        }
        let wifi_box = WifiBox::new(listeners.clone());
        start_event_listener(listeners, wifi_box.clone());
        show_stored_connections(wifi_box.clone());
        scan_for_wifi(wifi_box.clone());
        let wifi_frame = wrap_in_flow_box_child(SettingBox::new(&*wifi_box));
        reset_main.remove_all();
        reset_main.insert(&wifi_frame, -1);
        reset_main.set_max_children_per_line(1);
    };

pub const HANDLE_BLUETOOTH_CLICK: fn(Arc<Listeners>, FlowBox, Rc<RefCell<Position>>) =
    |listeners: Arc<Listeners>, reset_main: FlowBox, position: Rc<RefCell<Position>>| {
        if handle_init(listeners.clone(), position, Position::Bluetooth) {
            return;
        }
        let bluetooth_box = BluetoothBox::new(listeners.clone());
        populate_connected_bluetooth_devices(listeners, bluetooth_box.clone());
        // start_bluetooth_listener(listeners, bluetooth_box.clone());
        let bluetooth_frame = wrap_in_flow_box_child(SettingBox::new(&*bluetooth_box));
        reset_main.remove_all();
        reset_main.insert(&bluetooth_frame, -1);
        reset_main.set_max_children_per_line(1);
    };

pub const HANDLE_AUDIO_CLICK: fn(Arc<Listeners>, FlowBox, Rc<RefCell<Position>>) =
    |listeners: Arc<Listeners>, reset_main: FlowBox, position: Rc<RefCell<Position>>| {
        if handle_init(listeners.clone(), position, Position::Audio) {
            return;
        }
        let audio_output = Arc::new(SinkBox::new());
        let audio_input = Arc::new(SourceBox::new());
        start_audio_listener(
            listeners.clone(),
            Some(audio_output.clone()),
            Some(audio_input.clone()),
        );
        if !listeners.pulse_listener.load(Ordering::SeqCst) {
            spin_loop();
        }
        populate_sinks(audio_output.clone());
        populate_sources(audio_input.clone());
        let sink_frame = wrap_in_flow_box_child(SettingBox::new(&*audio_output));
        let source_frame = wrap_in_flow_box_child(SettingBox::new(&*audio_input));
        reset_main.remove_all();
        reset_main.insert(&sink_frame, -1);
        reset_main.insert(&source_frame, -1);
        reset_main.set_max_children_per_line(2);
    };

pub const HANDLE_VOLUME_CLICK: fn(Arc<Listeners>, FlowBox, Rc<RefCell<Position>>) =
    |listeners: Arc<Listeners>, reset_main: FlowBox, position: Rc<RefCell<Position>>| {
        if handle_init(listeners.clone(), position, Position::AudioOutput) {
            return;
        }
        let audio_output = Arc::new(SinkBox::new());
        start_audio_listener(listeners.clone(), Some(audio_output.clone()), None);
        if !listeners.pulse_listener.load(Ordering::SeqCst) {
            spin_loop();
        }
        populate_sinks(audio_output.clone());
        let audio_frame = wrap_in_flow_box_child(SettingBox::new(&*audio_output));
        reset_main.remove_all();
        reset_main.insert(&audio_frame, -1);
        reset_main.set_max_children_per_line(1);
    };

pub const HANDLE_MICROPHONE_CLICK: fn(Arc<Listeners>, FlowBox, Rc<RefCell<Position>>) =
    |listeners: Arc<Listeners>, reset_main: FlowBox, position: Rc<RefCell<Position>>| {
        if handle_init(listeners.clone(), position, Position::AudioInput) {
            return;
        }
        let audio_input = Arc::new(SourceBox::new());
        start_audio_listener(listeners.clone(), None, Some(audio_input.clone()));
        if !listeners.pulse_listener.load(Ordering::SeqCst) {
            spin_loop();
        }
        populate_sources(audio_input.clone());
        let source_frame = wrap_in_flow_box_child(SettingBox::new(&*audio_input));
        reset_main.remove_all();
        reset_main.insert(&source_frame, -1);
        reset_main.set_max_children_per_line(1);
    };

pub const HANDLE_HOME: fn(Arc<Listeners>, FlowBox, Rc<RefCell<Position>>) =
    |listeners: Arc<Listeners>, reset_main: FlowBox, position: Rc<RefCell<Position>>| {
        if handle_init(listeners, position, Position::Home) {
            return;
        }
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

fn handle_init(
    listeners: Arc<Listeners>,
    position: Rc<RefCell<Position>>,
    clicked_position: Position,
) -> bool {
    {
        let mut pos_borrow = position.borrow_mut();
        if *pos_borrow == clicked_position {
            return true;
        }
        *pos_borrow = clicked_position;
    }
    listeners.stop_network_listener();
    listeners.stop_audio_listener();
    listeners.stop_bluetooth_listener();
    false
}
