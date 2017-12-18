#[no_mangle]
pub unsafe extern "C" fn uint16_unpack(mut s: *const u8, mut u: *mut u16) {
    let mut result: u16;
    result = *s.offset(1isize) as (u16);
    result = (result as (i32) << 8i32) as (u16);
    result = (result as (i32) + *s.offset(0isize) as (i32)) as (u16);
    *u = result;
}

#[no_mangle]
pub unsafe extern "C" fn uint16_unpack_big(mut s: *const u8, mut u: *mut u16) {
    let mut result: u16;
    result = *s.offset(0isize) as (u16);
    result = (result as (i32) << 8i32) as (u16);
    result = (result as (i32) + *s.offset(1isize) as (i32)) as (u16);
    *u = result;
}
