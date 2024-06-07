extern crate x11;
extern crate x11_dl;

use std::ptr;

use chrono::{
    DateTime,
    Local
};

use anyhow::{
    Result,
    bail
};

use x11::xlib::{
    Display,
    XOpenDisplay,
    XDefaultRootWindow,
    XQueryTree,
    XFetchName,
    XTextProperty,
    XGetWMName
};

pub enum AppType {
    // TODO add more types later on for specializde functionality
    Browser,
    Terminal,
    Other,
    Error,
}

pub struct Window {
    pub title: String,
    pub id: u64,
    pub apptype: AppType,
    pub timestamp: DateTime<Local>,
}

impl Window {
    fn new() -> Window {
        Window {
            title: String::from(""),
            id: 0,
            apptype: AppType::Error,
            timestamp: Local::now(),
        }
    }
}

pub fn get_windows() -> Result<Vec<Window>> {
    let mut windows: Vec<Window> = Vec::new();

    // Open connection to the X server
    let display: *mut Display = unsafe { XOpenDisplay(ptr::null()) };
    if display.is_null() {
        eprintln!("Unable to open X display");
        bail!("Unable to attach to display.");
    }

    // Get the root window
    let root = unsafe { XDefaultRootWindow(display) };

    // Get the list of child windows
    let mut root_return: u64 = 0;
    let mut parent_return: u64 = 0;
    let mut children: *mut u64 = ptr::null_mut();
    let mut nchildren: u32 = 0;

    if unsafe {
        XQueryTree(display, root, &mut root_return, &mut parent_return, &mut children, &mut nchildren)} == 0 {
        eprintln!("Unable to query the window tree");
        return Ok(windows);
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
            // TODO Fallback to XFetchName if XGetWMName fails
            eprintln!("XGetWMName failed - Aborting");
            return Ok(windows);
        }

        if !window_name.is_null() {
            let title = unsafe { std::ffi::CStr::from_ptr(window_name) }.to_string_lossy().into_owned();
            win.title = title;
            win.id = window;
            win.apptype = AppType::Other;
        }

        if !window_name.is_null() {
            unsafe { x11::xlib::XFree(window_name as *mut std::ffi::c_void) };
        }

        if !matches!(win.apptype, AppType::Error) {
            windows.push(win);
        }
    }

    // Free the list of child windows
    if !children.is_null() {
        unsafe { x11::xlib::XFree(children as *mut std::ffi::c_void) };
    }

    // Close the connection to the X server
    unsafe { x11::xlib::XCloseDisplay(display) };
    return Ok(windows);
}
