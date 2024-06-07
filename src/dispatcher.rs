use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};



fn interval(delay: Duration, function: fn(Arc<Mutex<Config>>), config) {
    let _ = thread::spawn(move || {
        loop {
            function(config);
            thread::sleep(delay);
        }
    });
}
