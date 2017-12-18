#[no_mangle]
pub unsafe extern "C" fn cdb_hashadd(mut h: u32, mut c: u8) -> u32 {
    h = h.wrapping_add(h << 5i32);
    h ^ c as (u32)
}

#[no_mangle]
pub unsafe extern "C" fn cdb_hash(mut buf: *const u8, mut len: u32) -> u32 {
    let mut h: u32;
    h = 5381u32;
    'loop1: loop {
        if len == 0 {
            break;
        }
        h = cdb_hashadd(h, *{
            let _old = buf;
            buf = buf.offset(1isize);
            _old
        });
        len = len.wrapping_sub(1u32);
    }
    h
}
