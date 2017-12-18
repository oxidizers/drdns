extern "C" {
    fn tai_pack(arg1: *mut u8, arg2: *const tai);
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
    pub sec: tai,
    pub nano: usize,
    pub atto: usize,
}

impl Clone for taia {
    fn clone(&self) -> Self {
        *self
    }
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
