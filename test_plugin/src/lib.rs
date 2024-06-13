use std::sync::Arc;
use std::time::Duration;

use dbus::blocking::Connection;
use dbus::Error;
use gtk::{gio, Orientation};
use gtk::prelude::{BoxExt, ButtonExt};
use re_set_lib::utils::plugin::{PluginCapabilities, PluginImplementation, PluginTestFunc, SidebarInfo};

pub const BASE: &str = "org.Xetibo.ReSet.Daemon";
pub const DBUS_PATH: &str = "/org/Xetibo/ReSet/Plugins/test";
pub const INTERFACE: &str = "org.Xetibo.ReSet.TestPlugin";

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn capabilities() -> PluginCapabilities {
    println!("frontend capabilities called");
    PluginCapabilities::new(vec!["test"], PluginImplementation::Frontend)
}

#[no_mangle]
pub extern "C" fn frontend_startup() {
    println!("frontend startup called");
}

#[no_mangle]
pub extern "C" fn frontend_shutdown() {
    println!("frontend shutdown called");
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn frontend_data() -> (SidebarInfo, Vec<gtk::Box>) {
    println!("frontend data called");
    let info = SidebarInfo {
        name: "test",
        icon_name: "microphone-disabled-symbolic",
        parent: None,
    };
    let box1 = gtk::Box::builder().orientation(Orientation::Vertical).build();
    let box2 = gtk::Box::builder().orientation(Orientation::Horizontal).build();

    let label = Arc::new(LabelWrapper {
        label: gtk::Label::builder().label("Hello, World!").build(),
    });

    let label2 = gtk::Label::builder().label("Bye, World!").build();
    let button = gtk::Button::builder().label("Click me!").build();
    box1.append(&label.label);
    box2.append(&label2);
    box2.append(&button);

    button.connect_clicked(move |_| {
        let label = Arc::clone(&label);
        gio::spawn_blocking(move || {
            let conn = Connection::new_session().unwrap();
            let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
            let res: Result<(String, u32), Error> = proxy.method_call(INTERFACE, "Test", ());
            let (text, age) = res.unwrap();
            label.label.set_text(&format!("Name: {}, Age: {}", text, age));
        });
    });

    let boxes = vec![
        box1, box2,
    ];

    (info, boxes)
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn frontend_tests() -> Vec<PluginTestFunc> {
    println!("frontend tests called");
    vec![]
}

pub struct LabelWrapper {
    label: gtk::Label,
}

unsafe impl Send for LabelWrapper {}

unsafe impl Sync for LabelWrapper {}
