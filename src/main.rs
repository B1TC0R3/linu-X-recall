pub mod x11_windows;

use x11_windows::{Window, get_windows};

fn main() {
    let windows: Vec<Window> = match get_windows() {
        Ok(windows) => windows,
        Err(msg) => {
            println!("{msg}");
            Vec::new()
        }
    };

    for win in windows {
        println!("ID: {} - Title: {}", win.id, win.title);
    }
}
