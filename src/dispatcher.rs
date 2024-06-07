use crate::Config;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::sync::Arc;

pub fn interval(delay: u64, function: fn(Arc<Config>), config: Arc<Config>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            function(config.clone());
            thread::sleep(Duration::from_millis(delay));
        }
    })
}

pub fn single(function: fn(Arc<Config>), config: Arc<Config>) -> JoinHandle<()> {
    thread::spawn(move || {
        function(config.clone());
    })
}
