use gtk::{FlowBox, Label};
use gtk::prelude::WidgetExt;
use crate::components::audio::audioBox::AudioBox;
use crate::components::bluetooth::bluetoothBox::BluetoothBox;
use crate::components::temp::settingBox::SettingBox;
use crate::components::wifi::wifiBox::WifiBox;

pub const HANDLE_CONNECTIVITY_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let wifiBox = SettingBox::new(&WifiBox::new());
    let bluetoothBox = SettingBox::new(&BluetoothBox::new());
    wifiBox.set_width_request(500); // todo why not working from ui file
    bluetoothBox.set_width_request(500); // todo why not working from ui file
    resetMain.remove_all();
    resetMain.insert(&wifiBox, -1);
    resetMain.insert(&bluetoothBox, -1);
    resetMain.set_max_children_per_line(2);
};

pub const HANDLE_WIFI_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let wifiBox = SettingBox::new(&WifiBox::new());
    wifiBox.set_width_request(500); // todo why not working from ui file
    resetMain.remove_all();
    resetMain.insert(&wifiBox, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_BLUETOOTH_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let bluetoothBox = SettingBox::new(&BluetoothBox::new());
    bluetoothBox.set_width_request(500); // todo why not working from ui file
    resetMain.remove_all();
    resetMain.insert(&bluetoothBox, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_VPN_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let label = Label::new(Some("not implemented yet"));
    resetMain.remove_all();
    resetMain.insert(&label, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_AUDIO_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let audioBox = AudioBox::new();
    resetMain.remove_all();
    resetMain.insert(&audioBox, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_VOLUME_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let audioBox = AudioBox::new();
    resetMain.remove_all();
    resetMain.insert(&audioBox, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_MICROPHONE_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let label = Label::new(Some("not implemented yet"));
    resetMain.remove_all();
    resetMain.insert(&label, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_HOME: fn(FlowBox) =  |resetMain: FlowBox|   {
    resetMain.remove_all();
};

pub const HANDLE_PERIPHERALS_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let label = Label::new(Some("not implemented yet"));
    resetMain.remove_all();
    resetMain.insert(&label, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_MONITOR_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let label = Label::new(Some("not implemented yet"));
    resetMain.remove_all();
    resetMain.insert(&label, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_MOUSE_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let label = Label::new(Some("not implemented yet"));
    resetMain.remove_all();
    resetMain.insert(&label, -1);
    resetMain.set_max_children_per_line(1);
};

pub const HANDLE_KEYBOARD_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let label = Label::new(Some("not implemented yet"));
    resetMain.remove_all();
    resetMain.insert(&label, -1);
    resetMain.set_max_children_per_line(1);
};
