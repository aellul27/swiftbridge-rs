use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_void};
use std::sync::atomic::{AtomicPtr, Ordering};

use crate::last_error;

#[link(name = "swiftbridge", kind = "framework")]
unsafe extern "C" {
    fn swift_appkit_create_window(
        originX: c_double,
        originY: c_double,
        width: c_double,
        height: c_double,
        titlePtr: *const c_char,
    ) -> *const c_void;

    fn swift_appkit_window_close(windowPtr: *const c_void);
    fn swift_appkit_set_title(windowPtr: *const c_void, titlePtr: *const c_char);
    fn swift_appkit_set_location(windowPtr: *const c_void, x: c_double, y: c_double);
    fn swift_appkit_set_size(windowPtr: *const c_void, width: c_double, height: c_double);
}

/// Opaque handle to an `NSWindow` kept alive by Swift until released.
pub struct Window {
    ptr: AtomicPtr<c_void>,
}

impl Window {
    /// Construct from a raw pointer returned by the Swift bridge.
    pub(crate) fn from_raw(ptr: *const c_void) -> Self {
        Self {
            ptr: AtomicPtr::new(ptr as *mut c_void),
        }
    }

    /// Create a new window via the Swift bridge.
    pub fn create(
        origin_x: f64,
        origin_y: f64,
        width: f64,
        height: f64,
        title: &str,
    ) -> Result<Self, String> {
        let c_title = CString::new(title).map_err(|_| "title contained interior nul".to_string())?;
        unsafe {
            let w = swift_appkit_create_window(
                origin_x as c_double,
                origin_y as c_double,
                width as c_double,
                height as c_double,
                c_title.as_ptr(),
            );
            if w.is_null() {
                return Err(last_error());
            }
            Ok(Window::from_raw(w))
        }
    }

    fn as_ptr(&self) -> *mut c_void {
        self.ptr.load(Ordering::Acquire)
    }

    /// Explicitly releases the Swift window. Safe to call multiple times.
    pub fn destroy(&self) {
        let window_ptr = self.ptr.swap(std::ptr::null_mut(), Ordering::AcqRel);
        if !window_ptr.is_null() {
            unsafe {
                swift_appkit_window_close(window_ptr);
            }
        }
    }

    /// Sets the window title safely.
    pub fn set_title(&self, title: &str) {
        let c_title = match CString::new(title) {
            Ok(t) => t,
            Err(_) => return,
        };
        let window_ptr = self.as_ptr();
        if window_ptr.is_null() {
            return;
        }
        unsafe {
            swift_appkit_set_title(window_ptr, c_title.as_ptr());
        }
    }

    /// Sets the window location safely.
    pub fn set_location(&self, x: f64, y: f64) {
        let window_ptr = self.as_ptr();
        if window_ptr.is_null() {
            return;
        }
        unsafe {
            swift_appkit_set_location(window_ptr, x as c_double, y as c_double);
        }
    }

    /// Sets the window size safely.
    pub fn set_size(&self, width: f64, height: f64) {
        let window_ptr = self.as_ptr();
        if window_ptr.is_null() {
            return;
        }
        unsafe {
            swift_appkit_set_size(window_ptr, width as c_double, height as c_double);
        }
    }

    /// Access the raw pointer if needed for lower-level interop.
    pub fn raw(&self) -> *const c_void {
        self.as_ptr() as *const c_void
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.destroy();
    }
}