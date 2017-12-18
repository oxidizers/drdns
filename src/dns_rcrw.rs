use byte;

extern "C" {
    fn env_get(arg1: *const u8) -> *mut u8;
    fn gethostname(arg1: *mut u8, arg2: usize) -> i32;
    fn openreadclose(arg1: *const u8, arg2: *mut stralloc, arg3: u32) -> i32;
    fn str_chr(arg1: *const u8, arg2: i32) -> u32;
    fn stralloc_append(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn stralloc_catb(arg1: *mut stralloc, arg2: *const u8, arg3: u32) -> i32;
    fn stralloc_cats(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn stralloc_copy(arg1: *mut stralloc, arg2: *const stralloc) -> i32;
    fn stralloc_copys(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn taia_add(arg1: *mut taia, arg2: *const taia, arg3: *const taia);
    fn taia_less(arg1: *const taia, arg2: *const taia) -> i32;
    fn taia_now(arg1: *mut taia);
    fn taia_uint(arg1: *mut taia, arg2: u32);
}

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

static mut data: stralloc = stralloc {
    s: 0i32 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

static mut ok: i32 = 0i32;

static mut uses: u32 = 0u32;

#[derive(Copy)]
#[repr(C)]
pub struct tai {
    pub x: usize,
}

impl Clone for tai {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct taia {
    pub sec: tai,
    pub nano: usize,
    pub atto: usize,
}

impl Clone for taia {
    fn clone(&self) -> Self {
        *self
    }
}

static mut deadline: taia = taia {
    sec: tai { x: 0usize },
    nano: 0usize,
    atto: 0usize,
};

static mut rules: stralloc = stralloc {
    s: 0i32 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

unsafe extern "C" fn init(mut rules: *mut stralloc) -> i32 {
    let mut _currentBlock;
    let mut host: [u8; 256];
    let mut x: *const u8;
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    if stralloc_copys(rules, (*b"\0").as_ptr()) == 0 {
        -1i32
    } else {
        x = env_get((*b"DNSREWRITEFILE\0").as_ptr()) as (*const u8);
        if x.is_null() {
            x = (*b"/etc/dnsrewrite\0").as_ptr();
        }
        i = openreadclose(x, &mut data as (*mut stralloc), 64u32);
        (if i == -1i32 {
             -1i32
         } else if i != 0 {
             (if stralloc_append(&mut data as (*mut stralloc), (*b"\n\0").as_ptr()) == 0 {
                  -1i32
              } else {
                  i = 0i32;
                  j = 0i32;
                  'loop73: loop {
                      if !(j as (u32) < data.len) {
                          _currentBlock = 74;
                          break;
                      }
                      if *data.s.offset(j as (isize)) as (i32) == b'\n' as (i32) {
                          if stralloc_catb(
                            rules,
                            data.s.offset(i as (isize)) as (*const u8),
                            (j - i) as (u32),
                        ) == 0
                        {
                              _currentBlock = 86;
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
                          if stralloc_append(rules, (*b"\0").as_ptr()) == 0 {
                              _currentBlock = 85;
                              break;
                          }
                          i = j + 1i32;
                      }
                      j = j + 1;
                  }
                  (if _currentBlock == 74 {
                       0i32
                   } else if _currentBlock == 85 {
                       -1i32
                   } else {
                       -1i32
                   })
              })
         } else {
             x = env_get((*b"LOCALDOMAIN\0").as_ptr()) as (*const u8);
             (if !x.is_null() {
                  (if stralloc_copys(&mut data as (*mut stralloc), x) == 0 {
                       -1i32
                   } else if stralloc_append(&mut data as (*mut stralloc), (*b" \0").as_ptr()) ==
                              0
                {
                       -1i32
                   } else if stralloc_copys(rules, (*b"?:\0").as_ptr()) == 0 {
                       -1i32
                   } else {
                       i = 0i32;
                       j = 0i32;
                       'loop53: loop {
                           if !(j as (u32) < data.len) {
                               _currentBlock = 54;
                               break;
                           }
                           if *data.s.offset(j as (isize)) as (i32) == b' ' as (i32) {
                               if stralloc_cats(rules, (*b"+.\0").as_ptr()) == 0 {
                                   _currentBlock = 67;
                                   break;
                               }
                               if stralloc_catb(
                                rules,
                                data.s.offset(i as (isize)) as (*const u8),
                                (j - i) as (u32),
                            ) == 0
                            {
                                   _currentBlock = 66;
                                   break;
                               }
                               i = j + 1i32;
                           }
                           j = j + 1;
                       }
                       (if _currentBlock == 54 {
                            (if stralloc_append(rules, (*b"\0").as_ptr()) == 0 {
                                 -1i32
                             } else if stralloc_cats(rules, (*b"*.:\0").as_ptr()) == 0 {
                                 -1i32
                             } else if stralloc_append(rules, (*b"\0").as_ptr()) == 0 {
                                 -1i32
                             } else {
                                 0i32
                             })
                        } else if _currentBlock == 66 {
                            -1i32
                        } else {
                            -1i32
                        })
                   })
              } else {
                  i = openreadclose(
                    (*b"/etc/resolv.conf\0").as_ptr(),
                    &mut data as (*mut stralloc),
                    64u32,
                );
                  (if i == -1i32 {
                       -1i32
                   } else {
                       if i != 0 {
                           if stralloc_append(
                            &mut data as (*mut stralloc),
                            (*b"\n\0").as_ptr(),
                        ) == 0
                        {
                               return -1i32;
                           } else {
                               i = 0i32;
                               j = 0i32;
                               'loop10: loop {
                                   if !(j as (u32) < data.len) {
                                       _currentBlock = 11;
                                       break;
                                   }
                                   if *data.s.offset(j as (isize)) as (i32) == b'\n' as (i32) {
                                       if byte::diff(
                                        (*b"search \0").as_ptr() as (*mut u8),
                                        7u32,
                                        data.s.offset(i as (isize)),
                                    ) == 0 ||
                                           byte::diff(
                                            (*b"search\t\0").as_ptr() as (*mut u8),
                                            7u32,
                                            data.s.offset(i as (isize)),
                                        ) == 0 ||
                                           byte::diff(
                                            (*b"domain \0").as_ptr() as (*mut u8),
                                            7u32,
                                            data.s.offset(i as (isize)),
                                        ) == 0 ||
                                           byte::diff(
                                            (*b"domain\t\0").as_ptr() as (*mut u8),
                                            7u32,
                                            data.s.offset(i as (isize)),
                                        ) == 0
                                    {
                                           _currentBlock = 29;
                                           break;
                                       }
                                       i = j + 1i32;
                                   }
                                   j = j + 1;
                               }
                               if _currentBlock == 11 {
                               } else if stralloc_copys(rules, (*b"?:\0").as_ptr()) == 0 {
                                   return -1i32;
                               } else {
                                   i = i + 7i32;
                                   'loop31: loop {
                                       if !(i < j) {
                                           _currentBlock = 32;
                                           break;
                                       }
                                       k = byte::chr(
                                        data.s.offset(i as (isize)),
                                        (j - i) as (u32),
                                        b' ' as (i32),
                                    ) as (i32);
                                       k = byte::chr(
                                        data.s.offset(i as (isize)),
                                        k as (u32),
                                        b'\t' as (i32),
                                    ) as (i32);
                                       if k == 0 {
                                           i = i + 1;
                                       } else {
                                           if stralloc_cats(rules, (*b"+.\0").as_ptr()) == 0 {
                                               _currentBlock = 44;
                                               break;
                                           }
                                           if stralloc_catb(
                                            rules,
                                            data.s.offset(i as (isize)) as (*const u8),
                                            k as (u32),
                                        ) == 0
                                        {
                                               _currentBlock = 43;
                                               break;
                                           }
                                           i = i + k;
                                       }
                                   }
                                   if _currentBlock == 32 {
                                       if stralloc_append(rules, (*b"\0").as_ptr()) == 0 {
                                           return -1i32;
                                       } else if stralloc_cats(rules, (*b"*.:\0").as_ptr()) == 0 {
                                           return -1i32;
                                       } else if stralloc_append(rules, (*b"\0").as_ptr()) == 0 {
                                           return -1i32;
                                       } else {
                                           return 0i32;
                                       }
                                   } else if _currentBlock == 43 {
                                       return -1i32;
                                   } else {
                                       return -1i32;
                                   }
                               }
                           }
                       }
                       host[0usize] = 0u8;
                       (if gethostname(host.as_mut_ptr(), ::std::mem::size_of::<[u8; 256]>()) ==
                            -1i32
                    {
                            -1i32
                        } else {
                            host[::std::mem::size_of::<[u8; 256]>().wrapping_sub(1usize)] = 0u8;
                            i = str_chr(host.as_mut_ptr() as (*const u8), b'.' as (i32)) as (i32);
                            if host[i as (usize)] != 0 {
                                if stralloc_copys(rules, (*b"?:\0").as_ptr()) == 0 {
                                    return -1i32;
                                } else if stralloc_cats(
                                rules,
                                host.as_mut_ptr().offset(i as (isize)) as
                                    (*const u8),
                            ) == 0
                            {
                                    return -1i32;
                                } else if stralloc_append(rules, (*b"\0").as_ptr()) == 0 {
                                    return -1i32;
                                }
                            }
                            (if stralloc_cats(rules, (*b"*.:\0").as_ptr()) == 0 {
                                 -1i32
                             } else if stralloc_append(rules, (*b"\0").as_ptr()) == 0 {
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

#[no_mangle]
pub unsafe extern "C" fn dns_resolvconfrewrite(mut out: *mut stralloc) -> i32 {
    let mut now: taia;
    taia_now(&mut now as (*mut taia));
    if taia_less(
        &mut deadline as (*mut taia) as (*const taia),
        &mut now as (*mut taia) as (*const taia),
    ) != 0
    {
        ok = 0i32;
    }
    if uses == 0 {
        ok = 0i32;
    }
    if ok == 0 {
        if init(&mut rules as (*mut stralloc)) == -1i32 {
            return -1i32;
        } else {
            taia_uint(&mut deadline as (*mut taia), 600u32);
            taia_add(
                &mut deadline as (*mut taia),
                &mut now as (*mut taia) as (*const taia),
                &mut deadline as (*mut taia) as (*const taia),
            );
            uses = 10000u32;
            ok = 1i32;
        }
    }
    uses = uses.wrapping_sub(1u32);
    if stralloc_copy(out, &mut rules as (*mut stralloc) as (*const stralloc)) == 0 {
        -1i32
    } else {
        0i32
    }
}
