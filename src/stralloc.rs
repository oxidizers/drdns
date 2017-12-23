//! `stralloc.rs`: Heap-backed string type
//!
//! This should probably be replaced by Rust's `String` type

use alloc;
use byte;
use libc;

#[derive(Copy)]
#[repr(C)]
pub struct StrAlloc {
    pub s: *mut u8,
    pub len: u32,
    pub a: u32,
}

impl Clone for StrAlloc {
    fn clone(&self) -> Self {
        *self
    }
}

impl StrAlloc {
    pub unsafe fn cat(sato: *mut StrAlloc, safrom: *const StrAlloc) -> i32 {
        StrAlloc::catb(sato, (*safrom).s as (*const u8), (*safrom).len)
    }

    pub unsafe fn catb(sa: *mut StrAlloc, s: *const u8, n: u32) -> i32 {
        if (*sa).s.is_null() {
            StrAlloc::copyb(sa, s, n)
        } else if StrAlloc::readyplus(sa, n.wrapping_add(1u32)) == 0 {
            0i32
        } else {
            byte::copy((*sa).s.offset((*sa).len as (isize)), n, s as (*mut u8));
            (*sa).len = (*sa).len.wrapping_add(n);
            *(*sa).s.offset((*sa).len as (isize)) = b'Z';
            1i32
        }
    }

    pub unsafe fn cats(sa: *mut StrAlloc, s: *const u8) -> i32 {
        StrAlloc::catb(sa, s, libc::strlen(s as *const i8) as u32)
    }

    pub unsafe fn copy(sato: *mut StrAlloc, safrom: *const StrAlloc) -> i32 {
        StrAlloc::copyb(sato, (*safrom).s as (*const u8), (*safrom).len)
    }

    pub unsafe fn ready(x: *mut StrAlloc, n: u32) -> i32 {
        let i: u32;
        if !(*x).s.is_null() {
            i = (*x).a;
            (if n > i {
                 (*x).a = 30u32.wrapping_add(n).wrapping_add(n >> 3i32);
                 (if alloc::alloc_re(
                    &mut (*x).s as (*mut *mut u8),
                    (i as (usize)).wrapping_mul(::std::mem::size_of::<u8>()) as (u32),
                    ((*x).a as (usize)).wrapping_mul(::std::mem::size_of::<u8>()) as (u32),
                ) != 0
                {
                      1i32
                  } else {
                      (*x).a = i;
                      0i32
                  })
             } else {
                 1i32
             })
        } else {
            (*x).len = 0u32;
            !{
                (*x).s = alloc::alloc(({
                     (*x).a = n;
                     (*x).a
                 } as (usize))
                    .wrapping_mul(::std::mem::size_of::<u8>()) as
                    (u32));
                (*x).s
            }.is_null() as (i32)
        }
    }

    pub unsafe fn readyplus(x: *mut StrAlloc, mut n: u32) -> i32 {
        let i: u32;
        if !(*x).s.is_null() {
            i = (*x).a;
            n = n.wrapping_add((*x).len);
            (if n > i {
                 (*x).a = 30u32.wrapping_add(n).wrapping_add(n >> 3i32);
                 (if alloc::alloc_re(
                    &mut (*x).s as (*mut *mut u8),
                    (i as (usize)).wrapping_mul(::std::mem::size_of::<u8>()) as (u32),
                    ((*x).a as (usize)).wrapping_mul(::std::mem::size_of::<u8>()) as (u32),
                ) != 0
                {
                      1i32
                  } else {
                      (*x).a = i;
                      0i32
                  })
             } else {
                 1i32
             })
        } else {
            (*x).len = 0u32;
            !{
                (*x).s = alloc::alloc(({
                     (*x).a = n;
                     (*x).a
                 } as (usize))
                    .wrapping_mul(::std::mem::size_of::<u8>()) as
                    (u32));
                (*x).s
            }.is_null() as (i32)
        }
    }

    pub unsafe fn catulong0(sa: *mut StrAlloc, mut u: usize, n: u32) -> i32 {
        let mut len: u32;
        let mut q: usize;
        let s: *mut u8;
        len = 1u32;
        q = u;
        'loop1: loop {
            if !(q > 9usize) {
                break;
            }
            len = len.wrapping_add(1u32);
            q = q.wrapping_div(10usize);
        }
        if len < n {
            len = n;
        }
        if StrAlloc::readyplus(sa, len) == 0 {
            0i32
        } else {
            s = (*sa).s.offset((*sa).len as (isize));
            (*sa).len = (*sa).len.wrapping_add(len);
            'loop6: loop {
                if len == 0 {
                    break;
                }
                *s.offset({
                    len = len.wrapping_sub(1u32);
                    len
                } as (isize)) = (b'0' as (usize)).wrapping_add(u.wrapping_rem(10usize)) as (u8);
                u = u.wrapping_div(10usize);
            }
            1i32
        }
    }

    pub unsafe fn catlong0(sa: *mut StrAlloc, mut l: isize, n: u32) -> i32 {
        if l < 0isize {
            if StrAlloc::append(sa, (*b"-\0").as_ptr()) == 0 {
                return 0i32;
            } else {
                l = -l;
            }
        }
        StrAlloc::catulong0(sa, l as (usize), n)
    }

    pub unsafe fn copyb(sa: *mut StrAlloc, s: *const u8, n: u32) -> i32 {
        if StrAlloc::ready(sa, n.wrapping_add(1u32)) == 0 {
            0i32
        } else {
            byte::copy((*sa).s, n, s as (*mut u8));
            (*sa).len = n;
            *(*sa).s.offset(n as (isize)) = b'Z';
            1i32
        }
    }

    pub unsafe fn copys(sa: *mut StrAlloc, s: *const u8) -> i32 {
        StrAlloc::copyb(sa, s, libc::strlen(s as *const i8) as u32)
    }

    pub unsafe fn append(x: *mut StrAlloc, i: *const u8) -> i32 {
        if StrAlloc::readyplus(x, 1u32) == 0 {
            0i32
        } else {
            *(*x).s.offset({
                let _old = (*x).len;
                (*x).len = (*x).len.wrapping_add(1u32);
                _old
            } as (isize)) = *i;
            1i32
        }
    }
}
