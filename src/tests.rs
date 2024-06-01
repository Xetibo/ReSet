use std::thread;
use std::time::Duration;
use re_set_lib::utils::plugin_setup::FRONTEND_PLUGINS;
use crate::daemon_check;

#[tokio::test]
#[cfg(test)]
async fn test_plugins() {
    use re_set_lib::utils::plugin::plugin_tests;
    tokio::task::spawn(daemon_check());
    thread::sleep(Duration::from_millis(2000));
    unsafe {
        for plugin in FRONTEND_PLUGINS.iter() {
            let name = (plugin.frontend_name)();
            let tests = (plugin.frontend_tests)();
            plugin_tests(name, tests);
        }
    }
}
