//! `string.rs`: string-related functions
//!
//! These should probably be replaced with functionality from the
//! Rust standard library

pub unsafe fn chr(s: *const u8, c: i32) -> u32 {
    let ch: u8;
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

pub unsafe fn diff(mut s: *const u8, mut t: *const u8) -> i32 {
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

pub unsafe fn rchr(s: *const u8, c: i32) -> u32 {
    let ch: u8;
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

pub unsafe fn start(mut s: *const u8, mut t: *const u8) -> i32 {
    let current_block;
    let mut x: u8;
    'loop1: loop {
        x = *{
            let _old = t;
            t = t.offset(1isize);
            _old
        };
        if x == 0 {
            current_block = 16;
            break;
        }
        if x as (i32) !=
            *{
                let _old = s;
                s = s.offset(1isize);
                _old
            } as (i32)
        {
            current_block = 15;
            break;
        }
        x = *{
            let _old = t;
            t = t.offset(1isize);
            _old
        };
        if x == 0 {
            current_block = 14;
            break;
        }
        if x as (i32) !=
            *{
                let _old = s;
                s = s.offset(1isize);
                _old
            } as (i32)
        {
            current_block = 13;
            break;
        }
        x = *{
            let _old = t;
            t = t.offset(1isize);
            _old
        };
        if x == 0 {
            current_block = 12;
            break;
        }
        if x as (i32) !=
            *{
                let _old = s;
                s = s.offset(1isize);
                _old
            } as (i32)
        {
            current_block = 11;
            break;
        }
        x = *{
            let _old = t;
            t = t.offset(1isize);
            _old
        };
        if x == 0 {
            current_block = 10;
            break;
        }
        if x as (i32) !=
            *{
                let _old = s;
                s = s.offset(1isize);
                _old
            } as (i32)
        {
            current_block = 9;
            break;
        }
    }
    if current_block == 9 {
        0i32
    } else if current_block == 10 {
        1i32
    } else if current_block == 11 {
        0i32
    } else if current_block == 12 {
        1i32
    } else if current_block == 13 {
        0i32
    } else if current_block == 14 {
        1i32
    } else if current_block == 15 {
        0i32
    } else {
        1i32
    }
}
