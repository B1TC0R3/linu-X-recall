use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

struct Config {
    logpath: String,
}

fn get_config() -> Arc<Mutex<Config>> {
    Arc::new(Mutex::new(
        Config {
            logpath: "/var/".to_string()
        }
    ))
}

fn interval(delay: Duration, function: fn(Arc<Mutex<Config>>), config) {
    let _ = thread::spawn(move || {
        loop {
            function(config);
            thread::sleep(delay);
        }
    });
}
