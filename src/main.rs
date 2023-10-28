mod window;
mod audio;
mod wifi;

use gtk::prelude::*;
use gtk::{Application, gio};
use window::Window;

const APP_ID: &str = "org.Xetibo.ReSet";

fn main() {
    gio::resources_register_include!("src.templates.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(buildUI);
    app.run();
}

#[allow(non_snake_case)]
fn buildUI(app: &Application) {
    let window = Window::new(app);
    window.present();
}