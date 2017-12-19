use tai::Tai;
use taia::TaiA;

extern "C" {
    fn poll(arg1: *mut pollfd, arg2: u32, arg3: i32) -> i32;
}

#[derive(Copy)]
#[repr(C)]
pub struct pollfd {
    pub fd: i32,
    pub events: i16,
    pub revents: i16,
}

impl Clone for pollfd {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn iopause(
    mut x: *mut pollfd,
    mut len: u32,
    mut deadline: *mut TaiA,
    mut stamp: *mut TaiA,
) {
    let mut t: TaiA;
    let mut millisecs: i32;
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
    poll(x, len, millisecs);
}
