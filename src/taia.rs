//! `taia.rs`: TAI64NA timestamps with attosecond precision
//!
//! This module should probably be replaced with e.g. chrono
//!
//! TAI stands for Temps Atomique International, the current international
//! real time standard. One TAI second is defined as the duration of
//! 9192631770 periods of the radiation corresponding to the transition
//! between the two hyperfine levels of the ground state of the cesium atom.

use tai::tai;

extern "C" {
    fn gettimeofday(arg1: *mut timeval, arg2: *mut ::std::os::raw::c_void) -> i32;
    fn tai_pack(arg1: *mut u8, arg2: *const tai);
}

#[derive(Copy)]
#[repr(C)]
pub struct taia {
    pub sec: tai,
    pub nano: usize,
    pub atto: usize,
}

impl Clone for taia {
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

impl taia {
    #[no_mangle]
    pub unsafe extern "C" fn taia_add(mut t: *mut taia, mut u: *const taia, mut v: *const taia) {
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

    #[no_mangle]
    pub unsafe extern "C" fn taia_approx(mut t: *const taia) -> f64 {
        (*(&(*t).sec as (*const tai))).x as (f64) + taia_frac(t)
    }

    #[no_mangle]
    pub unsafe extern "C" fn taia_frac(mut t: *const taia) -> f64 {
        ((*t).atto as (f64) * 0.000000001f64 + (*t).nano as (f64)) * 0.000000001f64
    }

    #[no_mangle]
    pub unsafe extern "C" fn taia_less(mut t: *const taia, mut u: *const taia) -> i32 {
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

    #[no_mangle]
    pub unsafe extern "C" fn taia_now(mut t: *mut taia) {
        let mut now: timeval;
        gettimeofday(
            &mut now as (*mut timeval),
            0i32 as (*mut timezone) as (*mut ::std::os::raw::c_void),
        );
        (*(&mut (*t).sec as (*mut tai))).x =
            4611686018427387914u64.wrapping_add(now.tv_sec as (usize) as (u64)) as (usize);
        (*t).nano = (1000i32 * now.tv_usec + 500i32) as (usize);
        (*t).atto = 0usize;
    }

    #[no_mangle]
    pub unsafe extern "C" fn taia_pack(mut s: *mut u8, mut t: *const taia) {
        let mut x: usize;
        tai_pack(s, &(*t).sec as (*const tai));
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

    #[no_mangle]
    pub unsafe extern "C" fn taia_sub(mut t: *mut taia, mut u: *const taia, mut v: *const taia) {
        let mut unano: usize = (*u).nano;
        let mut uatto: usize = (*u).atto;
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

    #[no_mangle]
    pub unsafe extern "C" fn taia_tai(mut ta: *const taia, mut t: *mut tai) {
        *t = (*ta).sec;
    }

    #[no_mangle]
    pub unsafe extern "C" fn taia_uint(mut t: *mut taia, mut s: u32) {
        (*t).sec.x = s as (usize);
        (*t).nano = 0usize;
        (*t).atto = 0usize;
    }
}
