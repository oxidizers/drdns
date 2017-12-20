//! `uint32.rs`: network byte order (i.e. big endian) conversions
//!
//! This should probably be replaced by the byteorder crate

pub unsafe fn pack(s: *mut u8, mut u: u32) {
    *s.offset(0isize) = (u & 255u32) as (u8);
    u = u >> 8i32;
    *s.offset(1isize) = (u & 255u32) as (u8);
    u = u >> 8i32;
    *s.offset(2isize) = (u & 255u32) as (u8);
    *s.offset(3isize) = (u >> 8i32) as (u8);
}

pub unsafe fn pack_big(s: *mut u8, mut u: u32) {
    *s.offset(3isize) = (u & 255u32) as (u8);
    u = u >> 8i32;
    *s.offset(2isize) = (u & 255u32) as (u8);
    u = u >> 8i32;
    *s.offset(1isize) = (u & 255u32) as (u8);
    *s.offset(0isize) = (u >> 8i32) as (u8);
}

pub unsafe fn unpack(s: *const u8, u: *mut u32) {
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

pub unsafe fn unpack_big(s: *const u8, u: *mut u32) {
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
