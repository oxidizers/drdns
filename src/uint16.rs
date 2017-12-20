//! `uint16.rs`: network byte order (i.e. big endian) conversions
//!
//! This should probably be replaced by the byteorder crate

pub unsafe fn pack(s: *mut u8, u: u16) {
    *s.offset(0isize) = (u as (i32) & 255i32) as (u8);
    *s.offset(1isize) = (u as (i32) >> 8i32) as (u8);
}

pub unsafe fn pack_big(s: *mut u8, u: u16) {
    *s.offset(1isize) = (u as (i32) & 255i32) as (u8);
    *s.offset(0isize) = (u as (i32) >> 8i32) as (u8);
}

pub unsafe fn unpack(s: *const u8, u: *mut u16) {
    let mut result: u16;
    result = *s.offset(1isize) as (u16);
    result = (result as (i32) << 8i32) as (u16);
    result = (result as (i32) + *s.offset(0isize) as (i32)) as (u16);
    *u = result;
}

pub unsafe fn unpack_big(s: *const u8, u: *mut u16) {
    let mut result: u16;
    result = *s.offset(0isize) as (u16);
    result = (result as (i32) << 8i32) as (u16);
    result = (result as (i32) + *s.offset(1isize) as (i32)) as (u16);
    *u = result;
}
