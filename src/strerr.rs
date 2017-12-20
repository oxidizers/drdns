use buffer::Buffer;
use buffer_2::BUFFER_2;
use errno::errno;
use libc;

#[derive(Copy)]
#[repr(C)]
pub struct strerr {
    pub who: *mut strerr,
    pub x: *const u8,
    pub y: *const u8,
    pub z: *const u8,
}

#[no_mangle]
pub static mut strerr_sys: strerr = strerr {
    who: 0 as (*mut strerr),
    x: 0 as (*const u8),
    y: 0 as (*const u8),
    z: 0 as (*const u8),
};

impl Clone for strerr {
    fn clone(&self) -> Self {
        *self
    }
}

impl strerr {
    #[no_mangle]
    pub unsafe extern "C" fn strerr_sysinit() {
        strerr_sys.who = 0i32 as (*mut strerr);
        strerr_sys.x = (*const u8)libc::strerror(errno().0);
        strerr_sys.y = (*b"\0").as_ptr();
        strerr_sys.z = (*b"\0").as_ptr();
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
            Buffer::puts(BUFFER_2.as_mut_ptr(), x1);
        }
        if !x2.is_null() {
            Buffer::puts(BUFFER_2.as_mut_ptr(), x2);
        }
        if !x3.is_null() {
            Buffer::puts(BUFFER_2.as_mut_ptr(), x3);
        }
        if !x4.is_null() {
            Buffer::puts(BUFFER_2.as_mut_ptr(), x4);
        }
        if !x5.is_null() {
            Buffer::puts(BUFFER_2.as_mut_ptr(), x5);
        }
        if !x6.is_null() {
            Buffer::puts(BUFFER_2.as_mut_ptr(), x6);
        }
        'loop12: loop {
            if se.is_null() {
                break;
            }
            if !(*se).x.is_null() {
                Buffer::puts(BUFFER_2.as_mut_ptr(), (*se).x);
            }
            if !(*se).y.is_null() {
                Buffer::puts(BUFFER_2.as_mut_ptr(), (*se).y);
            }
            if !(*se).z.is_null() {
                Buffer::puts(BUFFER_2.as_mut_ptr(), (*se).z);
            }
            se = (*se).who as (*const strerr);
        }
        Buffer::puts(BUFFER_2.as_mut_ptr(), (*b"\n\0").as_ptr());
        Buffer::flush(BUFFER_2.as_mut_ptr());
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
        libc::_exit(e);
    }
}
