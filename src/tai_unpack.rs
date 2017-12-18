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
