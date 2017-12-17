#[no_mangle]
pub unsafe extern fn str_rchr(mut s : *const u8, mut c : i32) -> u32 {
    let mut ch : u8;
    let mut t : *const u8;
    let mut u : *const u8;
    ch = c as (u8);
    t = s;
    u = 0i32 as (*const u8);
    'loop1: loop {
        if *t == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            u = t;
        }
        t = t.offset(1isize);
        if *t == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            u = t;
        }
        t = t.offset(1isize);
        if *t == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            u = t;
        }
        t = t.offset(1isize);
        if *t == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            u = t;
        }
        t = t.offset(1isize);
    }
    if u.is_null() {
        u = t;
    }
    ((u as (isize)).wrapping_sub(
         s as (isize)
     ) / ::std::mem::size_of::<u8>() as (isize)) as (u32)
}
