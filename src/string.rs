//! `str.rs`: string-related functions
//!
//! These should probably be replaced with functionality from the
//! Rust standard library

#[no_mangle]
pub unsafe extern "C" fn str_chr(mut s: *const u8, mut c: i32) -> u32 {
    let mut ch: u8;
    let mut t: *const u8;
    ch = c as (u8);
    t = s;
    'loop1: loop {
        if *t == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            break;
        }
        t = t.offset(1isize);
        if *t == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            break;
        }
        t = t.offset(1isize);
        if *t == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            break;
        }
        t = t.offset(1isize);
        if *t == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            break;
        }
        t = t.offset(1isize);
    }
    ((t as (isize)).wrapping_sub(s as (isize)) / ::std::mem::size_of::<u8>() as (isize)) as (u32)
}

#[no_mangle]
pub unsafe extern "C" fn str_diff(mut s: *const u8, mut t: *const u8) -> i32 {
    let mut x: u8;
    'loop1: loop {
        x = *s;
        if x as (i32) != *t as (i32) {
            break;
        }
        if x == 0 {
            break;
        }
        s = s.offset(1isize);
        t = t.offset(1isize);
        x = *s;
        if x as (i32) != *t as (i32) {
            break;
        }
        if x == 0 {
            break;
        }
        s = s.offset(1isize);
        t = t.offset(1isize);
        x = *s;
        if x as (i32) != *t as (i32) {
            break;
        }
        if x == 0 {
            break;
        }
        s = s.offset(1isize);
        t = t.offset(1isize);
        x = *s;
        if x as (i32) != *t as (i32) {
            break;
        }
        if x == 0 {
            break;
        }
        s = s.offset(1isize);
        t = t.offset(1isize);
    }
    x as (u32) as (i32) - *t as (u32) as (i32)
}

#[no_mangle]
pub unsafe extern "C" fn str_rchr(mut s: *const u8, mut c: i32) -> u32 {
    let mut ch: u8;
    let mut t: *const u8;
    let mut u: *const u8;
    ch = c as (u8);
    t = s;
    u = 0i32 as (*const u8);
    'loop1: loop {
        if *t == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            u = t;
        }
        t = t.offset(1isize);
        if *t == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            u = t;
        }
        t = t.offset(1isize);
        if *t == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            u = t;
        }
        t = t.offset(1isize);
        if *t == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            u = t;
        }
        t = t.offset(1isize);
    }
    if u.is_null() {
        u = t;
    }
    ((u as (isize)).wrapping_sub(s as (isize)) / ::std::mem::size_of::<u8>() as (isize)) as (u32)
}

#[no_mangle]
pub unsafe extern "C" fn str_start(mut s: *const u8, mut t: *const u8) -> i32 {
    let mut _currentBlock;
    let mut x: u8;
    'loop1: loop {
        x = *{
            let _old = t;
            t = t.offset(1isize);
            _old
        };
        if x == 0 {
            _currentBlock = 16;
            break;
        }
        if x as (i32) !=
            *{
                let _old = s;
                s = s.offset(1isize);
                _old
            } as (i32)
        {
            _currentBlock = 15;
            break;
        }
        x = *{
            let _old = t;
            t = t.offset(1isize);
            _old
        };
        if x == 0 {
            _currentBlock = 14;
            break;
        }
        if x as (i32) !=
            *{
                let _old = s;
                s = s.offset(1isize);
                _old
            } as (i32)
        {
            _currentBlock = 13;
            break;
        }
        x = *{
            let _old = t;
            t = t.offset(1isize);
            _old
        };
        if x == 0 {
            _currentBlock = 12;
            break;
        }
        if x as (i32) !=
            *{
                let _old = s;
                s = s.offset(1isize);
                _old
            } as (i32)
        {
            _currentBlock = 11;
            break;
        }
        x = *{
            let _old = t;
            t = t.offset(1isize);
            _old
        };
        if x == 0 {
            _currentBlock = 10;
            break;
        }
        if x as (i32) !=
            *{
                let _old = s;
                s = s.offset(1isize);
                _old
            } as (i32)
        {
            _currentBlock = 9;
            break;
        }
    }
    if _currentBlock == 9 {
        0i32
    } else if _currentBlock == 10 {
        1i32
    } else if _currentBlock == 11 {
        0i32
    } else if _currentBlock == 12 {
        1i32
    } else if _currentBlock == 13 {
        0i32
    } else if _currentBlock == 14 {
        1i32
    } else if _currentBlock == 15 {
        0i32
    } else {
        1i32
    }
}
