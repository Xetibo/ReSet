#![allow(non_snake_case)]

use std::thread;
use std::time::Duration;

use dbus::blocking::Connection;
use dbus::Error;
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{gio, Application, CssProvider};
use reset_daemon::run_daemon;

use crate::components::window::window::Window;

mod components;

const APP_ID: &str = "org.Xetibo.ReSet";

#[tokio::main]
async fn main() {
    // TODO is this the best way to handle this??

    tokio::task::spawn(daemon_check());
    gio::resources_register_include!("src.templates.gresource")
        .expect("Failed to register resources.");
    gio::resources_register_include!("src.icons.gresource").expect("Failed to register resources.");
    gio::resources_register_include!("src.style.gresource").expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(move |_| {
        adw::init().unwrap();
        loadCss();
    });

    app.connect_activate(buildUI);
    app.run();
}

fn loadCss() {
    let provider = CssProvider::new();
    provider.load_from_resource("/org/Xetibo/ReSet/style/style.css");

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

#[allow(non_snake_case)]
fn buildUI(app: &Application) {
    let window = Window::new(app);
    window.present();
}

async fn daemon_check() {
    let handle = thread::spawn(|| {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.Xetibo.ReSetDaemon",
            "/org/Xetibo/ReSetDaemon",
            Duration::from_millis(100),
        );
        let res: Result<(), Error> = proxy.method_call("org.Xetibo.ReSetDaemon", "Check", ());
        res
    });
    let res = handle.join();
    if res.unwrap().is_err() {
        println!("Daemon was not running");
        run_daemon().await;
    } else {
        println!("Daemon was running");
    }
}
