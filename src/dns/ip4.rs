use byte;
use case;
use stralloc::StrAlloc;
use string;
use super::{DnsTransmit, domain, packet, rcrw, resolve, sortip};
use uint16;

pub unsafe fn packet(out: *mut StrAlloc, buf: *const u8, len: u32) -> i32 {
    let current_block;
    let mut pos: u32;
    let mut header: [u8; 12] = [0u8; 12];
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
                          current_block = 17;
                          break;
                      }
                      pos = packet::copy(buf, len, pos, header.as_mut_ptr(), 10u32);
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
                        (*b"\0\x01\0").as_ptr() as (*mut u8),
                    ) == 0
                    {
                          if byte::diff(
                            header.as_mut_ptr().offset(2isize),
                            2u32,
                            (*b"\0\x01\0").as_ptr() as (*mut u8),
                        ) == 0
                        {
                              if datalen as (i32) == 4i32 {
                                  if packet::copy(buf, len, pos, header.as_mut_ptr(), 4u32) == 0 {
                                      current_block = 15;
                                      break;
                                  }
                                  if StrAlloc::catb(
                                    out,
                                    header.as_mut_ptr() as (*const u8),
                                    4u32,
                                ) == 0
                                {
                                      current_block = 14;
                                      break;
                                  }
                              }
                          }
                      }
                      pos = pos.wrapping_add(datalen as (u32));
                  }
                  (if current_block == 5 {
                       sortip::sortip((*out).s, (*out).len);
                       0i32
                   } else if current_block == 14 {
                       -1i32
                   } else if current_block == 15 {
                       -1i32
                   } else if current_block == 16 {
                       -1i32
                   } else {
                       -1i32
                   })
              })
         })
    }
}

static mut Q: *mut u8 = 0i32 as (*mut u8);

pub unsafe fn ip4(out: *mut StrAlloc, fqdn: *const StrAlloc) -> i32 {
    let current_block;
    let mut i: u32;
    let mut code: u8;
    let mut ch: u8;
    if StrAlloc::copys(out, (*b"\0").as_ptr()) == 0 {
        -1i32
    } else {
        code = 0u8;
        i = 0u32;
        'loop2: loop {
            if !(i <= (*fqdn).len) {
                current_block = 3;
                break;
            }
            if i < (*fqdn).len {
                ch = *(*fqdn).s.offset(i as (isize));
            } else {
                ch = b'.';
            }
            if !(ch as (i32) == b'[' as (i32) || ch as (i32) == b']' as (i32)) {
                if ch as (i32) == b'.' as (i32) {
                    if StrAlloc::append(out, &mut code as (*mut u8) as (*const u8)) == 0 {
                        current_block = 20;
                        break;
                    }
                    code = 0u8;
                } else {
                    if !(ch as (i32) >= b'0' as (i32) && (ch as (i32) <= b'9' as (i32))) {
                        current_block = 10;
                        break;
                    }
                    code = (code as (i32) * 10i32) as (u8);
                    code = (code as (i32) + (ch as (i32) - b'0' as (i32))) as (u8);
                }
            }
            i = i.wrapping_add(1u32);
        }
        (if current_block == 3 {
             (*out).len = (*out).len & !3i32 as (u32);
             0i32
         } else if current_block == 10 {
             (if domain::fromdot(
                &mut Q as (*mut *mut u8),
                (*fqdn).s as (*const u8),
                (*fqdn).len,
            ) == 0
            {
                  -1i32
              } else if resolve::resolve(Q as (*const u8), (*b"\0\x01\0").as_ptr()) == -1i32 {
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
              })
         } else {
             -1i32
         })
    }
}

