#[no_mangle]
pub unsafe extern fn byte_chr(
    mut s : *mut u8, mut n : u32, mut c : i32
) -> u32 {
    let mut ch : u8;
    let mut t : *mut u8;
    ch = c as (u8);
    t = s;
    'loop1: loop {
        if n == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            break;
        }
        t = t.offset(1isize);
        n = n.wrapping_sub(1u32);
        if n == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            break;
        }
        t = t.offset(1isize);
        n = n.wrapping_sub(1u32);
        if n == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            break;
        }
        t = t.offset(1isize);
        n = n.wrapping_sub(1u32);
        if n == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            break;
        }
        t = t.offset(1isize);
        n = n.wrapping_sub(1u32);
    }
    ((t as (isize)).wrapping_sub(
         s as (isize)
     ) / ::std::mem::size_of::<u8>() as (isize)) as (u32)
}
