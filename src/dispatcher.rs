use crate::Config;
use std::thread;
use std::time::Duration;
use std::sync::Arc;

fn interval(delay: Duration, function: fn(Arc<Config>), config: Arc<Config>) {
    let _ = thread::spawn(move || {
        loop {
            function(config.clone());
            thread::sleep(delay);
        }
    });
}
