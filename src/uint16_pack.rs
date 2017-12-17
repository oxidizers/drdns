#[no_mangle]
pub unsafe extern fn uint16_pack(mut s : *mut u8, mut u : u16) {
    *s.offset(0isize) = (u as (i32) & 255i32) as (u8);
    *s.offset(1isize) = (u as (i32) >> 8i32) as (u8);
}

#[no_mangle]
pub unsafe extern fn uint16_pack_big(mut s : *mut u8, mut u : u16) {
    *s.offset(1isize) = (u as (i32) & 255i32) as (u8);
    *s.offset(0isize) = (u as (i32) >> 8i32) as (u8);
}
