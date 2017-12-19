//! `taia.rs`: TAI64NA timestamps with attosecond precision
//!
//! This module should probably be replaced with e.g. chrono
//!
//! TAI stands for Temps Atomique International, the current international
//! real time standard. One TAI second is defined as the duration of
//! 9192631770 periods of the radiation corresponding to the transition
//! between the two hyperfine levels of the ground state of the cesium atom.

use tai::Tai;

extern "C" {
    fn gettimeofday(arg1: *mut timeval, arg2: *mut ::std::os::raw::c_void) -> i32;
}

#[derive(Copy)]
#[repr(C)]
pub struct TaiA {
    pub sec: Tai,
    pub nano: usize,
    pub atto: usize,
}

impl Clone for TaiA {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: isize,
    pub tv_usec: i32,
}

impl Clone for timeval {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}

impl Clone for timezone {
    fn clone(&self) -> Self {
        *self
    }
}

impl TaiA {
    pub unsafe fn add(t: *mut TaiA, u: *const TaiA, v: *const TaiA) {
        (*t).sec.x = (*u).sec.x.wrapping_add((*v).sec.x);
        (*t).nano = (*u).nano.wrapping_add((*v).nano);
        (*t).atto = (*u).atto.wrapping_add((*v).atto);
        if (*t).atto > 999999999usize {
            (*t).atto = (*t).atto.wrapping_sub(1000000000usize);
            (*t).nano = (*t).nano.wrapping_add(1usize);
        }
        if (*t).nano > 999999999usize {
            (*t).nano = (*t).nano.wrapping_sub(1000000000usize);
            (*t).sec.x = (*t).sec.x.wrapping_add(1usize);
        }
    }

    pub unsafe fn approx(t: *const TaiA) -> f64 {
        (*(&(*t).sec as (*const Tai))).x as (f64) + TaiA::frac(t)
    }

    pub unsafe fn frac(t: *const TaiA) -> f64 {
        ((*t).atto as (f64) * 0.000000001f64 + (*t).nano as (f64)) * 0.000000001f64
    }

    pub unsafe fn less(t: *const TaiA, u: *const TaiA) -> i32 {
        if (*t).sec.x < (*u).sec.x {
            1i32
        } else if (*t).sec.x > (*u).sec.x {
            0i32
        } else if (*t).nano < (*u).nano {
            1i32
        } else if (*t).nano > (*u).nano {
            0i32
        } else {
            ((*t).atto < (*u).atto) as (i32)
        }
    }

    pub unsafe fn now(t: *mut TaiA) {
        let mut now: timeval = timeval {
            tv_sec: 0,
            tv_usec: 0
        };

        gettimeofday(
            &mut now as (*mut timeval),
            0i32 as (*mut timezone) as (*mut ::std::os::raw::c_void),
        );

        (*(&mut (*t).sec as (*mut Tai))).x =
            4611686018427387914u64.wrapping_add(now.tv_sec as (usize) as (u64)) as (usize);
        (*t).nano = (1000i32 * now.tv_usec + 500i32) as (usize);
        (*t).atto = 0usize;
    }

    pub unsafe fn pack(mut s: *mut u8, t: *const TaiA) {
        let mut x: usize;
        Tai::pack(s, &(*t).sec as (*const Tai));
        s = s.offset(8isize);
        x = (*t).atto;
        *s.offset(7isize) = (x & 255usize) as (u8);
        x = x >> 8i32;
        *s.offset(6isize) = (x & 255usize) as (u8);
        x = x >> 8i32;
        *s.offset(5isize) = (x & 255usize) as (u8);
        x = x >> 8i32;
        *s.offset(4isize) = x as (u8);
        x = (*t).nano;
        *s.offset(3isize) = (x & 255usize) as (u8);
        x = x >> 8i32;
        *s.offset(2isize) = (x & 255usize) as (u8);
        x = x >> 8i32;
        *s.offset(1isize) = (x & 255usize) as (u8);
        x = x >> 8i32;
        *s.offset(0isize) = x as (u8);
    }

    pub unsafe fn sub(t: *mut TaiA, u: *const TaiA, v: *const TaiA) {
        let unano: usize = (*u).nano;
        let uatto: usize = (*u).atto;
        (*t).sec.x = (*u).sec.x.wrapping_sub((*v).sec.x);
        (*t).nano = unano.wrapping_sub((*v).nano);
        (*t).atto = uatto.wrapping_sub((*v).atto);
        if (*t).atto > uatto {
            (*t).atto = (*t).atto.wrapping_add(1000000000usize);
            (*t).nano = (*t).nano.wrapping_sub(1usize);
        }
        if (*t).nano > unano {
            (*t).nano = (*t).nano.wrapping_add(1000000000usize);
            (*t).sec.x = (*t).sec.x.wrapping_sub(1usize);
        }
    }

    pub unsafe fn tai(ta: *const TaiA, t: *mut Tai) {
        *t = (*ta).sec;
    }

    pub unsafe fn uint(t: *mut TaiA, s: u32) {
        (*t).sec.x = s as (usize);
        (*t).nano = 0usize;
        (*t).atto = 0usize;
    }
}
