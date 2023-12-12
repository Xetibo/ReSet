use dbus::arg::RefArg;
use dbus::blocking::Connection;
use dbus::Error;
use dbus::Path;
use re_set_lib::network::connection::Connection as ResetConnection;
use std::collections::HashMap;
use std::time::Duration;

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
        "org.Xetibo.ReSet.Daemon",
        "/org/Xetibo/ReSet/Daemon",
        Duration::from_millis(1000),
    );
    let res: ResultType =
        proxy.method_call("org.Xetibo.ReSet.Wireless", "GetConnectionSettings", (path,));
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
