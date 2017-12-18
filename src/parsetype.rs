extern "C" {
    fn byte_copy(to: *mut u8, n: u32, from: *mut u8);
    fn case_diffs(arg1: *const u8, arg2: *const u8) -> i32;
    fn scan_ulong(arg1: *const u8, arg2: *mut usize) -> u32;
    fn uint16_pack_big(arg1: *mut u8, arg2: u16);
}

#[no_mangle]
pub unsafe extern "C" fn parsetype(mut s: *mut u8, mut type_: *mut u8) -> i32 {
    let mut u: usize;
    if *s.offset(scan_ulong(s as (*const u8), &mut u as (*mut usize)) as
        (isize)) == 0
    {
        uint16_pack_big(type_, u as (u16));
    } else if case_diffs(s as (*const u8), (*b"any\0").as_ptr()) == 0 {
        byte_copy(type_, 2u32, (*b"\0\xFF\0").as_ptr() as (*mut u8));
    } else if case_diffs(s as (*const u8), (*b"a\0").as_ptr()) == 0 {
        byte_copy(type_, 2u32, (*b"\0\x01\0").as_ptr() as (*mut u8));
    } else if case_diffs(s as (*const u8), (*b"ns\0").as_ptr()) == 0 {
        byte_copy(type_, 2u32, (*b"\0\x02\0").as_ptr() as (*mut u8));
    } else if case_diffs(s as (*const u8), (*b"mx\0").as_ptr()) == 0 {
        byte_copy(type_, 2u32, (*b"\0\x0F\0").as_ptr() as (*mut u8));
    } else if case_diffs(s as (*const u8), (*b"ptr\0").as_ptr()) == 0 {
        byte_copy(type_, 2u32, (*b"\0\x0C\0").as_ptr() as (*mut u8));
    } else if case_diffs(s as (*const u8), (*b"txt\0").as_ptr()) == 0 {
        byte_copy(type_, 2u32, (*b"\0\x10\0").as_ptr() as (*mut u8));
    } else if case_diffs(s as (*const u8), (*b"cname\0").as_ptr()) == 0 {
        byte_copy(type_, 2u32, (*b"\0\x05\0").as_ptr() as (*mut u8));
    } else if case_diffs(s as (*const u8), (*b"soa\0").as_ptr()) == 0 {
        byte_copy(type_, 2u32, (*b"\0\x06\0").as_ptr() as (*mut u8));
    } else if case_diffs(s as (*const u8), (*b"hinfo\0").as_ptr()) == 0 {
        byte_copy(type_, 2u32, (*b"\0\r\0").as_ptr() as (*mut u8));
    } else if case_diffs(s as (*const u8), (*b"rp\0").as_ptr()) == 0 {
        byte_copy(type_, 2u32, (*b"\0\x11\0").as_ptr() as (*mut u8));
    } else if case_diffs(s as (*const u8), (*b"sig\0").as_ptr()) == 0 {
        byte_copy(type_, 2u32, (*b"\0\x18\0").as_ptr() as (*mut u8));
    } else if case_diffs(s as (*const u8), (*b"key\0").as_ptr()) == 0 {
        byte_copy(type_, 2u32, (*b"\0\x19\0").as_ptr() as (*mut u8));
    } else if case_diffs(s as (*const u8), (*b"aaaa\0").as_ptr()) == 0 {
        byte_copy(type_, 2u32, (*b"\0\x1C\0").as_ptr() as (*mut u8));
    } else if case_diffs(s as (*const u8), (*b"axfr\0").as_ptr()) == 0 {
        byte_copy(type_, 2u32, (*b"\0\xFC\0").as_ptr() as (*mut u8));
    } else {
        return 0i32;
    }
    1i32
}
