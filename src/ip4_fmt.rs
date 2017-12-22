use ulong;

#[no_mangle]
pub unsafe extern "C" fn ip4_fmt(mut s: *mut u8, mut ip: *const u8) -> u32 {
    let mut len: u32;
    let mut i: u32;
    len = 0u32;
    i = ulong::fmt(s, *ip.offset(0isize) as (usize));
    len = len.wrapping_add(i);
    if !s.is_null() {
        s = s.offset(i as (isize));
    }
    if !s.is_null() {
        *{
            let _old = s;
            s = s.offset(1isize);
            _old
        } = b'.';
    }
    len = len.wrapping_add(1u32);
    i = ulong::fmt(s, *ip.offset(1isize) as (usize));
    len = len.wrapping_add(i);
    if !s.is_null() {
        s = s.offset(i as (isize));
    }
    if !s.is_null() {
        *{
            let _old = s;
            s = s.offset(1isize);
            _old
        } = b'.';
    }
    len = len.wrapping_add(1u32);
    i = ulong::fmt(s, *ip.offset(2isize) as (usize));
    len = len.wrapping_add(i);
    if !s.is_null() {
        s = s.offset(i as (isize));
    }
    if !s.is_null() {
        *{
            let _old = s;
            s = s.offset(1isize);
            _old
        } = b'.';
    }
    len = len.wrapping_add(1u32);
    i = ulong::fmt(s, *ip.offset(3isize) as (usize));
    len = len.wrapping_add(i);
    if !s.is_null() {
        s = s.offset(i as (isize));
    }
    len
}
