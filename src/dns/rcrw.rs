//! `dns/rcrw.rs`: rewrite resolv.conf

use byte;
use libc;
use openreadclose::openreadclose;
use stralloc::StrAlloc;
use string;
use tai::Tai;
use taia::TaiA;

static mut DATA: StrAlloc = StrAlloc {
    s: 0i32 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

static mut OK: i32 = 0i32;

static mut USES: u32 = 0u32;

static mut DEADLINE: TaiA = TaiA {
    sec: Tai { x: 0usize },
    nano: 0usize,
    atto: 0usize,
};

static mut RULES: StrAlloc = StrAlloc {
    s: 0i32 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

pub unsafe fn resolvconfrewrite(out: *mut StrAlloc) -> i32 {
    let mut now: TaiA = ::std::mem::zeroed();
    TaiA::now(&mut now as (*mut TaiA));
    if TaiA::less(
        &mut DEADLINE as (*mut TaiA) as (*const TaiA),
        &mut now as (*mut TaiA) as (*const TaiA),
    ) != 0
    {
        OK = 0i32;
    }
    if USES == 0 {
        OK = 0i32;
    }
    if OK == 0 {
        if init(&mut RULES as (*mut StrAlloc)) == -1i32 {
            return -1i32;
        } else {
            TaiA::uint(&mut DEADLINE as (*mut TaiA), 600u32);
            TaiA::add(
                &mut DEADLINE as (*mut TaiA),
                &mut now as (*mut TaiA) as (*const TaiA),
                &mut DEADLINE as (*mut TaiA) as (*const TaiA),
            );
            USES = 10000u32;
            OK = 1i32;
        }
    }
    USES = USES.wrapping_sub(1u32);
    if StrAlloc::copy(out, &mut RULES as (*mut StrAlloc) as (*const StrAlloc)) == 0 {
        -1i32
    } else {
        0i32
    }
}

unsafe fn init(rules: *mut StrAlloc) -> i32 {
    let mut current_block;
    let mut host: [u8; 256] = [0u8; 256];
    let mut x: *const u8;
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    if StrAlloc::copys(rules, (*b"\0").as_ptr()) == 0 {
        -1i32
    } else {
        x = libc::getenv((*b"DNSREWRITEFILE\0").as_ptr() as *const libc::c_char) as (*const u8);
        if x.is_null() {
            x = (*b"/etc/dnsrewrite\0").as_ptr();
        }
        i = openreadclose(x, &mut DATA as (*mut StrAlloc), 64u32);
        (if i == -1i32 {
             -1i32
         } else if i != 0 {
             (if StrAlloc::append(&mut DATA as (*mut StrAlloc), (*b"\n\0").as_ptr()) == 0 {
                  -1i32
              } else {
                  i = 0i32;
                  j = 0i32;
                  'loop73: loop {
                      if !(j as (u32) < DATA.len) {
                          current_block = 74;
                          break;
                      }
                      if *DATA.s.offset(j as (isize)) as (i32) == b'\n' as (i32) {
                          if StrAlloc::catb(
                            rules,
                            DATA.s.offset(i as (isize)) as (*const u8),
                            (j - i) as (u32),
                        ) == 0
                        {
                              current_block = 86;
                              break;
                          }
                          'loop77: loop {
                              if (*rules).len == 0 {
                                  break;
                              }
                              if *(*rules).s.offset(
                                (*rules).len.wrapping_sub(1u32) as (isize),
                            ) as (i32) != b' ' as (i32)
                            {
                                  if *(*rules).s.offset(
                                    (*rules).len.wrapping_sub(1u32) as (isize),
                                ) as (i32) != b'\t' as (i32)
                                {
                                      if *(*rules).s.offset(
                                        (*rules).len.wrapping_sub(1u32) as
                                            (isize),
                                    ) as (i32) !=
                                          b'\r' as (i32)
                                    {
                                          break;
                                      }
                                  }
                              }
                              (*rules).len = (*rules).len.wrapping_sub(1u32);
                          }
                          if StrAlloc::append(rules, (*b"\0").as_ptr()) == 0 {
                              current_block = 85;
                              break;
                          }
                          i = j + 1i32;
                      }
                      j = j + 1;
                  }
                  (if current_block == 74 {
                       0i32
                   } else if current_block == 85 {
                       -1i32
                   } else {
                       -1i32
                   })
              })
         } else {
             x = libc::getenv((*b"LOCALDOMAIN\0").as_ptr() as *const libc::c_char) as (*const u8);
             (if !x.is_null() {
                  (if StrAlloc::copys(&mut DATA as (*mut StrAlloc), x) == 0 {
                       -1i32
                   } else if StrAlloc::append(&mut DATA as (*mut StrAlloc), (*b" \0").as_ptr()) ==
                              0
                {
                       -1i32
                   } else if StrAlloc::copys(rules, (*b"?:\0").as_ptr()) == 0 {
                       -1i32
                   } else {
                       i = 0i32;
                       j = 0i32;
                       'loop53: loop {
                           if !(j as (u32) < DATA.len) {
                               current_block = 54;
                               break;
                           }
                           if *DATA.s.offset(j as (isize)) as (i32) == b' ' as (i32) {
                               if StrAlloc::cats(rules, (*b"+.\0").as_ptr()) == 0 {
                                   current_block = 67;
                                   break;
                               }
                               if StrAlloc::catb(
                                rules,
                                DATA.s.offset(i as (isize)) as (*const u8),
                                (j - i) as (u32),
                            ) == 0
                            {
                                   current_block = 66;
                                   break;
                               }
                               i = j + 1i32;
                           }
                           j = j + 1;
                       }
                       (if current_block == 54 {
                            (if StrAlloc::append(rules, (*b"\0").as_ptr()) == 0 {
                                 -1i32
                             } else if StrAlloc::cats(rules, (*b"*.:\0").as_ptr()) == 0 {
                                 -1i32
                             } else if StrAlloc::append(rules, (*b"\0").as_ptr()) == 0 {
                                 -1i32
                             } else {
                                 0i32
                             })
                        } else if current_block == 66 {
                            -1i32
                        } else {
                            -1i32
                        })
                   })
              } else {
                  i = openreadclose(
                    (*b"/etc/resolv.conf\0").as_ptr(),
                    &mut DATA as (*mut StrAlloc),
                    64u32,
                );
                  (if i == -1i32 {
                       -1i32
                   } else {
                       if i != 0 {
                           if StrAlloc::append(
                            &mut DATA as (*mut StrAlloc),
                            (*b"\n\0").as_ptr(),
                        ) == 0
                        {
                               return -1i32;
                           } else {
                               i = 0i32;
                               j = 0i32;
                               'loop10: loop {
                                   if !(j as (u32) < DATA.len) {
                                       current_block = 11;
                                       break;
                                   }
                                   if *DATA.s.offset(j as (isize)) as (i32) == b'\n' as (i32) {
                                       if byte::diff(
                                        (*b"search \0").as_ptr() as (*mut u8),
                                        7u32,
                                        DATA.s.offset(i as (isize)),
                                    ) == 0 ||
                                           byte::diff(
                                            (*b"search\t\0").as_ptr() as (*mut u8),
                                            7u32,
                                            DATA.s.offset(i as (isize)),
                                        ) == 0 ||
                                           byte::diff(
                                            (*b"domain \0").as_ptr() as (*mut u8),
                                            7u32,
                                            DATA.s.offset(i as (isize)),
                                        ) == 0 ||
                                           byte::diff(
                                            (*b"domain\t\0").as_ptr() as (*mut u8),
                                            7u32,
                                            DATA.s.offset(i as (isize)),
                                        ) == 0
                                    {
                                           current_block = 29;
                                           break;
                                       }
                                       i = j + 1i32;
                                   }
                                   j = j + 1;
                               }
                               if current_block == 11 {
                               } else if StrAlloc::copys(rules, (*b"?:\0").as_ptr()) == 0 {
                                   return -1i32;
                               } else {
                                   i = i + 7i32;
                                   'loop31: loop {
                                       if !(i < j) {
                                           current_block = 32;
                                           break;
                                       }
                                       k = byte::chr(
                                        DATA.s.offset(i as (isize)),
                                        (j - i) as (u32),
                                        b' ' as (i32),
                                    ) as (i32);
                                       k = byte::chr(
                                        DATA.s.offset(i as (isize)),
                                        k as (u32),
                                        b'\t' as (i32),
                                    ) as (i32);
                                       if k == 0 {
                                           i = i + 1;
                                       } else {
                                           if StrAlloc::cats(rules, (*b"+.\0").as_ptr()) == 0 {
                                               current_block = 44;
                                               break;
                                           }
                                           if StrAlloc::catb(
                                            rules,
                                            DATA.s.offset(i as (isize)) as (*const u8),
                                            k as (u32),
                                        ) == 0
                                        {
                                               current_block = 43;
                                               break;
                                           }
                                           i = i + k;
                                       }
                                   }
                                   if current_block == 32 {
                                       if StrAlloc::append(rules, (*b"\0").as_ptr()) == 0 {
                                           return -1i32;
                                       } else if StrAlloc::cats(rules, (*b"*.:\0").as_ptr()) == 0 {
                                           return -1i32;
                                       } else if StrAlloc::append(rules, (*b"\0").as_ptr()) == 0 {
                                           return -1i32;
                                       } else {
                                           return 0i32;
                                       }
                                   } else if current_block == 43 {
                                       return -1i32;
                                   } else {
                                       return -1i32;
                                   }
                               }
                           }
                       }
                       host[0usize] = 0u8;
                       (if libc::gethostname(host.as_mut_ptr() as *mut i8, ::std::mem::size_of::<[u8; 256]>()) ==
                            -1i32
                    {
                            -1i32
                        } else {
                            host[::std::mem::size_of::<[u8; 256]>().wrapping_sub(1usize)] = 0u8;
                            i = string::chr(host.as_mut_ptr() as (*const u8), b'.' as (i32)) as (i32);
                            if host[i as (usize)] != 0 {
                                if StrAlloc::copys(rules, (*b"?:\0").as_ptr()) == 0 {
                                    return -1i32;
                                } else if StrAlloc::cats(
                                rules,
                                host.as_mut_ptr().offset(i as (isize)) as
                                    (*const u8),
                            ) == 0
                            {
                                    return -1i32;
                                } else if StrAlloc::append(rules, (*b"\0").as_ptr()) == 0 {
                                    return -1i32;
                                }
                            }
                            (if StrAlloc::cats(rules, (*b"*.:\0").as_ptr()) == 0 {
                                 -1i32
                             } else if StrAlloc::append(rules, (*b"\0").as_ptr()) == 0 {
                                 -1i32
                             } else {
                                 0i32
                             })
                        })
                   })
              })
         })
    }
}
