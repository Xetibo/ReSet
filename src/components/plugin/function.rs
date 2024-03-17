extern "C" {
    pub fn startup() -> gtk::Box;
    pub fn sidebar() -> Sidebar;
    pub fn shutdown();
    pub fn run_test();
}

pub struct Sidebar {
    pub name: String,
    pub icon_name: String,
    pub parent: String,
    pub group: String,
}