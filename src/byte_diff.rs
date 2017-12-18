#[no_mangle]
pub unsafe extern "C" fn byte_diff(mut s: *mut u8, mut n: u32, mut t: *mut u8) -> i32 {
    let mut _currentBlock;
    'loop0: loop {
        if n == 0 {
            _currentBlock = 13;
            break;
        }
        if *s as (i32) != *t as (i32) {
            _currentBlock = 12;
            break;
        }
        s = s.offset(1isize);
        t = t.offset(1isize);
        n = n.wrapping_sub(1u32);
        if n == 0 {
            _currentBlock = 11;
            break;
        }
        if *s as (i32) != *t as (i32) {
            _currentBlock = 12;
            break;
        }
        s = s.offset(1isize);
        t = t.offset(1isize);
        n = n.wrapping_sub(1u32);
        if n == 0 {
            _currentBlock = 10;
            break;
        }
        if *s as (i32) != *t as (i32) {
            _currentBlock = 12;
            break;
        }
        s = s.offset(1isize);
        t = t.offset(1isize);
        n = n.wrapping_sub(1u32);
        if n == 0 {
            _currentBlock = 9;
            break;
        }
        if *s as (i32) != *t as (i32) {
            _currentBlock = 12;
            break;
        }
        s = s.offset(1isize);
        t = t.offset(1isize);
        n = n.wrapping_sub(1u32);
    }
    if _currentBlock == 9 {
        0i32
    } else if _currentBlock == 10 {
        0i32
    } else if _currentBlock == 11 {
        0i32
    } else if _currentBlock == 12 {
        *s as (u32) as (i32) - *t as (u32) as (i32)
    } else {
        0i32
    }
}
