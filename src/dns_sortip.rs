use byte;

extern "C" {
    fn dns_random(arg1: u32) -> u32;
}

#[no_mangle]
pub unsafe extern "C" fn dns_sortip(mut s: *mut u8, mut n: u32) {
    let mut i: u32;
    let mut tmp: [u8; 4];
    n = n >> 2i32;
    'loop1: loop {
        if !(n > 1u32) {
            break;
        }
        i = dns_random(n);
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
