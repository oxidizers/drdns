extern "C" {
    fn dns_domain_equal(arg1: *const u8, arg2: *const u8) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn dd(mut q: *const u8, mut base: *const u8, mut ip: *mut u8) -> i32 {
    let mut _currentBlock;
    let mut j: i32;
    let mut x: u32;
    j = 0i32;
    'loop1: loop {
        if dns_domain_equal(q, base) != 0 {
            _currentBlock = 24;
            break;
        }
        if j >= 4i32 {
            _currentBlock = 23;
            break;
        }
        if *q as (i32) <= 0i32 {
            _currentBlock = 22;
            break;
        }
        if *q as (i32) >= 4i32 {
            _currentBlock = 21;
            break;
        }
        if *q.offset(1isize) as (i32) < b'0' as (i32) ||
            *q.offset(1isize) as (i32) > b'9' as (i32)
        {
            _currentBlock = 20;
            break;
        }
        x = (*q.offset(1isize) as (i32) - b'0' as (i32)) as (u32);
        if *q as (i32) == 1i32 {
            *ip.offset(j as (isize)) = x as (u8);
            q = q.offset(2isize);
        } else {
            if x == 0 {
                _currentBlock = 17;
                break;
            }
            if *q.offset(2isize) as (i32) < b'0' as (i32) ||
                *q.offset(2isize) as (i32) > b'9' as (i32)
            {
                _currentBlock = 16;
                break;
            }
            x = x.wrapping_mul(10u32).wrapping_add(
                (*q.offset(2isize) as (i32) - b'0' as (i32)) as
                    (u32),
            );
            if *q as (i32) == 2i32 {
                *ip.offset(j as (isize)) = x as (u8);
                q = q.offset(3isize);
            } else {
                if *q.offset(3isize) as (i32) < b'0' as (i32) ||
                    *q.offset(3isize) as (i32) > b'9' as (i32)
                {
                    _currentBlock = 14;
                    break;
                }
                x = x.wrapping_mul(10u32).wrapping_add(
                    (*q.offset(3isize) as (i32) - b'0' as (i32)) as
                        (u32),
                );
                if x > 255u32 {
                    _currentBlock = 13;
                    break;
                }
                *ip.offset(j as (isize)) = x as (u8);
                q = q.offset(4isize);
            }
        }
        j = j + 1;
    }
    if _currentBlock == 13 {
        -1i32
    } else if _currentBlock == 14 {
        -1i32
    } else if _currentBlock == 16 {
        -1i32
    } else if _currentBlock == 17 {
        -1i32
    } else if _currentBlock == 20 {
        -1i32
    } else if _currentBlock == 21 {
        -1i32
    } else if _currentBlock == 22 {
        -1i32
    } else if _currentBlock == 23 {
        -1i32
    } else {
        j
    }
}
