//! `dns/domain.rs`: DNS domain functions

use alloc;
use byte;
use case;
use errno::{self, Errno};
use libc;
use stralloc::StrAlloc;

pub unsafe fn length(dn: *const u8) -> u32 {
    let mut x: *const u8;
    let mut c: u8;
    x = dn;
    'loop1: loop {
        if {
            c = *{
                let _old = x;
                x = x.offset(1isize);
                _old
            };
            c
        } == 0
        {
            break;
        }
        x = x.offset(c as (u32) as (isize));
    }
    ((x as (isize)).wrapping_sub(dn as (isize)) / ::std::mem::size_of::<u8>() as (isize)) as (u32)
}

pub unsafe fn free(out: *mut *mut u8) {
    if !(*out).is_null() {
        alloc::free(*out);
        *out = 0i32 as (*mut u8);
    }
}

pub unsafe fn copy(out: *mut *mut u8, input: *const u8) -> i32 {
    let len: u32;
    let x: *mut u8;
    len = length(input);
    x = alloc::alloc(len);
    if x.is_null() {
        0i32
    } else {
        byte::copy(x, len, input as (*mut u8));
        if !(*out).is_null() {
            alloc::free(*out);
        }
        *out = x;
        1i32
    }
}

pub unsafe fn equal(dn1: *const u8, dn2: *const u8) -> i32 {
    let len = length(dn1);
    if len != length(dn2) {
        0i32
    } else if case::diffb(dn1, len, dn2) != 0 {
        0i32
    } else {
        1i32
    }
}

pub unsafe fn suffix(mut big: *const u8, little: *const u8) -> i32 {
    let current_block;
    let mut c: u8;
    'loop1: loop {
        if equal(big, little) != 0 {
            current_block = 5;
            break;
        }
        c = *{
            let _old = big;
            big = big.offset(1isize);
            _old
        };
        if c == 0 {
            current_block = 4;
            break;
        }
        big = big.offset(c as (isize));
    }
    if current_block == 4 { 0i32 } else { 1i32 }
}

pub unsafe fn suffixpos(mut big: *const u8, little: *const u8) -> u32 {
    let current_block;
    let orig: *const u8 = big;
    let mut c: u8;
    'loop1: loop {
        if equal(big, little) != 0 {
            current_block = 5;
            break;
        }
        c = *{
            let _old = big;
            big = big.offset(1isize);
            _old
        };
        if c == 0 {
            current_block = 4;
            break;
        }
        big = big.offset(c as (isize));
    }
    if current_block == 4 {
        0u32
    } else {
        ((big as (isize)).wrapping_sub(orig as (isize)) /
            ::std::mem::size_of::<u8>() as (isize)) as (u32)
    }
}

pub unsafe fn fromdot(out: *mut *mut u8, mut buf: *const u8, mut n: u32) -> i32 {
    let current_block;
    let mut label: [u8; 63] = [0u8; 63];
    let mut labellen: u32 = 0u32;
    let mut name: [u8; 255] = [0u8; 255];
    let mut namelen: u32 = 0u32;
    let mut ch: u8;
    let x: *mut u8;
    errno::set_errno(Errno(libc::EPROTO));
    'loop1: loop {
        if n == 0 {
            current_block = 16;
            break;
        }
        ch = *{
            let _old = buf;
            buf = buf.offset(1isize);
            _old
        };
        n = n.wrapping_sub(1u32);
        if ch as (i32) == b'.' as (i32) {
            if labellen == 0 {
                continue;
            }
            if namelen.wrapping_add(labellen).wrapping_add(1u32) as (usize) >
                ::std::mem::size_of::<[u8; 255]>()
            {
                current_block = 15;
                break;
            }
            name[{
                     let _old = namelen;
                     namelen = namelen.wrapping_add(1u32);
                     _old
                 } as (usize)] = labellen as (u8);
            byte::copy(
                name.as_mut_ptr().offset(namelen as (isize)),
                labellen,
                label.as_mut_ptr(),
            );
            namelen = namelen.wrapping_add(labellen);
            labellen = 0u32;
        } else {
            if ch as (i32) == b'\\' as (i32) {
                if n == 0 {
                    current_block = 16;
                    break;
                }
                ch = *{
                    let _old = buf;
                    buf = buf.offset(1isize);
                    _old
                };
                n = n.wrapping_sub(1u32);
                if ch as (i32) >= b'0' as (i32) && (ch as (i32) <= b'7' as (i32)) {
                    ch = (ch as (i32) - b'0' as (i32)) as (u8);
                    if n != 0 && (*buf as (i32) >= b'0' as (i32)) &&
                        (*buf as (i32) <= b'7' as (i32))
                    {
                        ch = (ch as (i32) << 3i32) as (u8);
                        ch = (ch as (i32) + (*buf as (i32) - b'0' as (i32))) as (u8);
                        buf = buf.offset(1isize);
                        n = n.wrapping_sub(1u32);
                        if n != 0 && (*buf as (i32) >= b'0' as (i32)) &&
                            (*buf as (i32) <= b'7' as (i32))
                        {
                            ch = (ch as (i32) << 3i32) as (u8);
                            ch = (ch as (i32) + (*buf as (i32) - b'0' as (i32))) as (u8);
                            buf = buf.offset(1isize);
                            n = n.wrapping_sub(1u32);
                        }
                    }
                }
            }
            if labellen as (usize) >= ::std::mem::size_of::<[u8; 63]>() {
                current_block = 11;
                break;
            }
            label[{
                      let _old = labellen;
                      labellen = labellen.wrapping_add(1u32);
                      _old
                  } as (usize)] = ch;
        }
    }
    if current_block == 11 {
        0i32
    } else if current_block == 15 {
        0i32
    } else {
        if labellen != 0 {
            if namelen.wrapping_add(labellen).wrapping_add(1u32) as (usize) >
                ::std::mem::size_of::<[u8; 255]>()
            {
                return 0i32;
            } else {
                name[{
                         let _old = namelen;
                         namelen = namelen.wrapping_add(1u32);
                         _old
                     } as (usize)] = labellen as (u8);
                byte::copy(
                    name.as_mut_ptr().offset(namelen as (isize)),
                    labellen,
                    label.as_mut_ptr(),
                );
                namelen = namelen.wrapping_add(labellen);
            }
        }
        (if namelen.wrapping_add(1u32) as (usize) > ::std::mem::size_of::<[u8; 255]>() {
             0i32
         } else {
             name[{
                      let _old = namelen;
                      namelen = namelen.wrapping_add(1u32);
                      _old
                  } as (usize)] = 0u8;
             x = alloc::alloc(namelen);
             (if x.is_null() {
                  0i32
              } else {
                  byte::copy(x, namelen, name.as_mut_ptr());
                  if !(*out).is_null() {
                      alloc::free(*out);
                  }
                  *out = x;
                  1i32
              })
         })
    }
}

pub unsafe fn todot_cat(out: *mut StrAlloc, mut d: *const u8) -> i32 {
    let current_block;
    let mut ch: u8;
    let mut ch2: u8;
    let mut ch3: u8;
    let mut buf: [u8; 4] = [0u8; 4];
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
                        current_block = 13;
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
                        current_block = 11;
                        break 'loop1;
                    }
                }
            }
            if *d == 0 {
                current_block = 6;
                break;
            }
            if StrAlloc::append(out, (*b".\0").as_ptr()) == 0 {
                current_block = 5;
                break;
            }
        }
        (if current_block == 5 {
             0i32
         } else if current_block == 6 {
             1i32
         } else if current_block == 11 {
             0i32
         } else {
             0i32
         })
    }
}
