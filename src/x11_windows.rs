extern crate x11;
extern crate x11_dl;

use x11::xlib::{Display, XOpenDisplay, XDefaultRootWindow, XQueryTree, XFetchName, XTextProperty, XGetWMName};
use std::ptr;

pub enum AppType {
    Browser,
    Terminal,
    Other,
    Error, 
}


pub struct Window {
    pub title: String,
    pub id: u64,
    pub apptype: AppType,
}

impl Window {
    fn new() -> Window {
        Window { title: String::from(""), id: 0, apptype: AppType::Error }
    }
}



pub fn get_windows() -> Vec<Window> {
    let mut windows: Vec<Window> = Vec::new();

    // Open connection to the X server
    let display: *mut Display = unsafe { XOpenDisplay(ptr::null()) };
    if display.is_null() {
        eprintln!("Unable to open X display");
        return windows;
    }

    // Get the root window
    let root = unsafe { XDefaultRootWindow(display) };

    // Get the list of child windows
    let mut root_return: u64 = 0;
    let mut parent_return: u64 = 0;
    let mut children: *mut u64 = ptr::null_mut();
    let mut nchildren: u32 = 0;
    if unsafe { XQueryTree(display, root, &mut root_return, &mut parent_return, &mut children, &mut nchildren) } == 0 {
        eprintln!("Unable to query the window tree");
        return windows;
    }

    // Iterate over the child windows and get their titles
    for i in 0..nchildren {
        let mut win: Window = Window::new();

        let window = unsafe { *children.offset(i as isize) };
        let mut window_name: *mut i8 = ptr::null_mut();
        let mut text_prop: XTextProperty = XTextProperty {
            value: ptr::null_mut(),
            encoding: 0,
            format: 0,
            nitems: 0,
        };

        if unsafe { XGetWMName(display, window, &mut text_prop) } != 0 && !text_prop.value.is_null() {
            window_name = text_prop.value as *mut i8;
        } else if unsafe { XFetchName(display, window, &mut window_name) } != 0 && !window_name.is_null() {
            // Fallback to XFetchName if XGetWMName fails
        }

        if !window_name.is_null() {
            let title = unsafe { std::ffi::CStr::from_ptr(window_name) }.to_string_lossy().into_owned();
            println!("Window ID: {} - Title: {}", window, title);
            win.title = title;
            win.id = window;
        }

        if !window_name.is_null() {
            unsafe { x11::xlib::XFree(window_name as *mut std::ffi::c_void) };
        }

        windows.push(win);
    }

    // Free the list of child windows
    if !children.is_null() {
        unsafe { x11::xlib::XFree(children as *mut std::ffi::c_void) };
    }

    // Close the connection to the X server
    unsafe { x11::xlib::XCloseDisplay(display) };
    return windows;
}
