extern "C" {
    static mut errno: i32;
    fn error_str(arg1: i32) -> *const u8;
}

#[derive(Copy)]
#[repr(C)]
pub struct strerr {
    pub who: *mut strerr,
    pub x: *const u8,
    pub y: *const u8,
    pub z: *const u8,
}

impl Clone for strerr {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub static mut strerr_sys: strerr = strerr {
    who: 0 as (*mut strerr),
    x: 0 as (*const u8),
    y: 0 as (*const u8),
    z: 0 as (*const u8),
};

#[no_mangle]
pub unsafe extern "C" fn strerr_sysinit() {
    strerr_sys.who = 0i32 as (*mut strerr);
    strerr_sys.x = error_str(errno);
    strerr_sys.y = (*b"\0").as_ptr();
    strerr_sys.z = (*b"\0").as_ptr();
}
