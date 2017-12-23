use byte;
use super::random;

pub unsafe fn sortip(s: *mut u8, mut n: u32) {
    let mut i: u32;
    let mut tmp: [u8; 4] = [0u8; 4];
    n = n >> 2i32;
    'loop1: loop {
        if !(n > 1u32) {
            break;
        }
        i = random::random(n);
        n = n.wrapping_sub(1u32);
        byte::copy(tmp.as_mut_ptr(), 4u32, s.offset((i << 2i32) as (isize)));
        byte::copy(
            s.offset((i << 2i32) as (isize)),
            4u32,
            s.offset((n << 2i32) as (isize)),
        );
        byte::copy(s.offset((n << 2i32) as (isize)), 4u32, tmp.as_mut_ptr());
    }
}
