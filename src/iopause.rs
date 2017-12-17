extern {
    fn poll(arg1 : *mut pollfd, arg2 : u32, arg3 : i32) -> i32;
    fn taia_approx(arg1 : *const taia) -> f64;
    fn taia_less(arg1 : *const taia, arg2 : *const taia) -> i32;
    fn taia_sub(
        arg1 : *mut taia, arg2 : *const taia, arg3 : *const taia
    );
}

#[derive(Copy)]
#[repr(C)]
pub struct pollfd {
    pub fd : i32,
    pub events : i16,
    pub revents : i16,
}

impl Clone for pollfd {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct tai {
    pub x : usize,
}

impl Clone for tai {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct taia {
    pub sec : tai,
    pub nano : usize,
    pub atto : usize,
}

impl Clone for taia {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn iopause(
    mut x : *mut pollfd,
    mut len : u32,
    mut deadline : *mut taia,
    mut stamp : *mut taia
) {
    let mut t : taia;
    let mut millisecs : i32;
    let mut d : f64;
    let mut i : i32;
    if taia_less(
           deadline as (*const taia),
           stamp as (*const taia)
       ) != 0 {
        millisecs = 0i32;
    } else {
        t = *stamp;
        taia_sub(
            &mut t as (*mut taia),
            deadline as (*const taia),
            &mut t as (*mut taia) as (*const taia)
        );
        d = taia_approx(&mut t as (*mut taia) as (*const taia));
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
    poll(x,len,millisecs);
}
