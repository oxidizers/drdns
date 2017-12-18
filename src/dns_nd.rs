extern "C" {
    fn byte_copy(to: *mut u8, n: u32, from: *mut u8);
    fn fmt_ulong(arg1: *mut u8, arg2: usize) -> u32;
}

#[no_mangle]
pub unsafe extern "C" fn dns_name4_domain(mut name: *mut u8, mut ip: *const u8) {
    let mut namelen: u32;
    let mut i: u32;
    namelen = 0u32;
    i = fmt_ulong(
        name.offset(namelen as (isize)).offset(1isize),
        *ip.offset(3isize) as (usize),
    );
    *name.offset({
        let _old = namelen;
        namelen = namelen.wrapping_add(1u32);
        _old
    } as (isize)) = i as (u8);
    namelen = namelen.wrapping_add(i);
    i = fmt_ulong(
        name.offset(namelen as (isize)).offset(1isize),
        *ip.offset(2isize) as (usize),
    );
    *name.offset({
        let _old = namelen;
        namelen = namelen.wrapping_add(1u32);
        _old
    } as (isize)) = i as (u8);
    namelen = namelen.wrapping_add(i);
    i = fmt_ulong(
        name.offset(namelen as (isize)).offset(1isize),
        *ip.offset(1isize) as (usize),
    );
    *name.offset({
        let _old = namelen;
        namelen = namelen.wrapping_add(1u32);
        _old
    } as (isize)) = i as (u8);
    namelen = namelen.wrapping_add(i);
    i = fmt_ulong(
        name.offset(namelen as (isize)).offset(1isize),
        *ip.offset(0isize) as (usize),
    );
    *name.offset({
        let _old = namelen;
        namelen = namelen.wrapping_add(1u32);
        _old
    } as (isize)) = i as (u8);
    namelen = namelen.wrapping_add(i);
    byte_copy(
        name.offset(namelen as (isize)),
        14u32,
        (*b"\x07in-addr\x04arpa\0\0").as_ptr() as (*mut u8),
    );
}
