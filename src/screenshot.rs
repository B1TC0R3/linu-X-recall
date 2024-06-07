use xcap::Monitor;
use std::path::Path;
use anyhow::{Result, Context};
use chrono::Local;


fn normalized(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}

pub fn screenshot_full(log_dir: &Path) -> Result<()> {
    let timestamp_formatted = Local::now().format("%Y-%m-%d-%H:%m:%S");

    for monitor in Monitor::all()? {
        let screenshot_name = format!("screenshot-{}-{}.png", timestamp_formatted, normalized(monitor.name()));
        let screenshot_path = log_dir.join(screenshot_name);
        let screenshot = monitor.capture_image()?;
        println!("Monitor Name: {}", monitor.name());
        screenshot.save(screenshot_path)?;
    }

    Ok(())
}
