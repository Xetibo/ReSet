use gtk::FlowBox;
use crate::audio::AudioBox;
use crate::wifi::WifiBox;

pub const HANDLE_CONNECTIVITY_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let wifibox = WifiBox::new();
    resetMain.remove_all();
    resetMain.insert(&wifibox, -1);
};

pub const HANDLE_WIFI_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let wifibox = WifiBox::new();
    resetMain.remove_all();
    resetMain.insert(&wifibox, -1);
};

pub const HANDLE_BLUETOOTH_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let wifibox = WifiBox::new();
    resetMain.remove_all();
    resetMain.insert(&wifibox, -1);
};

pub const HANDLE_VPN_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let wifibox = WifiBox::new();
    resetMain.remove_all();
    resetMain.insert(&wifibox, -1);
};

pub const HANDLE_AUDIO_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let audioBox = AudioBox::new();
    resetMain.remove_all();
    resetMain.insert(&audioBox, -1);
};

pub const HANDLE_VOLUME_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let wifibox = WifiBox::new();
    resetMain.remove_all();
    resetMain.insert(&wifibox, -1);
};

pub const HANDLE_MICROPHONE_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let wifibox = WifiBox::new();
    resetMain.remove_all();
    resetMain.insert(&wifibox, -1);
};

pub const HANDLE_HOME: fn(FlowBox) =  |resetMain: FlowBox|   {
    resetMain.remove_all();
};
