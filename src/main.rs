#![allow(non_snake_case)]

use gtk::{Application, CssProvider, gio};
use gtk::gdk::Display;
use gtk::prelude::*;

use crate::components::window::window::Window;

mod components;

const APP_ID: &str = "org.Xetibo.ReSet";

fn main() {
    gio::resources_register_include!("src.templates.gresource")
        .expect("Failed to register resources.");
    gio::resources_register_include!("src.icons.gresource")
        .expect("Failed to register resources.");
    gio::resources_register_include!("src.style.gresource")
        .expect("Failed to register resources.");

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

