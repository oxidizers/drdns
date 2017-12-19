extern "C" {
    fn poll(arg1: *mut pollfd, arg2: u32, arg3: i32) -> i32;
    fn taia_approx(arg1: *const TaiA) -> f64;
    fn taia_less(arg1: *const TaiA, arg2: *const TaiA) -> i32;
    fn taia_sub(arg1: *mut TaiA, arg2: *const TaiA, arg3: *const TaiA);
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

#[derive(Copy)]
#[repr(C)]
pub struct taia {
    pub sec: Tai,
    pub nano: usize,
    pub atto: usize,
}

impl Clone for taia {
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
    if taia_less(deadline as (*const TaiA), stamp as (*const TaiA)) != 0 {
        millisecs = 0i32;
    } else {
        t = *stamp;
        taia_sub(
            &mut t as (*mut TaiA),
            deadline as (*const TaiA),
            &mut t as (*mut TaiA) as (*const TaiA),
        );
        d = taia_approx(&mut t as (*mut TaiA) as (*const TaiA));
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
