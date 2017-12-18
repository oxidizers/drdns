extern "C" {
    fn _exit(arg1: i32);
    static mut buffer_2: *mut buffer;
    fn buffer_flush(arg1: *mut buffer) -> i32;
    fn buffer_puts(arg1: *mut buffer, arg2: *const u8) -> i32;
    fn strerr_sysinit();
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

#[derive(Copy)]
#[repr(C)]
pub struct buffer {
    pub x: *mut u8,
    pub p: u32,
    pub n: u32,
    pub fd: i32,
    pub op: unsafe extern "C" fn() -> i32,
}

impl Clone for buffer {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn strerr_warn(
    mut x1: *const u8,
    mut x2: *const u8,
    mut x3: *const u8,
    mut x4: *const u8,
    mut x5: *const u8,
    mut x6: *const u8,
    mut se: *const strerr,
) {
    strerr_sysinit();
    if !x1.is_null() {
        buffer_puts(buffer_2, x1);
    }
    if !x2.is_null() {
        buffer_puts(buffer_2, x2);
    }
    if !x3.is_null() {
        buffer_puts(buffer_2, x3);
    }
    if !x4.is_null() {
        buffer_puts(buffer_2, x4);
    }
    if !x5.is_null() {
        buffer_puts(buffer_2, x5);
    }
    if !x6.is_null() {
        buffer_puts(buffer_2, x6);
    }
    'loop12: loop {
        if se.is_null() {
            break;
        }
        if !(*se).x.is_null() {
            buffer_puts(buffer_2, (*se).x);
        }
        if !(*se).y.is_null() {
            buffer_puts(buffer_2, (*se).y);
        }
        if !(*se).z.is_null() {
            buffer_puts(buffer_2, (*se).z);
        }
        se = (*se).who as (*const strerr);
    }
    buffer_puts(buffer_2, (*b"\n\0").as_ptr());
    buffer_flush(buffer_2);
}

#[no_mangle]
pub unsafe extern "C" fn strerr_die(
    mut e: i32,
    mut x1: *const u8,
    mut x2: *const u8,
    mut x3: *const u8,
    mut x4: *const u8,
    mut x5: *const u8,
    mut x6: *const u8,
    mut se: *const strerr,
) {
    strerr_warn(x1, x2, x3, x4, x5, x6, se);
    _exit(e);
}
