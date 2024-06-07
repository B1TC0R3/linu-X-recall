pub mod screenshot;
pub mod dispatcher;
use x11_windows::{Window, get_windows};
use screenshot::screenshot_full;

use std::{
    path::Path,
    process::exit,
    fs,
};

const LOG_DIR: &str = "var/log/recall";

/*
struct Context {
    logging_folder: Path,
    day_folder: Path,
    hour_folder: Path,
}*/

// Screenshots -> Only if not locked
// Deamon
// Bash History
// Browser History
// Browser cookies
// Keylogger



fn init_logdir() { 
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
    init_logdir();

    let windows: Vec<Window> = match get_windows() {
        Ok(windows) => windows,
        Err(msg) => {
            println!("{msg}");
            Vec::new()
        }
    };

    screenshot_full(Path::new(LOG_DIR));
}
