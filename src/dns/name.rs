//! `dns/name.rs`: DNS name facilities

use byte;
use stralloc::StrAlloc;
use dns;
use super::DnsTransmit;
use uint16;
use ulong;

static mut Q: *mut u8 = 0i32 as (*mut u8);

pub unsafe fn packet(out: *mut StrAlloc, buf: *const u8, len: u32) -> i32 {
    let current_block;
    let mut pos: u32;
    let mut header: [u8; 12] = [0u8; 12];
    let mut numanswers: u16 = 0;
    let mut datalen: u16 = 0;
    if StrAlloc::copys(out, (*b"\0").as_ptr()) == 0 {
        -1i32
    } else {
        pos = dns::packet::copy(buf, len, 0u32, header.as_mut_ptr(), 12u32);
        (if pos == 0 {
             -1i32
         } else {
             uint16::unpack_big(
                header.as_mut_ptr().offset(6isize) as (*const u8),
                &mut numanswers as (*mut u16),
            );
             pos = dns::packet::skipname(buf, len, pos);
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
                      pos = dns::packet::skipname(buf, len, pos);
                      if pos == 0 {
                          current_block = 17;
                          break;
                      }
                      pos = dns::packet::copy(buf, len, pos, header.as_mut_ptr(), 10u32);
                      if pos == 0 {
                          current_block = 16;
                          break;
                      }
                      uint16::unpack_big(
                        header.as_mut_ptr().offset(8isize) as (*const u8),
                        &mut datalen as (*mut u16),
                    );
                      if byte::diff(
                        header.as_mut_ptr(),
                        2u32,
                        (*b"\0\x0C\0").as_ptr() as (*mut u8),
                    ) == 0
                    {
                          if byte::diff(
                            header.as_mut_ptr().offset(2isize),
                            2u32,
                            (*b"\0\x01\0").as_ptr() as (*mut u8),
                        ) == 0
                        {
                              current_block = 11;
                              break;
                          }
                      }
                      pos = pos.wrapping_add(datalen as (u32));
                  }
                  (if current_block == 5 {
                       0i32
                   } else if current_block == 11 {
                       (if dns::packet::getname(buf, len, pos, &mut Q as (*mut *mut u8)) == 0 {
                            -1i32
                        } else if dns::domain::todot_cat(out, Q as (*const u8)) == 0 {
                            -1i32
                        } else {
                            0i32
                        })
                   } else if current_block == 16 {
                       -1i32
                   } else {
                       -1i32
                   })
              })
         })
    }
}

pub unsafe fn name4(out: *mut StrAlloc, ip: *const u8) -> i32 {
    let mut name: [u8; 31] = [0u8; 31];
    domain(name.as_mut_ptr(), ip);
    if dns::resolve::resolve(name.as_mut_ptr() as (*const u8), (*b"\0\x0C\0").as_ptr()) == -1i32 {
        -1i32
    } else if packet(
        out,
        dns::resolve::TX.packet as (*const u8),
        dns::resolve::TX.packetlen,
    ) == -1i32
    {
        -1i32
    } else {
        DnsTransmit::free(&mut dns::resolve::TX as (*mut DnsTransmit));
        dns::domain::free(&mut Q as (*mut *mut u8));
        0i32
    }
}

pub unsafe fn domain(name: *mut u8, ip: *const u8) {
    let mut namelen: u32;
    let mut i: u32;
    namelen = 0u32;
    i = ulong::fmt(
        name.offset(namelen as (isize)).offset(1isize),
        *ip.offset(3isize) as (usize),
    );
    *name.offset({
        let _old = namelen;
        namelen = namelen.wrapping_add(1u32);
        _old
    } as (isize)) = i as (u8);
    namelen = namelen.wrapping_add(i);
    i = ulong::fmt(
        name.offset(namelen as (isize)).offset(1isize),
        *ip.offset(2isize) as (usize),
    );
    *name.offset({
        let _old = namelen;
        namelen = namelen.wrapping_add(1u32);
        _old
    } as (isize)) = i as (u8);
    namelen = namelen.wrapping_add(i);
    i = ulong::fmt(
        name.offset(namelen as (isize)).offset(1isize),
        *ip.offset(1isize) as (usize),
    );
    *name.offset({
        let _old = namelen;
        namelen = namelen.wrapping_add(1u32);
        _old
    } as (isize)) = i as (u8);
    namelen = namelen.wrapping_add(i);
    i = ulong::fmt(
        name.offset(namelen as (isize)).offset(1isize),
        *ip.offset(0isize) as (usize),
    );
    *name.offset({
        let _old = namelen;
        namelen = namelen.wrapping_add(1u32);
        _old
    } as (isize)) = i as (u8);
    namelen = namelen.wrapping_add(i);
    byte::copy(
        name.offset(namelen as (isize)),
        14u32,
        (*b"\x07in-addr\x04arpa\0\0").as_ptr() as (*mut u8),
    );
}
