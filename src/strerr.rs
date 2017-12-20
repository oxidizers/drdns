//! `strerr.rs`: Error string functionality
//!
//! This should probably be replaced by panic!

use buffer::Buffer;
use buffer_2::BUFFER_2;
use errno::errno;
use libc;

#[derive(Copy)]
#[repr(C)]
pub struct StrErr {
    pub who: *mut StrErr,
    pub x: *const u8,
    pub y: *const u8,
    pub z: *const u8,
}

pub static mut STRERR_SYS: StrErr = StrErr {
    who: 0 as (*mut StrErr),
    x: 0 as (*const u8),
    y: 0 as (*const u8),
    z: 0 as (*const u8),
};

impl Clone for StrErr {
    fn clone(&self) -> Self {
        *self
    }
}

impl StrErr {
    pub unsafe fn sysinit() {
        STRERR_SYS.who = 0i32 as (*mut StrErr);
        STRERR_SYS.x = libc::strerror(errno().0) as *const u8;
        STRERR_SYS.y = (*b"\0").as_ptr();
        STRERR_SYS.z = (*b"\0").as_ptr();
    }

    pub unsafe fn warn(
        x1: *const u8,
        x2: *const u8,
        x3: *const u8,
        x4: *const u8,
        x5: *const u8,
        x6: *const u8,
        mut se: *const StrErr,
    ) {
        StrErr::sysinit();
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
            se = (*se).who as (*const StrErr);
        }
        Buffer::puts(BUFFER_2.as_mut_ptr(), (*b"\n\0").as_ptr());
        Buffer::flush(BUFFER_2.as_mut_ptr());
    }

    pub unsafe fn die(
        e: i32,
        x1: *const u8,
        x2: *const u8,
        x3: *const u8,
        x4: *const u8,
        x5: *const u8,
        x6: *const u8,
        se: *const StrErr,
    ) {
        StrErr::warn(x1, x2, x3, x4, x5, x6, se);
        libc::_exit(e);
    }
}
