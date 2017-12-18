use byte;

extern "C" {
    fn dns_domain_equal(arg1: *const u8, arg2: *const u8) -> i32;
    fn dns_domain_todot_cat(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn dns_packet_copy(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut u8, arg5: u32) -> u32;
    fn dns_packet_getname(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut *mut u8) -> u32;
    static mut errno: i32;
    static mut error_proto: i32;
    fn stralloc_catb(arg1: *mut stralloc, arg2: *const u8, arg3: u32) -> i32;
    fn stralloc_cats(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn stralloc_catulong0(arg1: *mut stralloc, arg2: usize, arg3: u32) -> i32;
    fn stralloc_copys(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn uint16_unpack_big(arg1: *const u8, arg2: *mut u16);
    fn uint32_unpack_big(arg1: *const u8, arg2: *mut u32);
}

static mut d: *mut u8 = 0 as (*mut u8);

#[derive(Copy)]
#[repr(C)]
pub struct stralloc {
    pub s: *mut u8,
    pub len: u32,
    pub a: u32,
}

impl Clone for stralloc {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn printrecord_cat(
    mut out: *mut stralloc,
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
             uint16_unpack_big(
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
              } else if stralloc_cats(out, (*b" \0").as_ptr()) == 0 {
                  0u32
              } else {
                  uint32_unpack_big(
                    misc.as_mut_ptr().offset(4isize) as (*const u8),
                    &mut u32 as (*mut u32),
                );
                  (if stralloc_catulong0(out, u32 as (usize), 0u32) == 0 {
                       0u32
                   } else if byte::diff(
                    misc.as_mut_ptr().offset(2isize),
                    2u32,
                    (*b"\0\x01\0").as_ptr() as (*mut u8),
                ) != 0
                {
                       (if stralloc_cats(out, (*b" weird class\n\0").as_ptr()) == 0 {
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
                           } else if stralloc_cats(out, x) == 0 {
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
                           if stralloc_cats(out, (*b" MX \0").as_ptr()) == 0 {
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
                                       uint16_unpack_big(
                                        misc.as_mut_ptr() as (*const u8),
                                        &mut u16 as (*mut u16),
                                    );
                                       if stralloc_catulong0(out, u16 as (usize), 0u32) == 0 {
                                           return 0u32;
                                       } else if stralloc_cats(out, (*b" \0").as_ptr()) == 0 {
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
                           if stralloc_cats(out, (*b" SOA \0").as_ptr()) == 0 {
                               return 0u32;
                           } else {
                               pos = dns_packet_getname(buf, len, pos, &mut d as (*mut *mut u8));
                               if pos == 0 {
                                   return 0u32;
                               } else if dns_domain_todot_cat(out, d as (*const u8)) == 0 {
                                   return 0u32;
                               } else if stralloc_cats(out, (*b" \0").as_ptr()) == 0 {
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
                                               if stralloc_cats(out, (*b" \0").as_ptr()) == 0 {
                                                   _currentBlock = 60;
                                                   break;
                                               }
                                               uint32_unpack_big(
                                                misc.as_mut_ptr().offset((4i32 * i) as (isize)) as
                                                    (*const u8),
                                                &mut u32 as (*mut u32),
                                            );
                                               if stralloc_catulong0(out, u32 as (usize), 0u32) ==
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
                               errno = error_proto;
                               return 0u32;
                           } else if stralloc_cats(out, (*b" A \0").as_ptr()) == 0 {
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
                                           if stralloc_cats(out, (*b".\0").as_ptr()) == 0 {
                                               _currentBlock = 43;
                                               break;
                                           }
                                       }
                                       if stralloc_catulong0(out, ch as (usize), 0u32) == 0 {
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
                       } else if stralloc_cats(out, (*b" \0").as_ptr()) == 0 {
                           return 0u32;
                       } else {
                           uint16_unpack_big(
                            misc.as_mut_ptr() as (*const u8),
                            &mut u16 as (*mut u16),
                        );
                           if stralloc_catulong0(out, u16 as (usize), 0u32) == 0 {
                               return 0u32;
                           } else if stralloc_cats(out, (*b" \0").as_ptr()) == 0 {
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
                                       if stralloc_catb(
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
                                       if stralloc_catb(
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
                       (if stralloc_cats(out, (*b"\n\0").as_ptr()) == 0 {
                            0u32
                        } else if pos != newpos {
                            errno = error_proto;
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
    mut out: *mut stralloc,
    mut buf: *const u8,
    mut len: u32,
    mut pos: u32,
    mut q: *const u8,
    mut qtype: *const u8,
) -> u32 {
    if stralloc_copys(out, (*b"\0").as_ptr()) == 0 {
        0u32
    } else {
        printrecord_cat(out, buf, len, pos, q, qtype)
    }
}
