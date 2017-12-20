use byte;
use tai::Tai;
use taia::TaiA;
use uint16;

extern "C" {
    fn dns_domain_free(arg1: *mut *mut u8);
    fn dns_domain_fromdot(arg1: *mut *mut u8, arg2: *const u8, arg3: u32) -> i32;
    fn dns_domain_todot_cat(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn dns_packet_copy(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut u8, arg5: u32) -> u32;
    fn dns_packet_getname(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut *mut u8) -> u32;
    fn dns_packet_skipname(arg1: *const u8, arg2: u32, arg3: u32) -> u32;
    fn dns_resolve(arg1: *const u8, arg2: *const u8) -> i32;
    static mut dns_resolve_tx: dns_transmit;
    fn dns_transmit_free(arg1: *mut dns_transmit);
    fn stralloc_append(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn stralloc_catb(arg1: *mut stralloc, arg2: *const u8, arg3: u32) -> i32;
    fn stralloc_copys(arg1: *mut stralloc, arg2: *const u8) -> i32;
}

static mut q: *mut u8 = 0i32 as (*mut u8);

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
pub unsafe extern "C" fn dns_mx_packet(
    mut out: *mut stralloc,
    mut buf: *const u8,
    mut len: u32,
) -> i32 {
    let mut _currentBlock;
    let mut pos: u32;
    let mut header: [u8; 12];
    let mut pref: [u8; 2];
    let mut numanswers: u16;
    let mut datalen: u16;
    if stralloc_copys(out, (*b"\0").as_ptr()) == 0 {
        -1i32
    } else {
        pos = dns_packet_copy(buf, len, 0u32, header.as_mut_ptr(), 12u32);
        (if pos == 0 {
             -1i32
         } else {
             uint16::unpack_big(
                header.as_mut_ptr().offset(6isize) as (*const u8),
                &mut numanswers as (*mut u16),
            );
             pos = dns_packet_skipname(buf, len, pos);
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
                          _currentBlock = 5;
                          break;
                      }
                      pos = dns_packet_skipname(buf, len, pos);
                      if pos == 0 {
                          _currentBlock = 22;
                          break;
                      }
                      pos = dns_packet_copy(buf, len, pos, header.as_mut_ptr(), 10u32);
                      if pos == 0 {
                          _currentBlock = 21;
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
                              if dns_packet_copy(buf, len, pos, pref.as_mut_ptr(), 2u32) == 0 {
                                  _currentBlock = 20;
                                  break;
                              }
                              if dns_packet_getname(
                                buf,
                                len,
                                pos.wrapping_add(2u32),
                                &mut q as (*mut *mut u8),
                            ) == 0
                            {
                                  _currentBlock = 19;
                                  break;
                              }
                              if stralloc_catb(out, pref.as_mut_ptr() as (*const u8), 2u32) == 0 {
                                  _currentBlock = 18;
                                  break;
                              }
                              if dns_domain_todot_cat(out, q as (*const u8)) == 0 {
                                  _currentBlock = 17;
                                  break;
                              }
                              if stralloc_append(out, (*b"\0").as_ptr()) == 0 {
                                  _currentBlock = 16;
                                  break;
                              }
                          }
                      }
                      pos = pos.wrapping_add(datalen as (u32));
                  }
                  (if _currentBlock == 5 {
                       0i32
                   } else if _currentBlock == 16 {
                       -1i32
                   } else if _currentBlock == 17 {
                       -1i32
                   } else if _currentBlock == 18 {
                       -1i32
                   } else if _currentBlock == 19 {
                       -1i32
                   } else if _currentBlock == 20 {
                       -1i32
                   } else if _currentBlock == 21 {
                       -1i32
                   } else {
                       -1i32
                   })
              })
         })
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct dns_transmit {
    pub query: *mut u8,
    pub querylen: u32,
    pub packet: *mut u8,
    pub packetlen: u32,
    pub s1: i32,
    pub tcpstate: i32,
    pub udploop: u32,
    pub curserver: u32,
    pub deadline: TaiA,
    pub pos: u32,
    pub servers: *const u8,
    pub localip: [u8; 4],
    pub qtype: [u8; 2],
}

impl Clone for dns_transmit {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn dns_mx(mut out: *mut stralloc, mut fqdn: *const stralloc) -> i32 {
    if dns_domain_fromdot(
        &mut q as (*mut *mut u8),
        (*fqdn).s as (*const u8),
        (*fqdn).len,
    ) == 0
    {
        -1i32
    } else if dns_resolve(q as (*const u8), (*b"\0\x0F\0").as_ptr()) == -1i32 {
        -1i32
    } else if dns_mx_packet(
        out,
        dns_resolve_tx.packet as (*const u8),
        dns_resolve_tx.packetlen,
    ) == -1i32
    {
        -1i32
    } else {
        dns_transmit_free(&mut dns_resolve_tx as (*mut dns_transmit));
        dns_domain_free(&mut q as (*mut *mut u8));
        0i32
    }
}
