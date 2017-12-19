//! `tai.rs`: Timestamp-related functionality
//!
//! This module should probably be replaced with e.g. chrono
//!
//! TAI stands for Temps Atomique International, the current international
//! real time standard. One TAI second is defined as the duration of
//! 9192631770 periods of the radiation corresponding to the transition
//! between the two hyperfine levels of the ground state of the cesium atom.

extern "C" {
    fn time(arg1: *mut isize) -> isize;
}

#[derive(Copy)]
#[repr(C)]
pub struct tai {
    pub x: usize,
}

impl Clone for tai {
    fn clone(&self) -> Self {
        *self
    }
}

impl tai {
    #[no_mangle]
    pub unsafe extern "C" fn tai_add(mut t: *mut tai, mut u: *const tai, mut v: *const tai) {
        (*t).x = (*u).x.wrapping_add((*v).x);
    }

    #[no_mangle]
    pub unsafe extern "C" fn tai_now(mut t: *mut tai) {
        (*t).x =
            4611686018427387914u64.wrapping_add(time(0i32 as (*mut isize)) as (usize) as (u64)) as
                (usize);
    }

    #[no_mangle]
    pub unsafe extern "C" fn tai_pack(mut s: *mut u8, mut t: *const tai) {
        let mut x: usize;
        x = (*t).x;
        *s.offset(7isize) = (x & 255usize) as (u8);
        x = x >> 8i32;
        *s.offset(6isize) = (x & 255usize) as (u8);
        x = x >> 8i32;
        *s.offset(5isize) = (x & 255usize) as (u8);
        x = x >> 8i32;
        *s.offset(4isize) = (x & 255usize) as (u8);
        x = x >> 8i32;
        *s.offset(3isize) = (x & 255usize) as (u8);
        x = x >> 8i32;
        *s.offset(2isize) = (x & 255usize) as (u8);
        x = x >> 8i32;
        *s.offset(1isize) = (x & 255usize) as (u8);
        x = x >> 8i32;
        *s.offset(0isize) = x as (u8);
    }

    #[no_mangle]
    pub unsafe extern "C" fn tai_sub(mut t: *mut tai, mut u: *const tai, mut v: *const tai) {
        (*t).x = (*u).x.wrapping_sub((*v).x);
    }

    #[no_mangle]
    pub unsafe extern "C" fn tai_uint(mut t: *mut tai, mut u: u32) {
        (*t).x = u as (usize);
    }

    #[no_mangle]
    pub unsafe extern "C" fn tai_unpack(mut s: *const u8, mut t: *mut tai) {
        let mut x: usize;
        x = *s.offset(0isize) as (usize);
        x = x << 8i32;
        x = x.wrapping_add(*s.offset(1isize) as (usize));
        x = x << 8i32;
        x = x.wrapping_add(*s.offset(2isize) as (usize));
        x = x << 8i32;
        x = x.wrapping_add(*s.offset(3isize) as (usize));
        x = x << 8i32;
        x = x.wrapping_add(*s.offset(4isize) as (usize));
        x = x << 8i32;
        x = x.wrapping_add(*s.offset(5isize) as (usize));
        x = x << 8i32;
        x = x.wrapping_add(*s.offset(6isize) as (usize));
        x = x << 8i32;
        x = x.wrapping_add(*s.offset(7isize) as (usize));
        (*t).x = x;
    }
}
