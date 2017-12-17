#[no_mangle]
pub unsafe extern fn uint32_unpack(
    mut s : *const u8, mut u : *mut u32
) {
    let mut result : u32;
    result = *s.offset(3isize) as (u32);
    result = result << 8i32;
    result = result.wrapping_add(*s.offset(2isize) as (u32));
    result = result << 8i32;
    result = result.wrapping_add(*s.offset(1isize) as (u32));
    result = result << 8i32;
    result = result.wrapping_add(*s.offset(0isize) as (u32));
    *u = result;
}

#[no_mangle]
pub unsafe extern fn uint32_unpack_big(
    mut s : *const u8, mut u : *mut u32
) {
    let mut result : u32;
    result = *s.offset(0isize) as (u32);
    result = result << 8i32;
    result = result.wrapping_add(*s.offset(1isize) as (u32));
    result = result << 8i32;
    result = result.wrapping_add(*s.offset(2isize) as (u32));
    result = result << 8i32;
    result = result.wrapping_add(*s.offset(3isize) as (u32));
    *u = result;
}
