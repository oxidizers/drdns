//! `iopause.rs`: Pause while waiting for I/O

use libc;
use taia::TaiA;

pub unsafe fn iopause(
    x: *mut libc::pollfd,
    len: u32,
    deadline: *mut TaiA,
    stamp: *mut TaiA,
) {
    let mut t: TaiA;
    let millisecs: i32;
    let mut d: f64;
    let mut i: i32;
    if TaiA::less(deadline as (*const TaiA), stamp as (*const TaiA)) != 0 {
        millisecs = 0i32;
    } else {
        t = *stamp;
        TaiA::sub(
            &mut t as (*mut TaiA),
            deadline as (*const TaiA),
            &mut t as (*mut TaiA) as (*const TaiA),
        );
        d = TaiA::approx(&mut t as (*mut TaiA) as (*const TaiA));
        if d > 1000.0f64 {
            d = 1000.0f64;
        }
        millisecs = (d * 1000.0f64 + 20.0f64) as (i32);
    }
    i = 0i32;
    'loop6: loop {
        if !(i as (u32) < len) {
            break;
        }
        (*x.offset(i as (isize))).revents = 0i16;
        i = i + 1;
    }
    libc::poll(x, len as libc::nfds_t, millisecs);
}
