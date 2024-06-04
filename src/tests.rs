#[tokio::test]
#[cfg(test)]
async fn test_plugins() {
    use crate::daemon_check;
    use re_set_lib::utils::plugin::plugin_tests;
    use re_set_lib::utils::plugin_setup::FRONTEND_PLUGINS;
    use std::hint;
    use std::sync::atomic::AtomicBool;
    use std::sync::Arc;
    let ready = Arc::new(AtomicBool::new(false));
    let rc = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    rc.spawn(daemon_check(ready.clone()));
    while !ready.load(std::sync::atomic::Ordering::SeqCst) {
        hint::spin_loop();
    }
    unsafe {
        println!("pang");
        for plugin in FRONTEND_PLUGINS.iter() {
            let name = (plugin.frontend_name)();
            let tests = (plugin.frontend_tests)();
            plugin_tests(name, tests);
        }
    }
    rc.shutdown_background();
}
