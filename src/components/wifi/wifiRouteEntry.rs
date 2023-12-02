use std::cell::RefCell;
use std::rc::Rc;
use adw::glib;
use adw::glib::Object;
use ReSet_Lib::network::connection::{Connection};
use crate::components::wifi::utils::IpProtocol;

use crate::components::wifi::wifiRouteEntryImpl;

glib::wrapper! {
    pub struct WifiRouteEntry(ObjectSubclass<wifiRouteEntryImpl::WifiRouteEntryImpl>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl WifiRouteEntry {
    pub fn new(address: Option<usize>, rc: Rc<RefCell<Connection>>, protocol: IpProtocol) -> Self {
        let entry: WifiRouteEntry = Object::builder().build();
        // if let Some(address) = address {
        //     let entryImp = entry.imp();
        //     let map = address.to_map();
        // 
        //     let addr = getValueFromKey(&map, "address");
        //     let prefix =  getValueFromKey(&map, "prefix-length");
        //     let gateway =  getValueFromKey(&map, "gateway");
        //     let metric =  getValueFromKey(&map, "metric");
        // 
        //     entryImp.resetRouteAddress.set_text(&addr);
        //     entryImp.resetRouteNetmask.set_text(&prefix);
        //     entryImp.resetRouteGateway.set_text(&gateway);
        //     entryImp.resetRouteMetric.set_text(&metric);
        //     entryImp.resetRouteRow.set_title(&format!("{}, {}, {}, {}", addr, prefix, gateway, metric));
        // }
        entry
    }
}

