use gtk::{FlowBox, FlowBoxChild, Label};
use gtk::prelude::FlowBoxChildExt;
use crate::components::audio::audioBox::AudioBox;
use crate::components::bluetooth::bluetoothBox::BluetoothBox;
use crate::components::wifi::wifiBox::WifiBox;

pub const HANDLE_CONNECTIVITY_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let wifiBox = WifiBox::new();
    let bluetoothBox = BluetoothBox::new();
    resetMain.remove_all();
    resetMain.insert(&wifiBox, -1);
    resetMain.insert(&bluetoothBox, -1);
    // todo center flowbox children
};

pub const HANDLE_WIFI_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let wifibox = WifiBox::new();
    resetMain.remove_all();
    let child = FlowBoxChild::new();
    child.set_child(Some(&wifibox));
    resetMain.insert(&child, -1);
};

pub const HANDLE_BLUETOOTH_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let bluetoothBox = BluetoothBox::new();
    resetMain.remove_all();
    resetMain.insert(&bluetoothBox, -1);
};

pub const HANDLE_VPN_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let label = Label::new(Some("not implemented yet"));
    resetMain.remove_all();
    resetMain.insert(&label, -1);
};

pub const HANDLE_AUDIO_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let audioBox = AudioBox::new();
    resetMain.remove_all();
    resetMain.insert(&audioBox, -1);
};

pub const HANDLE_VOLUME_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let audioBox = AudioBox::new();
    resetMain.remove_all();
    resetMain.insert(&audioBox, -1);
};

pub const HANDLE_MICROPHONE_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let label = Label::new(Some("not implemented yet"));
    resetMain.remove_all();
    resetMain.insert(&label, -1);
};

pub const HANDLE_HOME: fn(FlowBox) =  |resetMain: FlowBox|   {
    resetMain.remove_all();
};
