extern {
    fn gettimeofday(
        arg1 : *mut timeval, arg2 : *mut ::std::os::raw::c_void
    ) -> i32;
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

#[derive(Copy)]
#[repr(C)]
pub struct timeval {
    pub tv_sec : isize,
    pub tv_usec : i32,
}

impl Clone for timeval {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest : i32,
    pub tz_dsttime : i32,
}

impl Clone for timezone {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn taia_now(mut t : *mut taia) {
    let mut now : timeval;
    gettimeofday(
        &mut now as (*mut timeval),
        0i32 as (*mut timezone) as (*mut ::std::os::raw::c_void)
    );
    (*(&mut (*t).sec as (*mut tai))).x = 4611686018427387914u64.wrapping_add(
                                             now.tv_sec as (usize) as (u64)
                                         ) as (usize);
    (*t).nano = (1000i32 * now.tv_usec + 500i32) as (usize);
    (*t).atto = 0usize;
}
