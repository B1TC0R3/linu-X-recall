pub mod screenshot;
pub mod dispatcher;
use x11_windows::{Window, get_windows};
use screenshot::screenshot_full;

use std::{
    path::Path,
    process::exit,
    fs,
};

struct Config {
    logdir: &str,
}

fn get_config() -> Arc<Mutex<Config>> {
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



fn init_logdir(dir) {
    if !Path::new(LOG_DIR).exists() {
        match fs::create_dir_all(LOG_DIR) {
            Ok(_) => { return; }
            Err(e) => {
                eprintln!("Error creating recall logdir '{}': {}", LOG_DIR, e);
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
