use byte;
use dns;
use errno::{self, Errno};
use libc;
use stralloc::StrAlloc;
use uint16;

extern "C" {
    fn printrecord_cat(
        arg1: *mut StrAlloc,
        arg2: *const u8,
        arg3: u32,
        arg4: u32,
        arg5: *const u8,
        arg6: *const u8,
    ) -> u32;
}

static mut d: *mut u8 = 0 as (*mut u8);

#[no_mangle]
pub unsafe extern "C" fn printpacket_cat(
    mut out: *mut StrAlloc,
    mut buf: *mut u8,
    mut len: u32,
) -> u32 {
    let mut _currentBlock;
    let mut numqueries: u16;
    let mut numanswers: u16;
    let mut numauthority: u16;
    let mut numglue: u16;
    let mut pos: u32;
    let mut data: [u8; 12];
    let mut type_: u16;
    pos = dns::packet::copy(buf as (*const u8), len, 0u32, data.as_mut_ptr(), 12u32);
    if pos == 0 {
        0u32
    } else {
        uint16::unpack_big(
            data.as_mut_ptr().offset(4isize) as (*const u8),
            &mut numqueries as (*mut u16),
        );
        uint16::unpack_big(
            data.as_mut_ptr().offset(6isize) as (*const u8),
            &mut numanswers as (*mut u16),
        );
        uint16::unpack_big(
            data.as_mut_ptr().offset(8isize) as (*const u8),
            &mut numauthority as (*mut u16),
        );
        uint16::unpack_big(
            data.as_mut_ptr().offset(10isize) as (*const u8),
            &mut numglue as (*mut u16),
        );
        (if StrAlloc::catulong0(out, len as (usize), 0u32) == 0 {
             0u32
         } else if StrAlloc::cats(out, (*b" bytes, \0").as_ptr()) == 0 {
             0u32
         } else if StrAlloc::catulong0(out, numqueries as (usize), 0u32) == 0 {
             0u32
         } else if StrAlloc::cats(out, (*b"+\0").as_ptr()) == 0 {
             0u32
         } else if StrAlloc::catulong0(out, numanswers as (usize), 0u32) == 0 {
             0u32
         } else if StrAlloc::cats(out, (*b"+\0").as_ptr()) == 0 {
             0u32
         } else if StrAlloc::catulong0(out, numauthority as (usize), 0u32) == 0 {
             0u32
         } else if StrAlloc::cats(out, (*b"+\0").as_ptr()) == 0 {
             0u32
         } else if StrAlloc::catulong0(out, numglue as (usize), 0u32) == 0 {
             0u32
         } else if StrAlloc::cats(out, (*b" records\0").as_ptr()) == 0 {
             0u32
         } else {
             if data[2usize] as (i32) & 128i32 != 0 {
                 if StrAlloc::cats(out, (*b", response\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             }
             if data[2usize] as (i32) & 120i32 != 0 {
                 if StrAlloc::cats(out, (*b", weird op\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             }
             if data[2usize] as (i32) & 4i32 != 0 {
                 if StrAlloc::cats(out, (*b", authoritative\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             }
             if data[2usize] as (i32) & 2i32 != 0 {
                 if StrAlloc::cats(out, (*b", truncated\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             }
             if data[2usize] as (i32) & 1i32 != 0 {
                 if StrAlloc::cats(out, (*b", weird rd\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             }
             if data[3usize] as (i32) & 128i32 != 0 {
                 if StrAlloc::cats(out, (*b", weird ra\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             }
             let switch1 = data[3usize] as (i32) & 15i32;
             if switch1 == 5i32 {
                 if StrAlloc::cats(out, (*b", refused\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             } else if switch1 == 4i32 {
                 if StrAlloc::cats(out, (*b", notimp\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             } else if switch1 == 3i32 {
                 if StrAlloc::cats(out, (*b", nxdomain\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             } else if switch1 == 0i32 {
                 if StrAlloc::cats(out, (*b", noerror\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             } else if StrAlloc::cats(out, (*b", weird rcode\0").as_ptr()) == 0 {
                 return 0u32;
             }
             if data[3usize] as (i32) & 112i32 != 0 {
                 if StrAlloc::cats(out, (*b", weird z\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             }
             (if StrAlloc::cats(out, (*b"\n\0").as_ptr()) == 0 {
                  0u32
              } else {
                  'loop40: loop {
                      if numqueries == 0 {
                          _currentBlock = 41;
                          break;
                      }
                      numqueries = (numqueries as (i32) - 1) as (u16);
                      if StrAlloc::cats(out, (*b"query: \0").as_ptr()) == 0 {
                          _currentBlock = 71;
                          break;
                      }
                      pos = dns::packet::getname(
                        buf as (*const u8),
                        len,
                        pos,
                        &mut d as (*mut *mut u8),
                    );
                      if pos == 0 {
                          _currentBlock = 70;
                          break;
                      }
                      pos = dns::packet::copy(buf as (*const u8), len, pos, data.as_mut_ptr(), 4u32);
                      if pos == 0 {
                          _currentBlock = 69;
                          break;
                      }
                      if byte::diff(
                        data.as_mut_ptr().offset(2isize),
                        2u32,
                        (*b"\0\x01\0").as_ptr() as (*mut u8),
                    ) != 0
                    {
                          if StrAlloc::cats(out, (*b"weird class\0").as_ptr()) == 0 {
                              _currentBlock = 68;
                              break;
                          }
                      } else {
                          uint16::unpack_big(
                            data.as_mut_ptr() as (*const u8),
                            &mut type_ as (*mut u16),
                        );
                          if StrAlloc::catulong0(out, type_ as (usize), 0u32) == 0 {
                              _currentBlock = 64;
                              break;
                          }
                          if StrAlloc::cats(out, (*b" \0").as_ptr()) == 0 {
                              _currentBlock = 63;
                              break;
                          }
                          if dns::domain::todot_cat(out, d as (*const u8)) == 0 {
                              _currentBlock = 62;
                              break;
                          }
                      }
                      if StrAlloc::cats(out, (*b"\n\0").as_ptr()) == 0 {
                          _currentBlock = 67;
                          break;
                      }
                  }
                  (if _currentBlock == 41 {
                       'loop41: loop {
                           if numanswers != 0 {
                               numanswers = (numanswers as (i32) - 1) as (u16);
                               if StrAlloc::cats(out, (*b"answer: \0").as_ptr()) == 0 {
                                   _currentBlock = 54;
                                   break;
                               }
                           } else if numauthority != 0 {
                               numauthority = (numauthority as (i32) - 1) as (u16);
                               if StrAlloc::cats(out, (*b"authority: \0").as_ptr()) == 0 {
                                   _currentBlock = 50;
                                   break;
                               }
                           } else {
                               if numglue == 0 {
                                   _currentBlock = 44;
                                   break;
                               }
                               numglue = (numglue as (i32) - 1) as (u16);
                               if StrAlloc::cats(out, (*b"additional: \0").as_ptr()) == 0 {
                                   _currentBlock = 48;
                                   break;
                               }
                           }
                           pos = printrecord_cat(
                            out,
                            buf as (*const u8),
                            len,
                            pos,
                            0i32 as (*const u8),
                            0i32 as (*const u8),
                        );
                           if pos == 0 {
                               _currentBlock = 53;
                               break;
                           }
                       }
                       (if _currentBlock == 44 {
                            (if pos != len {
                                 errno::set_errno(Errno(libc::EPROTO));
                                 0u32
                             } else {
                                 1u32
                             })
                        } else if _currentBlock == 48 {
                            0u32
                        } else if _currentBlock == 50 {
                            0u32
                        } else if _currentBlock == 53 {
                            0u32
                        } else {
                            0u32
                        })
                   } else if _currentBlock == 62 {
                       0u32
                   } else if _currentBlock == 63 {
                       0u32
                   } else if _currentBlock == 64 {
                       0u32
                   } else if _currentBlock == 67 {
                       0u32
                   } else if _currentBlock == 68 {
                       0u32
                   } else if _currentBlock == 69 {
                       0u32
                   } else if _currentBlock == 70 {
                       0u32
                   } else {
                       0u32
                   })
              })
         })
    }
}
