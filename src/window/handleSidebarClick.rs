use gtk::FlowBox;
use crate::wifi::WifiBox;

pub const HANDLE_WIFI_CLICK: fn(FlowBox) =  |resetMain: FlowBox|   {
    let wifibox = WifiBox::new();
    resetMain.remove_all();
    resetMain.insert(&wifibox, -1);
};

pub const HANDLE_HOME: fn(FlowBox) =  |resetMain: FlowBox|   {
    resetMain.remove_all();
};
