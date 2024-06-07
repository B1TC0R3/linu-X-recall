use crate::Config;
use xcap::Monitor;
use std::path::Path;
use anyhow::Result;
use std::sync::{Arc, RwLock};
use chrono::Local;


fn normalized(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}

pub fn screenshot_full(config: Arc<RwLock<Config>>) -> Result<()> {
    let timestamp_formatted = Local::now().format("%Y-%m-%d-%H:%M:%S");
    let current_log_dir = config.read().unwrap().get_current_logdir().clone();
    let log_dir = Path::new(&current_log_dir);

    for monitor in Monitor::all()? {
        let screenshot_name = format!("screenshot-{}-{}.png", timestamp_formatted, normalized(monitor.name()));
        let screenshot_path = log_dir.join(screenshot_name);
        let screenshot = monitor.capture_image()?;
        println!("Monitor Name: {}", monitor.name());
        screenshot.save(screenshot_path)?;
    }

    Ok(())
}
