use std::ffi::CStr;
use std::os::raw::c_char;

#[link(name = "MQ2Main")]
extern "C" {
    static gszVersion: [c_char; 32];
    static gszTime: [c_char; 32];
}

pub(crate) fn eq_version() -> &'static str {
    let v = unsafe { CStr::from_ptr(gszVersion.as_ptr()) };

    v.to_str().unwrap()
}

pub(crate) fn eq_time() -> &'static str {
    let v = unsafe { CStr::from_ptr(gszTime.as_ptr()) };

    v.to_str().unwrap()
}
