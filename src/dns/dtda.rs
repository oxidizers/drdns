use stralloc::StrAlloc;

#[no_mangle]
pub unsafe extern "C" fn dns_domain_todot_cat(mut out: *mut StrAlloc, mut d: *const u8) -> i32 {
    let mut _currentBlock;
    let mut ch: u8;
    let mut ch2: u8;
    let mut ch3: u8;
    let mut buf: [u8; 4];
    if *d == 0 {
        StrAlloc::append(out, (*b".\0").as_ptr())
    } else {
        'loop1: loop {
            ch = *{
                let _old = d;
                d = d.offset(1isize);
                _old
            };
            'loop2: loop {
                if {
                    let _old = ch;
                    ch = (ch as (i32) - 1) as (u8);
                    _old
                } == 0
                {
                    break;
                }
                ch2 = *{
                    let _old = d;
                    d = d.offset(1isize);
                    _old
                };
                if ch2 as (i32) >= b'A' as (i32) && (ch2 as (i32) <= b'Z' as (i32)) {
                    ch2 = (ch2 as (i32) + 32i32) as (u8);
                }
                if ch2 as (i32) >= b'a' as (i32) && (ch2 as (i32) <= b'z' as (i32)) ||
                    ch2 as (i32) >= b'0' as (i32) && (ch2 as (i32) <= b'9' as (i32)) ||
                    ch2 as (i32) == b'-' as (i32) ||
                    ch2 as (i32) == b'_' as (i32)
                {
                    if StrAlloc::append(out, &mut ch2 as (*mut u8) as (*const u8)) == 0 {
                        _currentBlock = 13;
                        break 'loop1;
                    }
                } else {
                    ch3 = ch2;
                    buf[3usize] = (b'0' as (i32) + (ch3 as (i32) & 7i32)) as (u8);
                    ch3 = (ch3 as (i32) >> 3i32) as (u8);
                    buf[2usize] = (b'0' as (i32) + (ch3 as (i32) & 7i32)) as (u8);
                    ch3 = (ch3 as (i32) >> 3i32) as (u8);
                    buf[1usize] = (b'0' as (i32) + (ch3 as (i32) & 7i32)) as (u8);
                    buf[0usize] = b'\\';
                    if StrAlloc::catb(out, buf.as_mut_ptr() as (*const u8), 4u32) == 0 {
                        _currentBlock = 11;
                        break 'loop1;
                    }
                }
            }
            if *d == 0 {
                _currentBlock = 6;
                break;
            }
            if StrAlloc::append(out, (*b".\0").as_ptr()) == 0 {
                _currentBlock = 5;
                break;
            }
        }
        (if _currentBlock == 5 {
             0i32
         } else if _currentBlock == 6 {
             1i32
         } else if _currentBlock == 11 {
             0i32
         } else {
             0i32
         })
    }
}
