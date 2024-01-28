use macroquest::Plugin;

#[macroquest::plugin]
struct MQRustSkeleton {}

impl macroquest::Plugin for MQRustSkeleton {
    fn new() -> Self {
        MQRustSkeleton {}
    }

    fn initialize(&mut self) {
        use std::ffi::CString;
        let s = CString::new("it worked").unwrap();
        unsafe {
            ffi::DebugSpewAlways(s.as_ptr());
        }
    }
}

mod ffi {
    use std::os::raw::c_char;

    #[link(name = "MQ2Main")]
    extern "C" {
        pub fn DebugSpewAlways(szFormat: *const c_char, ...);
    }
}
