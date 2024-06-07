pub mod screenshot;
pub mod dispatcher;
use x11_windows::{Window, get_windows};
use screenshot::screenshot_full;

use std::{
    path::Path,
    process::exit,
    fs,
};

pub struct Config {
    logdir: &str,
}

pub fn get_config() -> Arc<Mutex<Config>> {
    Arc::new(Mutex::new(
        Config {
            logdir: "var/log/recall"
        }
    ))
}

// Screenshots -> Only if not locked
// Deamon
// Bash History
// Browser History
// Browser cookies
// Keylogger

fn init_logdir(dir: &str) {
    if !Path::new(dir).exists() {
        match fs::create_dir_all(dir) {
            Ok(_) => { return; }
            Err(e) => {
                eprintln!("Error creating recall logdir '{}': {}", dir, e);
                exit(-1);
            }
        }
    }
}

fn main() {
    let config = get_config();
    init_logdir(config.logdir);

    let windows: Vec<Window> = match get_windows() {
        Ok(windows) => windows,
        Err(msg) => {
            println!("{msg}");
            Vec::new()
        }
    };

    screenshot_full(Path::new(config.logdir));
}
