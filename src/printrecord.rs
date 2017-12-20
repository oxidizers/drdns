use byte;
use errno::{self, Errno};
use libc;
use uint16;
use uint32;

extern "C" {
    fn dns_domain_equal(arg1: *const u8, arg2: *const u8) -> i32;
    fn dns_domain_todot_cat(arg1: *mut StrAlloc, arg2: *const u8) -> i32;
    fn dns_packet_copy(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut u8, arg5: u32) -> u32;
    fn dns_packet_getname(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut *mut u8) -> u32;
}

static mut d: *mut u8 = 0 as (*mut u8);

#[no_mangle]
pub unsafe extern "C" fn printrecord_cat(
    mut out: *mut StrAlloc,
    mut buf: *const u8,
    mut len: u32,
    mut pos: u32,
    mut q: *const u8,
    mut qtype: *const u8,
) -> u32 {
    let mut _currentBlock;
    let mut x: *const u8;
    let mut misc: [u8; 20];
    let mut datalen: u16;
    let mut u16: u16;
    let mut u32: u32;
    let mut newpos: u32;
    let mut i: i32;
    let mut ch: u8;
    pos = dns_packet_getname(buf, len, pos, &mut d as (*mut *mut u8));
    if pos == 0 {
        0u32
    } else {
        pos = dns_packet_copy(buf, len, pos, misc.as_mut_ptr(), 10u32);
        (if pos == 0 {
             0u32
         } else {
             uint16::unpack_big(
                misc.as_mut_ptr().offset(8isize) as (*const u8),
                &mut datalen as (*mut u16),
            );
             newpos = pos.wrapping_add(datalen as (u32));
             if !q.is_null() {
                 if dns_domain_equal(d as (*const u8), q) == 0 {
                     return newpos;
                 } else if byte::diff(qtype as (*mut u8), 2u32, misc.as_mut_ptr()) != 0 &&
                            (byte::diff(
                        qtype as (*mut u8),
                        2u32,
                        (*b"\0\xFF\0").as_ptr() as (*mut u8),
                    ) != 0)
                {
                     return newpos;
                 }
             }
             (if dns_domain_todot_cat(out, d as (*const u8)) == 0 {
                  0u32
              } else if StrAlloc::cats(out, (*b" \0").as_ptr()) == 0 {
                  0u32
              } else {
                  uint32::unpack_big(
                    misc.as_mut_ptr().offset(4isize) as (*const u8),
                    &mut u32 as (*mut u32),
                );
                  (if StrAlloc::catulong0(out, u32 as (usize), 0u32) == 0 {
                       0u32
                   } else if byte::diff(
                    misc.as_mut_ptr().offset(2isize),
                    2u32,
                    (*b"\0\x01\0").as_ptr() as (*mut u8),
                ) != 0
                {
                       (if StrAlloc::cats(out, (*b" weird class\n\0").as_ptr()) == 0 {
                            0u32
                        } else {
                            newpos
                        })
                   } else {
                       x = 0i32 as (*const u8);
                       if byte::diff(
                        misc.as_mut_ptr(),
                        2u32,
                        (*b"\0\x02\0").as_ptr() as (*mut u8),
                    ) == 0
                    {
                           x = (*b" NS \0").as_ptr();
                       }
                       if byte::diff(
                        misc.as_mut_ptr(),
                        2u32,
                        (*b"\0\x0C\0").as_ptr() as (*mut u8),
                    ) == 0
                    {
                           x = (*b" PTR \0").as_ptr();
                       }
                       if byte::diff(
                        misc.as_mut_ptr(),
                        2u32,
                        (*b"\0\x05\0").as_ptr() as (*mut u8),
                    ) == 0
                    {
                           x = (*b" CNAME \0").as_ptr();
                       }
                       if !x.is_null() {
                           pos = dns_packet_getname(buf, len, pos, &mut d as (*mut *mut u8));
                           if pos == 0 {
                               return 0u32;
                           } else if StrAlloc::cats(out, x) == 0 {
                               return 0u32;
                           } else if dns_domain_todot_cat(out, d as (*const u8)) == 0 {
                               return 0u32;
                           }
                       } else if byte::diff(
                        misc.as_mut_ptr(),
                        2u32,
                        (*b"\0\x0F\0").as_ptr() as (*mut u8),
                    ) == 0
                    {
                           if StrAlloc::cats(out, (*b" MX \0").as_ptr()) == 0 {
                               return 0u32;
                           } else {
                               pos = dns_packet_copy(buf, len, pos, misc.as_mut_ptr(), 2u32);
                               if pos == 0 {
                                   return 0u32;
                               } else {
                                   pos =
                                       dns_packet_getname(buf, len, pos, &mut d as (*mut *mut u8));
                                   if pos == 0 {
                                       return 0u32;
                                   } else {
                                       uint16::unpack_big(
                                        misc.as_mut_ptr() as (*const u8),
                                        &mut u16 as (*mut u16),
                                    );
                                       if StrAlloc::catulong0(out, u16 as (usize), 0u32) == 0 {
                                           return 0u32;
                                       } else if StrAlloc::cats(out, (*b" \0").as_ptr()) == 0 {
                                           return 0u32;
                                       } else if dns_domain_todot_cat(out, d as (*const u8)) == 0 {
                                           return 0u32;
                                       }
                                   }
                               }
                           }
                       } else if byte::diff(
                        misc.as_mut_ptr(),
                        2u32,
                        (*b"\0\x06\0").as_ptr() as (*mut u8),
                    ) == 0
                    {
                           if StrAlloc::cats(out, (*b" SOA \0").as_ptr()) == 0 {
                               return 0u32;
                           } else {
                               pos = dns_packet_getname(buf, len, pos, &mut d as (*mut *mut u8));
                               if pos == 0 {
                                   return 0u32;
                               } else if dns_domain_todot_cat(out, d as (*const u8)) == 0 {
                                   return 0u32;
                               } else if StrAlloc::cats(out, (*b" \0").as_ptr()) == 0 {
                                   return 0u32;
                               } else {
                                   pos =
                                       dns_packet_getname(buf, len, pos, &mut d as (*mut *mut u8));
                                   if pos == 0 {
                                       return 0u32;
                                   } else if dns_domain_todot_cat(out, d as (*const u8)) == 0 {
                                       return 0u32;
                                   } else {
                                       pos =
                                           dns_packet_copy(buf, len, pos, misc.as_mut_ptr(), 20u32);
                                       if pos == 0 {
                                           return 0u32;
                                       } else {
                                           i = 0i32;
                                           'loop55: loop {
                                               if !(i < 5i32) {
                                                   _currentBlock = 83;
                                                   break;
                                               }
                                               if StrAlloc::cats(out, (*b" \0").as_ptr()) == 0 {
                                                   _currentBlock = 60;
                                                   break;
                                               }
                                               uint32::unpack_big(
                                                misc.as_mut_ptr().offset((4i32 * i) as (isize)) as
                                                    (*const u8),
                                                &mut u32 as (*mut u32),
                                            );
                                               if StrAlloc::catulong0(out, u32 as (usize), 0u32) ==
                                                   0
                                            {
                                                   _currentBlock = 59;
                                                   break;
                                               }
                                               i = i + 1;
                                           }
                                           if _currentBlock == 83 {
                                           } else if _currentBlock == 59 {
                                               return 0u32;
                                           } else {
                                               return 0u32;
                                           }
                                       }
                                   }
                               }
                           }
                       } else if byte::diff(
                        misc.as_mut_ptr(),
                        2u32,
                        (*b"\0\x01\0").as_ptr() as (*mut u8),
                    ) == 0
                    {
                           if datalen as (i32) != 4i32 {
                               errno::set_errno(Errno(libc::EPROTO));
                               return 0u32;
                           } else if StrAlloc::cats(out, (*b" A \0").as_ptr()) == 0 {
                               return 0u32;
                           } else {
                               pos = dns_packet_copy(buf, len, pos, misc.as_mut_ptr(), 4u32);
                               if pos == 0 {
                                   return 0u32;
                               } else {
                                   i = 0i32;
                                   'loop37: loop {
                                       if !(i < 4i32) {
                                           _currentBlock = 83;
                                           break;
                                       }
                                       ch = misc[i as (usize)];
                                       if i != 0 {
                                           if StrAlloc::cats(out, (*b".\0").as_ptr()) == 0 {
                                               _currentBlock = 43;
                                               break;
                                           }
                                       }
                                       if StrAlloc::catulong0(out, ch as (usize), 0u32) == 0 {
                                           _currentBlock = 42;
                                           break;
                                       }
                                       i = i + 1;
                                   }
                                   if _currentBlock == 83 {
                                   } else if _currentBlock == 42 {
                                       return 0u32;
                                   } else {
                                       return 0u32;
                                   }
                               }
                           }
                       } else if StrAlloc::cats(out, (*b" \0").as_ptr()) == 0 {
                           return 0u32;
                       } else {
                           uint16::unpack_big(
                            misc.as_mut_ptr() as (*const u8),
                            &mut u16 as (*mut u16),
                        );
                           if StrAlloc::catulong0(out, u16 as (usize), 0u32) == 0 {
                               return 0u32;
                           } else if StrAlloc::cats(out, (*b" \0").as_ptr()) == 0 {
                               return 0u32;
                           } else {
                               'loop22: loop {
                                   if {
                                       let _old = datalen;
                                       datalen = (datalen as (i32) - 1) as (u16);
                                       _old
                                   } == 0
                                {
                                       _currentBlock = 83;
                                       break;
                                   }
                                   pos = dns_packet_copy(buf, len, pos, misc.as_mut_ptr(), 1u32);
                                   if pos == 0 {
                                       _currentBlock = 29;
                                       break;
                                   }
                                   if misc[0usize] as (i32) >= 33i32 &&
                                       (misc[0usize] as (i32) <= 126i32) &&
                                       (misc[0usize] as (i32) != b'\\' as (i32))
                                {
                                       if StrAlloc::catb(
                                        out,
                                        misc.as_mut_ptr() as (*const u8),
                                        1u32,
                                    ) == 0
                                    {
                                           _currentBlock = 28;
                                           break;
                                       }
                                   } else {
                                       ch = misc[0usize];
                                       misc[3usize] = (b'0' as (i32) + (7i32 & ch as (i32))) as
                                           (u8);
                                       ch = (ch as (i32) >> 3i32) as (u8);
                                       misc[2usize] = (b'0' as (i32) + (7i32 & ch as (i32))) as
                                           (u8);
                                       ch = (ch as (i32) >> 3i32) as (u8);
                                       misc[1usize] = (b'0' as (i32) + (7i32 & ch as (i32))) as
                                           (u8);
                                       misc[0usize] = b'\\';
                                       if StrAlloc::catb(
                                        out,
                                        misc.as_mut_ptr() as (*const u8),
                                        4u32,
                                    ) == 0
                                    {
                                           _currentBlock = 26;
                                           break;
                                       }
                                   }
                               }
                               if _currentBlock == 83 {
                               } else if _currentBlock == 26 {
                                   return 0u32;
                               } else if _currentBlock == 28 {
                                   return 0u32;
                               } else {
                                   return 0u32;
                               }
                           }
                       }
                       (if StrAlloc::cats(out, (*b"\n\0").as_ptr()) == 0 {
                            0u32
                        } else if pos != newpos {
                            errno::set_errno(Errno(libc::EPROTO));
                            0u32
                        } else {
                            newpos
                        })
                   })
              })
         })
    }
}

#[no_mangle]
pub unsafe extern "C" fn printrecord(
    mut out: *mut StrAlloc,
    mut buf: *const u8,
    mut len: u32,
    mut pos: u32,
    mut q: *const u8,
    mut qtype: *const u8,
) -> u32 {
    if StrAlloc::copys(out, (*b"\0").as_ptr()) == 0 {
        0u32
    } else {
        printrecord_cat(out, buf, len, pos, q, qtype)
    }
}
