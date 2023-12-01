use dbus::arg::RefArg;
use dbus::blocking::Connection;
use dbus::Error;
use dbus::Path;
use std::collections::HashMap;
use std::time::Duration;
use ReSet_Lib::network::connection::Connection as ResetConnection;

type ResultType =
    Result<(HashMap<String, HashMap<String, dbus::arg::Variant<Box<dyn RefArg>>>>,), Error>;

pub fn getConnectionSettings(path: Path<'static>) -> ResetConnection {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ReSetDaemon",
        Duration::from_millis(1000),
    );
    let res: ResultType =
        proxy.method_call("org.xetibo.ReSetWireless", "GetConnectionSettings", (path,));
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