unsafe fn doit(work: *mut StrAlloc, mut rule: *const u8) -> i32 {
    let ch: u8;
    let colon: u32;
    let prefixlen: u32;
    ch = *{
        let _old = rule;
        rule = rule.offset(1isize);
        _old
    };
    if ch as (i32) != b'?' as (i32) && (ch as (i32) != b'=' as (i32)) &&
        (ch as (i32) != b'*' as (i32)) && (ch as (i32) != b'-' as (i32))
    {
        1i32
    } else {
        colon = string::chr(rule, b':' as (i32));
        (if *rule.offset(colon as (isize)) == 0 {
             1i32
         } else if (*work).len < colon {
             1i32
         } else {
             prefixlen = (*work).len.wrapping_sub(colon);
             (if ch as (i32) == b'=' as (i32) && (prefixlen != 0) {
                  1i32
              } else if case::diffb(
                rule,
                colon,
                (*work).s.offset(prefixlen as (isize)) as (*const u8),
            ) != 0
            {
                  1i32
              } else {
                  if ch as (i32) == b'?' as (i32) {
                      if byte::chr((*work).s, prefixlen, b'.' as (i32)) < prefixlen {
                          return 1i32;
                      } else if byte::chr((*work).s, prefixlen, b'[' as (i32)) < prefixlen {
                          return 1i32;
                      } else if byte::chr((*work).s, prefixlen, b']' as (i32)) < prefixlen {
                          return 1i32;
                      }
                  }
                  (*work).len = prefixlen;
                  if ch as (i32) == b'-' as (i32) {
                      (*work).len = 0u32;
                  }
                  StrAlloc::cats(work, rule.offset(colon as (isize)).offset(1isize))
              })
         })
    }
}

pub unsafe fn qualify_rules(
    out: *mut StrAlloc,
    fqdn: *mut StrAlloc,
    input: *const StrAlloc,
    rules: *const StrAlloc,
) -> i32 {
    let mut current_block;
    let mut i: u32;
    let mut j: u32;
    let plus: u32;
    let fqdnlen: u32;
    if StrAlloc::copy(fqdn, input) == 0 {
        -1i32
    } else {
        j = {
            i = 0u32;
            i
        };
        'loop2: loop {
            if !(j < (*rules).len) {
                current_block = 3;
                break;
            }
            if *(*rules).s.offset(j as (isize)) == 0 {
                if doit(fqdn, (*rules).s.offset(i as (isize)) as (*const u8)) == 0 {
                    current_block = 17;
                    break;
                }
                i = j.wrapping_add(1u32);
            }
            j = j.wrapping_add(1u32);
        }
        (if current_block == 3 {
             fqdnlen = (*fqdn).len;
             plus = byte::chr((*fqdn).s, fqdnlen, b'+' as (i32));
             (if plus >= fqdnlen {
                  ip4(out, fqdn as (*const StrAlloc))
              } else {
                  i = plus.wrapping_add(1u32);
                  'loop5: loop {
                      j = byte::chr(
                        (*fqdn).s.offset(i as (isize)),
                        fqdnlen.wrapping_sub(i),
                        b'+' as (i32),
                    );
                      byte::copy(
                        (*fqdn).s.offset(plus as (isize)),
                        j,
                        (*fqdn).s.offset(i as (isize)),
                    );
                      (*fqdn).len = plus.wrapping_add(j);
                      if ip4(out, fqdn as (*const StrAlloc)) == -1i32 {
                          current_block = 11;
                          break;
                      }
                      if (*out).len != 0 {
                          current_block = 10;
                          break;
                      }
                      i = i.wrapping_add(j);
                      if i >= fqdnlen {
                          current_block = 9;
                          break;
                      }
                      i = i.wrapping_add(1u32);
                  }
                  (if current_block == 9 {
                       0i32
                   } else if current_block == 10 {
                       0i32
                   } else {
                       -1i32
                   })
              })
         } else {
             -1i32
         })
    }
}

static mut RULES: StrAlloc = StrAlloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

pub unsafe fn qualify(out: *mut StrAlloc, fqdn: *mut StrAlloc, input: *const StrAlloc) -> i32 {
    if rcrw::resolvconfrewrite(&mut RULES as (*mut StrAlloc)) == -1i32 {
        -1i32
    } else {
        qualify_rules(
            out,
            fqdn,
            input,
            &mut RULES as (*mut StrAlloc) as (*const StrAlloc),
        )
    }
}
