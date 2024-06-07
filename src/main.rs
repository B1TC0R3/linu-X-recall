pub mod screenshot;
pub mod dispatcher;
pub mod x11_windows;
pub mod keylogger;

use x11_windows::{get_windows, Window};
use dispatcher::{interval, single};
use screenshot::screenshot_full;
use chrono::Local;
use std::time::Duration;
use anyhow::Result;
use std::thread;
use std::sync::{Arc, RwLock};
use std::{
    path::Path,
    process::exit,
    fs,
};

pub struct Config {
    logdir: String,
    curdir: String,
}

impl Config {
    fn get_current_logdir(&self) -> String {
        Path::new(
            &self.logdir
        ).join(
            &self.curdir
        ).display().to_string()
    }
}

pub fn get_config() -> Arc<RwLock<Config>> {
    Arc::new(RwLock::new(
        Config {
            logdir: "var/log/recall".to_string(),
            curdir: Local::now().format("%Y/%B/%d/%H/%M").to_string(),
        }
    ))
}

// Screenshots -> Only if not locked
// Deamon
// Bash History
// Browser History
// Browser cookies
// Keylogger

fn update_dirs(config: Arc<RwLock<Config>>) -> Result<()> {
    loop {
        let curdir = Local::now().format("%Y/%B/%d/%H/%M").to_string();

        if config.read().unwrap().curdir == curdir {
            thread::sleep(Duration::from_secs(30));
            continue;
        }

        let path = Path::new(&config.read().unwrap().logdir).join(&config.read().unwrap().curdir);
        if !Path::new(&path).exists() {
           match fs::create_dir_all(&path) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error creating current dir '{}': {}", curdir, e);
                    continue;
                }
            }
        }

        config.write().unwrap().curdir = curdir;
        println!("Updated log path to: {}", config.read().unwrap().get_current_logdir());
        thread::sleep(Duration::from_secs(30));
    }
}

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
    init_logdir(&config.read().unwrap().get_current_logdir());

    let _windows: Vec<Window> = match get_windows() {
        Ok(windows) => windows,
        Err(msg) => {
            println!("{msg}");
            Vec::new()
        }
    };

    let dirupdate_handle = single(update_dirs, config.clone());
    let screenshot_threat = interval(10000, screenshot_full, config.clone());
    let keylogger_threat = single(keylogger::run, config.clone());

    dirupdate_handle.join().unwrap();
    screenshot_threat.join().unwrap();
    keylogger_threat.join().unwrap();
}
