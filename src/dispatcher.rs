use crate::Config;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::sync::{Arc, RwLock};
use anyhow::Result;

pub fn interval(delay: u64, function: fn(Arc<RwLock<Config>>) -> Result<()>, config: Arc<RwLock<Config>>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let _ = function(config.clone());
            thread::sleep(Duration::from_millis(delay));
        }
    })
}

pub fn single(function: fn(Arc<RwLock<Config>>) -> Result<()>, config: Arc<RwLock<Config>>) -> JoinHandle<()> {
    thread::spawn(move || {
        function(config.clone()).unwrap();
    })
}
