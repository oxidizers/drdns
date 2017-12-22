//! `case.rs`: case comparison utilities
//!
//! These should probably be replaced with functionality from the
//! Rust standard library

pub unsafe fn diffb(mut s: *const u8, mut len: u32, mut t: *const u8) -> i32 {
    let current_block;
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    'loop1: loop {
        if !(len > 0u32) {
            current_block = 2;
            break;
        }
        len = len.wrapping_sub(1u32);
        x = (*{
                 let _old = s;
                 s = s.offset(1isize);
                 _old
             } as (i32) - b'A' as (i32)) as (u8);
        if x as (i32) <= b'Z' as (i32) - b'A' as (i32) {
            x = (x as (i32) + b'a' as (i32)) as (u8);
        } else {
            x = (x as (i32) + b'A' as (i32)) as (u8);
        }
        y = (*{
                 let _old = t;
                 t = t.offset(1isize);
                 _old
             } as (i32) - b'A' as (i32)) as (u8);
        if y as (i32) <= b'Z' as (i32) - b'A' as (i32) {
            y = (y as (i32) + b'a' as (i32)) as (u8);
        } else {
            y = (y as (i32) + b'A' as (i32)) as (u8);
        }
        if x as (i32) != y as (i32) {
            current_block = 10;
            break;
        }
    }
    if current_block == 2 {
        0i32
    } else {
        x as (u32) as (i32) - y as (u32) as (i32)
    }
}

pub unsafe fn diffs(mut s: *const u8, mut t: *const u8) -> i32 {
    let mut x: u8;
    let mut y: u8;
    'loop1: loop {
        x = (*{
                 let _old = s;
                 s = s.offset(1isize);
                 _old
             } as (i32) - b'A' as (i32)) as (u8);
        if x as (i32) <= b'Z' as (i32) - b'A' as (i32) {
            x = (x as (i32) + b'a' as (i32)) as (u8);
        } else {
            x = (x as (i32) + b'A' as (i32)) as (u8);
        }
        y = (*{
                 let _old = t;
                 t = t.offset(1isize);
                 _old
             } as (i32) - b'A' as (i32)) as (u8);
        if y as (i32) <= b'Z' as (i32) - b'A' as (i32) {
            y = (y as (i32) + b'a' as (i32)) as (u8);
        } else {
            y = (y as (i32) + b'A' as (i32)) as (u8);
        }
        if x as (i32) != y as (i32) {
            break;
        }
        if x == 0 {
            break;
        }
    }
    x as (u32) as (i32) - y as (u32) as (i32)
}

pub unsafe fn lowerb(mut s: *mut u8, mut len: u32) {
    let mut x: u8;
    'loop1: loop {
        if !(len > 0u32) {
            break;
        }
        len = len.wrapping_sub(1u32);
        x = (*s as (i32) - b'A' as (i32)) as (u8);
        if x as (i32) <= b'Z' as (i32) - b'A' as (i32) {
            *s = (x as (i32) + b'a' as (i32)) as (u8);
        }
        s = s.offset(1isize);
    }
}
