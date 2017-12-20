//! `uint32.rs`: network byte order (i.e. big endian) conversions
//!
//! This should probably be replaced by the byteorder crate

#[no_mangle]
pub unsafe extern "C" fn uint32_pack(mut s: *mut u8, mut u: u32) {
    *s.offset(0isize) = (u & 255u32) as (u8);
    u = u >> 8i32;
    *s.offset(1isize) = (u & 255u32) as (u8);
    u = u >> 8i32;
    *s.offset(2isize) = (u & 255u32) as (u8);
    *s.offset(3isize) = (u >> 8i32) as (u8);
}

#[no_mangle]
pub unsafe extern "C" fn uint32_pack_big(mut s: *mut u8, mut u: u32) {
    *s.offset(3isize) = (u & 255u32) as (u8);
    u = u >> 8i32;
    *s.offset(2isize) = (u & 255u32) as (u8);
    u = u >> 8i32;
    *s.offset(1isize) = (u & 255u32) as (u8);
    *s.offset(0isize) = (u >> 8i32) as (u8);
}
#[no_mangle]
pub unsafe extern "C" fn uint32_unpack(mut s: *const u8, mut u: *mut u32) {
    let mut result: u32;
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
pub unsafe extern "C" fn uint32_unpack_big(mut s: *const u8, mut u: *mut u32) {
    let mut result: u32;
    result = *s.offset(0isize) as (u32);
    result = result << 8i32;
    result = result.wrapping_add(*s.offset(1isize) as (u32));
    result = result << 8i32;
    result = result.wrapping_add(*s.offset(2isize) as (u32));
    result = result << 8i32;
    result = result.wrapping_add(*s.offset(3isize) as (u32));
    *u = result;
}
