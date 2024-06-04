use std::hint::{self};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use components::utils::{BASE, DBUS_PATH};
use components::window::reset_window::ReSetWindow;
use dbus::blocking::Connection;
use dbus::Error;
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{gio, Application, CssProvider};
use reset_daemon::run_daemon;

mod components;
mod tests;

const APP_ID: &str = "org.Xetibo.ReSet";

/// Version of the current package.
/// Use this to avoid version mismatch conflicts.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    tokio::task::spawn(daemon_check());
    gio::resources_register_include!("src.templates.gresource")
        .expect("Failed to register resources.");
    gio::resources_register_include!("src.icons.gresource").expect("Failed to register resources.");
    gio::resources_register_include!("src.style.gresource").expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(move |_| {
        adw::init().unwrap();
        load_css();
    });

    app.connect_activate(build_ui);
    app.connect_shutdown(shutdown);
    app.run();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/org/Xetibo/ReSet/style/style.css");

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let window = ReSetWindow::new(app);
    window.present();
}

fn shutdown(_: &Application) {
    thread::spawn(|| {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(100));
        let res: Result<(), Error> = proxy.method_call(BASE, "UnregisterClient", ("ReSet",));
        res
    });
}

async fn daemon_check() {
    let handle = thread::spawn(|| {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(100));
        let res: Result<(), Error> = proxy.method_call(BASE, "RegisterClient", ("ReSet",));
        res
    });
    let ready = Arc::new(AtomicBool::new(false));
    let res = handle.join();
    if res.unwrap().is_err() {
        run_daemon(Some(ready.clone())).await;
    }
    while !ready.load(std::sync::atomic::Ordering::SeqCst) {
        hint::spin_loop();
    }
}
