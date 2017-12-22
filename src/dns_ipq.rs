use byte;
use case;
use stralloc::StrAlloc;
use string;

extern "C" {
    fn dns_ip4(arg1: *mut StrAlloc, arg2: *const StrAlloc) -> i32;
    fn dns_resolvconfrewrite(arg1: *mut StrAlloc) -> i32;
}

unsafe extern "C" fn doit(mut work: *mut StrAlloc, mut rule: *const u8) -> i32 {
    let mut ch: u8;
    let mut colon: u32;
    let mut prefixlen: u32;
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

#[no_mangle]
pub unsafe extern "C" fn dns_ip4_qualify_rules(
    mut out: *mut StrAlloc,
    mut fqdn: *mut StrAlloc,
    mut in_: *const StrAlloc,
    mut rules: *const StrAlloc,
) -> i32 {
    let mut _currentBlock;
    let mut i: u32;
    let mut j: u32;
    let mut plus: u32;
    let mut fqdnlen: u32;
    if StrAlloc::copy(fqdn, in_) == 0 {
        -1i32
    } else {
        j = {
            i = 0u32;
            i
        };
        'loop2: loop {
            if !(j < (*rules).len) {
                _currentBlock = 3;
                break;
            }
            if *(*rules).s.offset(j as (isize)) == 0 {
                if doit(fqdn, (*rules).s.offset(i as (isize)) as (*const u8)) == 0 {
                    _currentBlock = 17;
                    break;
                }
                i = j.wrapping_add(1u32);
            }
            j = j.wrapping_add(1u32);
        }
        (if _currentBlock == 3 {
             fqdnlen = (*fqdn).len;
             plus = byte::chr((*fqdn).s, fqdnlen, b'+' as (i32));
             (if plus >= fqdnlen {
                  dns_ip4(out, fqdn as (*const StrAlloc))
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
                      if dns_ip4(out, fqdn as (*const StrAlloc)) == -1i32 {
                          _currentBlock = 11;
                          break;
                      }
                      if (*out).len != 0 {
                          _currentBlock = 10;
                          break;
                      }
                      i = i.wrapping_add(j);
                      if i >= fqdnlen {
                          _currentBlock = 9;
                          break;
                      }
                      i = i.wrapping_add(1u32);
                  }
                  (if _currentBlock == 9 {
                       0i32
                   } else if _currentBlock == 10 {
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

#[no_mangle]
pub unsafe extern "C" fn dns_ip4_qualify(
    mut out: *mut StrAlloc,
    mut fqdn: *mut StrAlloc,
    mut in_: *const StrAlloc,
) -> i32 {
    static mut rules: StrAlloc = StrAlloc {
        s: 0 as (*mut u8),
        len: 0u32,
        a: 0u32,
    };
    if dns_resolvconfrewrite(&mut rules as (*mut StrAlloc)) == -1i32 {
        -1i32
    } else {
        dns_ip4_qualify_rules(
            out,
            fqdn,
            in_,
            &mut rules as (*mut StrAlloc) as (*const StrAlloc),
        )
    }
}
