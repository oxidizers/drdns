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
pub unsafe extern "C" fn tai_pack(mut s: *mut u8, mut t: *const tai) {
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
