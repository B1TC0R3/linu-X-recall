pub mod x11_windows;



use x11_windows::{Window, get_windows};




fn main() {
    let windows: Vec<Window> = get_windows();

    for win in windows {
        println!("ID: {} - Title: {}", win.id, win.title);
    }


}
