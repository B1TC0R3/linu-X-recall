use std::{thread, time};
use anyhow::Result;

fn interval(interval: u64, function: fn() -> Result<()>) {
    thread::spawn(move || {
        let sleep_duration = time::Duration::from_millis(interval);

        loop {
            let _ = function();
            thread::sleep(sleep_duration);
        }
    });
}
