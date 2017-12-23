//! `dns/txt.rs`: TXT record packet functionality

use byte;
use stralloc::StrAlloc;
use super::{domain, packet, resolve};
use super::DnsTransmit;
use uint16;

pub unsafe fn packet(out: *mut StrAlloc, buf: *const u8, len: u32) -> i32 {
    let current_block;
    let mut pos: u32;
    let mut header: [u8; 12] = [0u8; 12];
    let mut numanswers: u16 = 0;
    let mut datalen: u16 = 0;
    let mut ch: u8;
    let mut txtlen: u32;
    let mut i: i32;
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
                          current_block = 25;
                          break;
                      }
                      pos = packet::copy(buf, len, pos, header.as_mut_ptr(), 10u32);
                      if pos == 0 {
                          current_block = 24;
                          break;
                      }
                      uint16::unpack_big(
                        header.as_mut_ptr().offset(8isize) as (*const u8),
                        &mut datalen as (*mut u16),
                    );
                      if byte::diff(
                        header.as_mut_ptr(),
                        2u32,
                        (*b"\0\x10\0").as_ptr() as (*mut u8),
                    ) == 0
                    {
                          if byte::diff(
                            header.as_mut_ptr().offset(2isize),
                            2u32,
                            (*b"\0\x01\0").as_ptr() as (*mut u8),
                        ) == 0
                        {
                              if pos.wrapping_add(datalen as (u32)) > len {
                                  current_block = 23;
                                  break;
                              }
                              txtlen = 0u32;
                              i = 0i32;
                              'loop12: loop {
                                  if !(i < datalen as (i32)) {
                                      break;
                                  }
                                  ch = *buf.offset(pos.wrapping_add(i as (u32)) as (isize));
                                  if txtlen == 0 {
                                      txtlen = ch as (u32);
                                  } else {
                                      txtlen = txtlen.wrapping_sub(1u32);
                                      if ch as (i32) < 32i32 {
                                          ch = b'?';
                                      }
                                      if ch as (i32) > 126i32 {
                                          ch = b'?';
                                      }
                                      if StrAlloc::append(
                                        out,
                                        &mut ch as (*mut u8) as (*const u8),
                                    ) == 0
                                    {
                                          current_block = 20;
                                          break 'loop4;
                                      }
                                  }
                                  i = i + 1;
                              }
                          }
                      }
                      pos = pos.wrapping_add(datalen as (u32));
                  }
                  (if current_block == 5 {
                       0i32
                   } else if current_block == 20 {
                       -1i32
                   } else if current_block == 23 {
                       -1i32
                   } else if current_block == 24 {
                       -1i32
                   } else {
                       -1i32
                   })
              })
         })
    }
}

static mut Q: *mut u8 = 0i32 as (*mut u8);

pub unsafe fn txt(out: *mut StrAlloc, fqdn: *const StrAlloc) -> i32 {
    if domain::fromdot(
        &mut Q as (*mut *mut u8),
        (*fqdn).s as (*const u8),
        (*fqdn).len,
    ) == 0
    {
        -1i32
    } else if resolve::resolve(Q as (*const u8), (*b"\0\x10\0").as_ptr()) == -1i32 {
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
