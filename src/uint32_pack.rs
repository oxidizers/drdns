#[no_mangle]
pub unsafe extern fn uint32_pack(mut s : *mut u8, mut u : u32) {
    *s.offset(0isize) = (u & 255u32) as (u8);
    u = u >> 8i32;
    *s.offset(1isize) = (u & 255u32) as (u8);
    u = u >> 8i32;
    *s.offset(2isize) = (u & 255u32) as (u8);
    *s.offset(3isize) = (u >> 8i32) as (u8);
}

#[no_mangle]
pub unsafe extern fn uint32_pack_big(mut s : *mut u8, mut u : u32) {
    *s.offset(3isize) = (u & 255u32) as (u8);
    u = u >> 8i32;
    *s.offset(2isize) = (u & 255u32) as (u8);
    u = u >> 8i32;
    *s.offset(1isize) = (u & 255u32) as (u8);
    *s.offset(0isize) = (u >> 8i32) as (u8);
}
