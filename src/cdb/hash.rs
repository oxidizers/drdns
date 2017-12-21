//! `cdb/hash.rs`: C DataBase (CDB) hash function

pub unsafe fn hash(mut buf: *const u8, mut len: u32) -> u32 {
    let mut h = 5381u32;

    while len > 0 {
        h = add(h, *buf);
        buf = buf.offset(1isize);
        len -= 1;
    }

    h
}

fn add(h: u32, c: u8) -> u32 {
    h.wrapping_add(h << 5i32) ^ c as (u32)
}

#[test]
fn test_cdb_hash() {
    unsafe {
        assert_eq!(hash(b"".as_ptr(), 0), 0x0001505);
        assert_eq!(hash(b"Hello, world!".as_ptr(), 13), 0x564369e8);
        assert_eq!(hash(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".as_ptr(), 32), 0x40032705);
    }
}
