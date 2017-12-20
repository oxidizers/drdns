use byte;
use errno::{self, Errno};
use libc;
use uint16;

extern "C" {
    fn dns_domain_todot_cat(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn dns_packet_copy(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut u8, arg5: u32) -> u32;
    fn dns_packet_getname(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut *mut u8) -> u32;
    fn printrecord_cat(
        arg1: *mut stralloc,
        arg2: *const u8,
        arg3: u32,
        arg4: u32,
        arg5: *const u8,
        arg6: *const u8,
    ) -> u32;
    fn stralloc_cats(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn stralloc_catulong0(arg1: *mut stralloc, arg2: usize, arg3: u32) -> i32;
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
pub unsafe extern "C" fn printpacket_cat(
    mut out: *mut stralloc,
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
    pos = dns_packet_copy(buf as (*const u8), len, 0u32, data.as_mut_ptr(), 12u32);
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
        (if stralloc_catulong0(out, len as (usize), 0u32) == 0 {
             0u32
         } else if stralloc_cats(out, (*b" bytes, \0").as_ptr()) == 0 {
             0u32
         } else if stralloc_catulong0(out, numqueries as (usize), 0u32) == 0 {
             0u32
         } else if stralloc_cats(out, (*b"+\0").as_ptr()) == 0 {
             0u32
         } else if stralloc_catulong0(out, numanswers as (usize), 0u32) == 0 {
             0u32
         } else if stralloc_cats(out, (*b"+\0").as_ptr()) == 0 {
             0u32
         } else if stralloc_catulong0(out, numauthority as (usize), 0u32) == 0 {
             0u32
         } else if stralloc_cats(out, (*b"+\0").as_ptr()) == 0 {
             0u32
         } else if stralloc_catulong0(out, numglue as (usize), 0u32) == 0 {
             0u32
         } else if stralloc_cats(out, (*b" records\0").as_ptr()) == 0 {
             0u32
         } else {
             if data[2usize] as (i32) & 128i32 != 0 {
                 if stralloc_cats(out, (*b", response\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             }
             if data[2usize] as (i32) & 120i32 != 0 {
                 if stralloc_cats(out, (*b", weird op\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             }
             if data[2usize] as (i32) & 4i32 != 0 {
                 if stralloc_cats(out, (*b", authoritative\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             }
             if data[2usize] as (i32) & 2i32 != 0 {
                 if stralloc_cats(out, (*b", truncated\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             }
             if data[2usize] as (i32) & 1i32 != 0 {
                 if stralloc_cats(out, (*b", weird rd\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             }
             if data[3usize] as (i32) & 128i32 != 0 {
                 if stralloc_cats(out, (*b", weird ra\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             }
             let switch1 = data[3usize] as (i32) & 15i32;
             if switch1 == 5i32 {
                 if stralloc_cats(out, (*b", refused\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             } else if switch1 == 4i32 {
                 if stralloc_cats(out, (*b", notimp\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             } else if switch1 == 3i32 {
                 if stralloc_cats(out, (*b", nxdomain\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             } else if switch1 == 0i32 {
                 if stralloc_cats(out, (*b", noerror\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             } else if stralloc_cats(out, (*b", weird rcode\0").as_ptr()) == 0 {
                 return 0u32;
             }
             if data[3usize] as (i32) & 112i32 != 0 {
                 if stralloc_cats(out, (*b", weird z\0").as_ptr()) == 0 {
                     return 0u32;
                 }
             }
             (if stralloc_cats(out, (*b"\n\0").as_ptr()) == 0 {
                  0u32
              } else {
                  'loop40: loop {
                      if numqueries == 0 {
                          _currentBlock = 41;
                          break;
                      }
                      numqueries = (numqueries as (i32) - 1) as (u16);
                      if stralloc_cats(out, (*b"query: \0").as_ptr()) == 0 {
                          _currentBlock = 71;
                          break;
                      }
                      pos = dns_packet_getname(
                        buf as (*const u8),
                        len,
                        pos,
                        &mut d as (*mut *mut u8),
                    );
                      if pos == 0 {
                          _currentBlock = 70;
                          break;
                      }
                      pos = dns_packet_copy(buf as (*const u8), len, pos, data.as_mut_ptr(), 4u32);
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
                          if stralloc_cats(out, (*b"weird class\0").as_ptr()) == 0 {
                              _currentBlock = 68;
                              break;
                          }
                      } else {
                          uint16::unpack_big(
                            data.as_mut_ptr() as (*const u8),
                            &mut type_ as (*mut u16),
                        );
                          if stralloc_catulong0(out, type_ as (usize), 0u32) == 0 {
                              _currentBlock = 64;
                              break;
                          }
                          if stralloc_cats(out, (*b" \0").as_ptr()) == 0 {
                              _currentBlock = 63;
                              break;
                          }
                          if dns_domain_todot_cat(out, d as (*const u8)) == 0 {
                              _currentBlock = 62;
                              break;
                          }
                      }
                      if stralloc_cats(out, (*b"\n\0").as_ptr()) == 0 {
                          _currentBlock = 67;
                          break;
                      }
                  }
                  (if _currentBlock == 41 {
                       'loop41: loop {
                           if numanswers != 0 {
                               numanswers = (numanswers as (i32) - 1) as (u16);
                               if stralloc_cats(out, (*b"answer: \0").as_ptr()) == 0 {
                                   _currentBlock = 54;
                                   break;
                               }
                           } else if numauthority != 0 {
                               numauthority = (numauthority as (i32) - 1) as (u16);
                               if stralloc_cats(out, (*b"authority: \0").as_ptr()) == 0 {
                                   _currentBlock = 50;
                                   break;
                               }
                           } else {
                               if numglue == 0 {
                                   _currentBlock = 44;
                                   break;
                               }
                               numglue = (numglue as (i32) - 1) as (u16);
                               if stralloc_cats(out, (*b"additional: \0").as_ptr()) == 0 {
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
