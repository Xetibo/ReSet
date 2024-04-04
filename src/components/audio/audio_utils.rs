use std::{sync::Arc, time::Duration};

use dbus::{
    arg::{AppendAll, ReadAll},
    blocking::Connection,
    Error,
};

use crate::components::{
    base::error_impl::{show_error, ReSetErrorImpl},
    utils::{AUDIO, BASE, DBUS_PATH},
};

use super::audio_entry::DBusFunction;

pub fn audio_dbus_call<B, O, I>(
    source_box: Arc<B>,
    args: I,
    function: &'static DBusFunction,
) -> Option<O>
where
    O: ReadAll,
    I: AppendAll,
    B: ReSetErrorImpl + 'static,
{
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let res: Result<O, Error> = proxy.method_call(AUDIO, function.function, args);
    if res.is_err() {
        show_error::<B>(source_box.clone(), function.error);
        return None;
    }
    Some(res.unwrap())
}
