use std::os::raw::c_void;

use crate::last_error;
use crate::window::Window;

#[link(name = "swiftbridge", kind = "framework")]
unsafe extern "C" {
    fn swift_appkit_create_app() -> *mut c_void;
    fn swift_appkit_run(appPtr: *const c_void);
}

/// A handle to the shared `NSApplication` created by Swift.
pub struct App {
    ptr: *mut c_void,
}

// Mark as Send + Sync manually for the app handle.
unsafe impl Send for App {}
unsafe impl Sync for App {}

impl Clone for App {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr }
    }
}

impl App {
    /// Create or get the shared application instance.
    pub fn create() -> Result<Self, String> {
        unsafe {
            let p = swift_appkit_create_app();
            if p.is_null() {
                return Err(last_error());
            }
            Ok(Self { ptr: p })
        }
    }

    /// Run the AppKit run loop (blocks until the app exits).
    pub fn run(&self) {
        unsafe { swift_appkit_run(self.ptr) }
    }
    
    /// Create a window via the Swift bridge.
    pub fn create_window(
        &self,
        origin_x: f64,
        origin_y: f64,
        width: f64,
        height: f64,
        title: &str,
    ) -> Result<Window, String> {
        let _ = self.ptr;
        Window::create(origin_x, origin_y, width, height, title)
    }
}