#[no_mangle]
pub unsafe extern "C" fn byte_chr(mut s: *mut u8, mut n: u32, mut c: i32) -> u32 {
    let mut ch: u8;
    let mut t: *mut u8;
    ch = c as (u8);
    t = s;
    'loop1: loop {
        if n == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            break;
        }
        t = t.offset(1isize);
        n = n.wrapping_sub(1u32);
        if n == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            break;
        }
        t = t.offset(1isize);
        n = n.wrapping_sub(1u32);
        if n == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            break;
        }
        t = t.offset(1isize);
        n = n.wrapping_sub(1u32);
        if n == 0 {
            break;
        }
        if *t as (i32) == ch as (i32) {
            break;
        }
        t = t.offset(1isize);
        n = n.wrapping_sub(1u32);
    }
    ((t as (isize)).wrapping_sub(s as (isize)) / ::std::mem::size_of::<u8>() as (isize)) as (u32)
}

#[no_mangle]
pub unsafe extern "C" fn byte_copy(mut to: *mut u8, mut n: u32, mut from: *mut u8) {
    let mut _currentBlock;
    'loop0: loop {
        if n == 0 {
            _currentBlock = 8;
            break;
        }
        *{
            let _old = to;
            to = to.offset(1isize);
            _old
        } = *{
            let _old = from;
            from = from.offset(1isize);
            _old
        };
        n = n.wrapping_sub(1u32);
        if n == 0 {
            _currentBlock = 7;
            break;
        }
        *{
            let _old = to;
            to = to.offset(1isize);
            _old
        } = *{
            let _old = from;
            from = from.offset(1isize);
            _old
        };
        n = n.wrapping_sub(1u32);
        if n == 0 {
            _currentBlock = 6;
            break;
        }
        *{
            let _old = to;
            to = to.offset(1isize);
            _old
        } = *{
            let _old = from;
            from = from.offset(1isize);
            _old
        };
        n = n.wrapping_sub(1u32);
        if n == 0 {
            _currentBlock = 5;
            break;
        }
        *{
            let _old = to;
            to = to.offset(1isize);
            _old
        } = *{
            let _old = from;
            from = from.offset(1isize);
            _old
        };
        n = n.wrapping_sub(1u32);
    }
    if _currentBlock == 5 {
    } else if _currentBlock == 6 {
    } else if _currentBlock == 7 {
    }
}

#[no_mangle]
pub unsafe extern "C" fn byte_copyr(mut to: *mut u8, mut n: u32, mut from: *mut u8) {
    let mut _currentBlock;
    to = to.offset(n as (isize));
    from = from.offset(n as (isize));
    'loop1: loop {
        if n == 0 {
            _currentBlock = 9;
            break;
        }
        *{
            to = to.offset(-1isize);
            to
        } = *{
            from = from.offset(-1isize);
            from
        };
        n = n.wrapping_sub(1u32);
        if n == 0 {
            _currentBlock = 8;
            break;
        }
        *{
            to = to.offset(-1isize);
            to
        } = *{
            from = from.offset(-1isize);
            from
        };
        n = n.wrapping_sub(1u32);
        if n == 0 {
            _currentBlock = 7;
            break;
        }
        *{
            to = to.offset(-1isize);
            to
        } = *{
            from = from.offset(-1isize);
            from
        };
        n = n.wrapping_sub(1u32);
        if n == 0 {
            _currentBlock = 6;
            break;
        }
        *{
            to = to.offset(-1isize);
            to
        } = *{
            from = from.offset(-1isize);
            from
        };
        n = n.wrapping_sub(1u32);
    }
    if _currentBlock == 6 {
    } else if _currentBlock == 7 {
    } else if _currentBlock == 8 {
    }
}

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

#[no_mangle]
pub unsafe extern "C" fn byte_zero(mut s: *mut u8, mut n: u32) {
    'loop0: loop {
        if n == 0 {
            break;
        }
        *{
            let _old = s;
            s = s.offset(1isize);
            _old
        } = 0u8;
        n = n.wrapping_sub(1u32);
        if n == 0 {
            break;
        }
        *{
            let _old = s;
            s = s.offset(1isize);
            _old
        } = 0u8;
        n = n.wrapping_sub(1u32);
        if n == 0 {
            break;
        }
        *{
            let _old = s;
            s = s.offset(1isize);
            _old
        } = 0u8;
        n = n.wrapping_sub(1u32);
        if n == 0 {
            break;
        }
        *{
            let _old = s;
            s = s.offset(1isize);
            _old
        } = 0u8;
        n = n.wrapping_sub(1u32);
    }
}
