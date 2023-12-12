use dbus::arg::RefArg;
use dbus::blocking::Connection;
use dbus::Error;
use dbus::Path;
use re_set_lib::network::connection::Connection as ResetConnection;
use std::collections::HashMap;
use std::time::Duration;

use crate::components::utils::BASE;
use crate::components::utils::DBUS_PATH;
use crate::components::utils::WIRELESS;

#[derive(Default, Copy, Clone)]
pub enum IpProtocol {
    #[default]
    IPv4,
    IPv6,
}

type ResultType =
    Result<(HashMap<String, HashMap<String, dbus::arg::Variant<Box<dyn RefArg>>>>,), Error>;

pub fn get_connection_settings(path: Path<'static>) -> ResetConnection {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        BASE,
        DBUS_PATH,
        Duration::from_millis(1000),
    );
    let res: ResultType =
        proxy.method_call(WIRELESS, "GetConnectionSettings", (path,));
    if res.is_err() {
        ResetConnection::default();
    }
    let (res,) = res.unwrap();
    let res = ResetConnection::convert_from_propmap(res);
    if res.is_err() {
        ResetConnection::default();
    }
    res.unwrap()
}
