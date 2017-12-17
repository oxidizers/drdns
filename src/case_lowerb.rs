#[no_mangle]
pub unsafe extern fn case_lowerb(mut s : *mut u8, mut len : u32) {
    let mut x : u8;
    'loop1: loop {
        if !(len > 0u32) {
            break;
        }
        len = len.wrapping_sub(1u32);
        x = (*s as (i32) - b'A' as (i32)) as (u8);
        if x as (i32) <= b'Z' as (i32) - b'A' as (i32) {
            *s = (x as (i32) + b'a' as (i32)) as (u8);
        }
        s = s.offset(1isize);
    }
}
