use std::ffi::CStr;

pub mod app;
pub mod window;

pub use app::App;
pub use window::Window;

#[link(name = "swiftbridge", kind = "framework")]
unsafe extern "C" {
    fn swiftbridge_last_error() -> *const i8;
    fn swiftbridge_clear_last_error();
}

pub(crate) fn last_error() -> String {
    unsafe {
        let p = swiftbridge_last_error();
        if p.is_null() {
            return "unknown error".to_string();
        }
        let s = CStr::from_ptr(p).to_string_lossy().into_owned();
        swiftbridge_clear_last_error();
        s
    }
}
