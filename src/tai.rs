//! `tai.rs`: TAI64 timestamps with 1-second precision
//!
//! This module should probably be replaced with e.g. chrono
//!
//! TAI stands for Temps Atomique International, the current international
//! real time standard. One TAI second is defined as the duration of
//! 9192631770 periods of the radiation corresponding to the transition
//! between the two hyperfine levels of the ground state of the cesium atom.

use libc;

#[derive(Copy)]
#[repr(C)]
pub struct Tai {
    pub x: usize,
}

impl Clone for Tai {
    fn clone(&self) -> Self {
        *self
    }
}

impl Tai {
    pub unsafe fn add(t: *mut Tai, u: *const Tai, v: *const Tai) {
        (*t).x = (*u).x.wrapping_add((*v).x);
    }

    pub unsafe fn now(t: *mut Tai) {
        (*t).x =
            4611686018427387914u64.wrapping_add(libc::time(0i32 as (*mut i64)) as (usize) as (u64)) as
                (usize);
    }

    pub unsafe fn pack(s: *mut u8, t: *const Tai) {
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

    pub unsafe fn sub(t: *mut Tai, u: *const Tai, v: *const Tai) {
        (*t).x = (*u).x.wrapping_sub((*v).x);
    }

    pub unsafe fn uint(t: *mut Tai, u: u32) {
        (*t).x = u as (usize);
    }

    pub unsafe fn unpack(s: *const u8, t: *mut Tai) {
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
