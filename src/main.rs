pub mod screenshot;
pub mod dispatcher;
pub mod x11_windows;
pub mod keylogger;

use x11_windows::{get_windows, Window};
use screenshot::screenshot_full;
use std::sync::Arc;


use std::{
    path::Path,
    process::exit,
    fs,
};

pub struct Config {
    logdir: String,
}

pub fn get_config() -> Arc<Config> {
    Arc::new(
        Config {
            logdir: "var/log/recall".to_string()
        }
    )
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
    init_logdir(&config.logdir);

    let windows: Vec<Window> = match get_windows() {
        Ok(windows) => windows,
        Err(msg) => {
            println!("{msg}");
            Vec::new()
        }
    };

    screenshot_full(Path::new(&config.clone().logdir)).unwrap();

    keylogger::run(Path::new(LOG_DIR));
}
