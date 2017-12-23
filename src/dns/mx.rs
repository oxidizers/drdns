//! `dns/mx.rs`: MX record packet functionality

use byte;
use stralloc::StrAlloc;
use super::{domain, packet, resolve};
use super::DnsTransmit;
use uint16;

static mut Q: *mut u8 = 0i32 as (*mut u8);

pub unsafe fn packet(out: *mut StrAlloc, buf: *const u8, len: u32) -> i32 {
    let current_block;
    let mut pos: u32;
    let mut header: [u8; 12] = [0u8; 12];
    let mut pref: [u8; 2] = [0u8; 2];
    let mut numanswers: u16 = 0;
    let mut datalen: u16 = 0;
    if StrAlloc::copys(out, (*b"\0").as_ptr()) == 0 {
        -1i32
    } else {
        pos = packet::copy(buf, len, 0u32, header.as_mut_ptr(), 12u32);
        (if pos == 0 {
             -1i32
         } else {
             uint16::unpack_big(
                header.as_mut_ptr().offset(6isize) as (*const u8),
                &mut numanswers as (*mut u16),
            );
             pos = packet::skipname(buf, len, pos);
             (if pos == 0 {
                  -1i32
              } else {
                  pos = pos.wrapping_add(4u32);
                  'loop4: loop {
                      if {
                          let _old = numanswers;
                          numanswers = (numanswers as (i32) - 1) as (u16);
                          _old
                      } == 0
                    {
                          current_block = 5;
                          break;
                      }
                      pos = packet::skipname(buf, len, pos);
                      if pos == 0 {
                          current_block = 22;
                          break;
                      }
                      pos = packet::copy(buf, len, pos, header.as_mut_ptr(), 10u32);
                      if pos == 0 {
                          current_block = 21;
                          break;
                      }
                      uint16::unpack_big(
                        header.as_mut_ptr().offset(8isize) as (*const u8),
                        &mut datalen as (*mut u16),
                    );
                      if byte::diff(
                        header.as_mut_ptr(),
                        2u32,
                        (*b"\0\x0F\0").as_ptr() as (*mut u8),
                    ) == 0
                    {
                          if byte::diff(
                            header.as_mut_ptr().offset(2isize),
                            2u32,
                            (*b"\0\x01\0").as_ptr() as (*mut u8),
                        ) == 0
                        {
                              if packet::copy(buf, len, pos, pref.as_mut_ptr(), 2u32) == 0 {
                                  current_block = 20;
                                  break;
                              }
                              if packet::getname(
                                buf,
                                len,
                                pos.wrapping_add(2u32),
                                &mut Q as (*mut *mut u8),
                            ) == 0
                            {
                                  current_block = 19;
                                  break;
                              }
                              if StrAlloc::catb(out, pref.as_mut_ptr() as (*const u8), 2u32) == 0 {
                                  current_block = 18;
                                  break;
                              }
                              if domain::todot_cat(out, Q as (*const u8)) == 0 {
                                  current_block = 17;
                                  break;
                              }
                              if StrAlloc::append(out, (*b"\0").as_ptr()) == 0 {
                                  current_block = 16;
                                  break;
                              }
                          }
                      }
                      pos = pos.wrapping_add(datalen as (u32));
                  }
                  (if current_block == 5 {
                       0i32
                   } else if current_block == 16 {
                       -1i32
                   } else if current_block == 17 {
                       -1i32
                   } else if current_block == 18 {
                       -1i32
                   } else if current_block == 19 {
                       -1i32
                   } else if current_block == 20 {
                       -1i32
                   } else if current_block == 21 {
                       -1i32
                   } else {
                       -1i32
                   })
              })
         })
    }
}

pub unsafe fn mx(out: *mut StrAlloc, fqdn: *const StrAlloc) -> i32 {
    if domain::fromdot(
        &mut Q as (*mut *mut u8),
        (*fqdn).s as (*const u8),
        (*fqdn).len,
    ) == 0
    {
        -1i32
    } else if resolve::resolve(Q as (*const u8), (*b"\0\x0F\0").as_ptr()) == -1i32 {
        -1i32
    } else if packet(
        out,
        resolve::TX.packet as (*const u8),
        resolve::TX.packetlen,
    ) == -1i32
    {
        -1i32
    } else {
        DnsTransmit::free(&mut resolve::TX as (*mut DnsTransmit));
        domain::free(&mut Q as (*mut *mut u8));
        0i32
    }
}
