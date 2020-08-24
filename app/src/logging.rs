use std::sync;
use cfg_if::cfg_if;

static LOGGING: sync::Once = sync::Once::new();

cfg_if! {
    if #[cfg(feature = "logging")] {
        pub fn init_logging() {
            LOGGING.call_once(|| {
                use log::Level;
                console_log::init_with_level(Level::Trace).expect("error initializing log");
            });
        }
    } else {
        pub fn init_logging() {}
    }
}

cfg_if! {
    if #[cfg(feature = "panic")] {
        pub fn init_panic() {
            console_error_panic_hook::set_once();
        }
    } else {
        pub fn init_panic() {}
    }
}
