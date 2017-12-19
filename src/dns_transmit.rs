use alloc;
use byte;
use errno::{self, Errno};
use libc;

extern "C" {
    fn close(arg1: i32) -> i32;
    fn dns_domain_equal(arg1: *const u8, arg2: *const u8) -> i32;
    fn dns_domain_length(arg1: *const u8) -> u32;
    fn dns_packet_copy(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut u8, arg5: u32) -> u32;
    fn dns_packet_getname(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut *mut u8) -> u32;
    fn dns_random(arg1: u32) -> u32;
    fn recv(arg1: i32, arg2: *mut ::std::os::raw::c_void, arg3: usize, arg4: i32) -> isize;
    fn send(arg1: i32, arg2: *const ::std::os::raw::c_void, arg3: usize, arg4: i32) -> isize;
    fn socket_bind4(arg1: i32, arg2: *mut u8, arg3: u16) -> i32;
    fn socket_connect4(arg1: i32, arg2: *const u8, arg3: u16) -> i32;
    fn socket_connected(arg1: i32) -> i32;
    fn socket_tcp() -> i32;
    fn socket_udp() -> i32;
    fn taia_add(arg1: *mut taia, arg2: *const taia, arg3: *const taia);
    fn taia_less(arg1: *const taia, arg2: *const taia) -> i32;
    fn taia_now(arg1: *mut taia);
    fn taia_uint(arg1: *mut taia, arg2: u32);
    fn uint16_pack_big(arg1: *mut u8, arg2: u16);
}

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
    pub deadline: taia,
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

unsafe extern "C" fn queryfree(mut d: *mut dns_transmit) {
    if (*d).query.is_null() {
    } else {
        alloc::free((*d).query);
        (*d).query = 0i32 as (*mut u8);
    }
}

unsafe extern "C" fn socketfree(mut d: *mut dns_transmit) {
    if (*d).s1 == 0 {
    } else {
        close((*d).s1 - 1i32);
        (*d).s1 = 0i32;
    }
}

unsafe extern "C" fn packetfree(mut d: *mut dns_transmit) {
    if (*d).packet.is_null() {
    } else {
        alloc::free((*d).packet);
        (*d).packet = 0i32 as (*mut u8);
    }
}

#[no_mangle]
pub unsafe extern "C" fn dns_transmit_free(mut d: *mut dns_transmit) {
    queryfree(d);
    socketfree(d);
    packetfree(d);
}

static mut timeouts: [i32; 4] = [1i32, 3i32, 11i32, 45i32];

unsafe extern "C" fn randombind(mut d: *mut dns_transmit) -> i32 {
    let mut _currentBlock;
    let mut j: i32;
    j = 0i32;
    'loop1: loop {
        if !(j < 10i32) {
            _currentBlock = 2;
            break;
        }
        if socket_bind4(
            (*d).s1 - 1i32,
            (*d).localip.as_mut_ptr(),
            1025u32.wrapping_add(dns_random(64510u32)) as (u16),
        ) == 0i32
        {
            _currentBlock = 7;
            break;
        }
        j = j + 1;
    }
    if _currentBlock == 2 {
        (if socket_bind4((*d).s1 - 1i32, (*d).localip.as_mut_ptr(), 0u16) == 0i32 {
             0i32
         } else {
             -1i32
         })
    } else {
        0i32
    }
}

unsafe extern "C" fn thistcp(mut d: *mut dns_transmit) -> i32 {
    let mut _currentBlock;
    let mut now: taia;
    let mut ip: *const u8;
    socketfree(d);
    packetfree(d);
    'loop1: loop {
        if !((*d).curserver < 16u32) {
            _currentBlock = 2;
            break;
        }
        ip = (*d).servers.offset(
            4u32.wrapping_mul((*d).curserver) as (isize),
        );
        if byte::diff(
            ip as (*mut u8),
            4u32,
            (*b"\0\0\0\0\0").as_ptr() as (*mut u8),
        ) != 0
        {
            *(*d).query.offset(2isize) = dns_random(256u32) as (u8);
            *(*d).query.offset(3isize) = dns_random(256u32) as (u8);
            (*d).s1 = 1i32 + socket_tcp();
            if (*d).s1 == 0 {
                _currentBlock = 13;
                break;
            }
            if randombind(d) == -1i32 {
                _currentBlock = 12;
                break;
            }
            taia_now(&mut now as (*mut taia));
            taia_uint(&mut (*d).deadline as (*mut taia), 10u32);
            taia_add(
                &mut (*d).deadline as (*mut taia),
                &mut (*d).deadline as (*mut taia) as (*const taia),
                &mut now as (*mut taia) as (*const taia),
            );
            if socket_connect4((*d).s1 - 1i32, ip, 53u16) == 0i32 {
                _currentBlock = 11;
                break;
            }
            if errno::errno() == Errno(libc::EINPROGRESS) || errno::errno() == Errno(libc::EWOULDBLOCK) {
                _currentBlock = 10;
                break;
            }
            socketfree(d);
        }
        (*d).curserver = (*d).curserver.wrapping_add(1u32);
    }
    if _currentBlock == 2 {
        dns_transmit_free(d);
        -1i32
    } else if _currentBlock == 10 {
        (*d).tcpstate = 1i32;
        0i32
    } else if _currentBlock == 11 {
        (*d).tcpstate = 2i32;
        0i32
    } else if _currentBlock == 12 {
        dns_transmit_free(d);
        -1i32
    } else {
        dns_transmit_free(d);
        -1i32
    }
}

unsafe extern "C" fn firsttcp(mut d: *mut dns_transmit) -> i32 {
    (*d).curserver = 0u32;
    thistcp(d)
}

unsafe extern "C" fn thisudp(mut d: *mut dns_transmit) -> i32 {
    let mut _currentBlock;
    let mut ip: *const u8;
    socketfree(d);
    'loop1: loop {
        if !((*d).udploop < 4u32) {
            _currentBlock = 2;
            break;
        }
        'loop3: loop {
            if !((*d).curserver < 16u32) {
                break;
            }
            ip = (*d).servers.offset(
                4u32.wrapping_mul((*d).curserver) as (isize),
            );
            if byte::diff(
                ip as (*mut u8),
                4u32,
                (*b"\0\0\0\0\0").as_ptr() as (*mut u8),
            ) != 0
            {
                *(*d).query.offset(2isize) = dns_random(256u32) as (u8);
                *(*d).query.offset(3isize) = dns_random(256u32) as (u8);
                (*d).s1 = 1i32 + socket_udp();
                if (*d).s1 == 0 {
                    _currentBlock = 14;
                    break 'loop1;
                }
                if randombind(d) == -1i32 {
                    _currentBlock = 13;
                    break 'loop1;
                }
                if socket_connect4((*d).s1 - 1i32, ip, 53u16) == 0i32 {
                    if send(
                        (*d).s1 - 1i32,
                        (*d).query.offset(2isize) as (*const ::std::os::raw::c_void),
                        (*d).querylen.wrapping_sub(2u32) as (usize),
                        0i32,
                    ) as (usize) ==
                        (*d).querylen.wrapping_sub(2u32) as (usize)
                    {
                        _currentBlock = 12;
                        break 'loop1;
                    }
                }
                socketfree(d);
            }
            (*d).curserver = (*d).curserver.wrapping_add(1u32);
        }
        (*d).udploop = (*d).udploop.wrapping_add(1u32);
        (*d).curserver = 0u32;
    }
    if _currentBlock == 2 {
        dns_transmit_free(d);
        -1i32
    } else if _currentBlock == 12 {
        let mut now: taia;
        taia_now(&mut now as (*mut taia));
        taia_uint(
            &mut (*d).deadline as (*mut taia),
            timeouts[(*d).udploop as (usize)] as (u32),
        );
        taia_add(
            &mut (*d).deadline as (*mut taia),
            &mut (*d).deadline as (*mut taia) as (*const taia),
            &mut now as (*mut taia) as (*const taia),
        );
        (*d).tcpstate = 0i32;
        0i32
    } else if _currentBlock == 13 {
        dns_transmit_free(d);
        -1i32
    } else {
        dns_transmit_free(d);
        -1i32
    }
}

unsafe extern "C" fn firstudp(mut d: *mut dns_transmit) -> i32 {
    (*d).curserver = 0u32;
    thisudp(d)
}

#[no_mangle]
pub unsafe extern "C" fn dns_transmit_start(
    mut d: *mut dns_transmit,
    mut servers: *const u8,
    mut flagrecursive: i32,
    mut q: *const u8,
    mut qtype: *const u8,
    mut localip: *const u8,
) -> i32 {
    let mut len: u32;
    dns_transmit_free(d);
    errno::set_errno(Errno(libc::EIO));
    len = dns_domain_length(q);
    (*d).querylen = len.wrapping_add(18u32);
    (*d).query = alloc::alloc((*d).querylen);
    if (*d).query.is_null() {
        -1i32
    } else {
        uint16_pack_big((*d).query, len.wrapping_add(16u32) as (u16));
        byte::copy(
            (*d).query.offset(2isize),
            12u32,
            if flagrecursive != 0 {
                (*b"\0\0\x01\0\0\x01\0\0\0\0\0\0\0").as_ptr()
            } else {
                (*b"\0\0\0\0\0\x01\0\0\0\0\0\0gcc-bug-workaround\0").as_ptr()
            } as (*mut u8),
        );
        byte::copy((*d).query.offset(14isize), len, q as (*mut u8));
        byte::copy(
            (*d).query.offset(14isize).offset(len as (isize)),
            2u32,
            qtype as (*mut u8),
        );
        byte::copy(
            (*d).query.offset(16isize).offset(len as (isize)),
            2u32,
            (*b"\0\x01\0").as_ptr() as (*mut u8),
        );
        byte::copy((*d).qtype.as_mut_ptr(), 2u32, qtype as (*mut u8));
        (*d).servers = servers;
        byte::copy((*d).localip.as_mut_ptr(), 4u32, localip as (*mut u8));
        (*d).udploop = if flagrecursive != 0 { 1i32 } else { 0i32 } as (u32);
        (if len.wrapping_add(16u32) > 512u32 {
             firsttcp(d)
         } else {
             firstudp(d)
         })
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct pollfd {
    pub fd: i32,
    pub events: i16,
    pub revents: i16,
}

impl Clone for pollfd {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn dns_transmit_io(
    mut d: *mut dns_transmit,
    mut x: *mut pollfd,
    mut deadline: *mut taia,
) {
    (*x).fd = (*d).s1 - 1i32;
    let switch1 = (*d).tcpstate;
    if switch1 == 2i32 || switch1 == 1i32 {
        (*x).events = 0x4i16;
    } else if switch1 == 5i32 || switch1 == 4i32 || switch1 == 3i32 || switch1 == 0i32 {
        (*x).events = 0x1i16;
    }
    if taia_less(
        &mut (*d).deadline as (*mut taia) as (*const taia),
        deadline as (*const taia),
    ) != 0
    {
        *deadline = (*d).deadline;
    }
}

unsafe extern "C" fn nextudp(mut d: *mut dns_transmit) -> i32 {
    (*d).curserver = (*d).curserver.wrapping_add(1u32);
    thisudp(d)
}

unsafe extern "C" fn nexttcp(mut d: *mut dns_transmit) -> i32 {
    (*d).curserver = (*d).curserver.wrapping_add(1u32);
    thistcp(d)
}

unsafe extern "C" fn irrelevant(
    mut d: *const dns_transmit,
    mut buf: *const u8,
    mut len: u32,
) -> i32 {
    let mut out: [u8; 12];
    let mut dn: *mut u8;
    let mut pos: u32;
    pos = dns_packet_copy(buf, len, 0u32, out.as_mut_ptr(), 12u32);
    if pos == 0 {
        1i32
    } else if byte::diff(out.as_mut_ptr(), 2u32, (*d).query.offset(2isize)) != 0 {
        1i32
    } else if out[4usize] as (i32) != 0i32 {
        1i32
    } else if out[5usize] as (i32) != 1i32 {
        1i32
    } else {
        dn = 0i32 as (*mut u8);
        pos = dns_packet_getname(buf, len, pos, &mut dn as (*mut *mut u8));
        (if pos == 0 {
             1i32
         } else if dns_domain_equal(
            dn as (*const u8),
            (*d).query.offset(14isize) as (*const u8),
        ) == 0
        {
             alloc::free(dn);
             1i32
         } else {
             alloc::free(dn);
             pos = dns_packet_copy(buf, len, pos, out.as_mut_ptr(), 4u32);
             (if pos == 0 {
                  1i32
              } else if byte::diff(out.as_mut_ptr(), 2u32, (*d).qtype.as_mut_ptr()) != 0 {
                  1i32
              } else if byte::diff(
                out.as_mut_ptr().offset(2isize),
                2u32,
                (*b"\0\x01\0").as_ptr() as (*mut u8),
            ) != 0
            {
                  1i32
              } else {
                  0i32
              })
         })
    }
}

unsafe extern "C" fn serverwantstcp(mut buf: *const u8, mut len: u32) -> i32 {
    let mut out: [u8; 12];
    if dns_packet_copy(buf, len, 0u32, out.as_mut_ptr(), 12u32) == 0 {
        1i32
    } else if out[2usize] as (i32) & 2i32 != 0 {
        1i32
    } else {
        0i32
    }
}

unsafe extern "C" fn serverfailed(mut buf: *const u8, mut len: u32) -> i32 {
    let mut out: [u8; 12];
    let mut rcode: u32;
    if dns_packet_copy(buf, len, 0u32, out.as_mut_ptr(), 12u32) == 0 {
        1i32
    } else {
        rcode = out[3usize] as (u32);
        rcode = rcode & 15u32;
        (if rcode != 0 && (rcode != 3u32) {
             errno::set_errno(Errno(libc::EAGAIN));
             1i32
         } else {
             0i32
         })
    }
}

#[no_mangle]
pub unsafe extern "C" fn dns_transmit_get(
    mut d: *mut dns_transmit,
    mut x: *const pollfd,
    mut when: *const taia,
) -> i32 {
    let mut udpbuf: [u8; 513];
    let mut ch: u8;
    let mut r: i32;
    let mut fd: i32;
    errno::set_errno(Errno(libc::EIO));
    fd = (*d).s1 - 1i32;
    if (*x).revents == 0 {
        (if taia_less(when, &mut (*d).deadline as (*mut taia) as (*const taia)) != 0 {
             0i32
         } else {
             errno::set_errno(Errno(libc::ETIMEDOUT));
             (if (*d).tcpstate == 0i32 {
                  nextudp(d)
              } else {
                  nexttcp(d)
              })
         })
    } else if (*d).tcpstate == 0i32 {
        r = recv(
            fd,
            udpbuf.as_mut_ptr() as (*mut ::std::os::raw::c_void),
            ::std::mem::size_of::<[u8; 513]>(),
            0i32,
        ) as (i32);
        (if r <= 0i32 {
             if errno::errno() == Errno(libc::ECONNREFUSED) {
                 if (*d).udploop == 2u32 {
                     return 0i32;
                 }
             }
             nextudp(d)
         } else if (r + 1i32) as (usize) > ::std::mem::size_of::<[u8; 513]>() {
             0i32
         } else if irrelevant(
            d as (*const dns_transmit),
            udpbuf.as_mut_ptr() as (*const u8),
            r as (u32),
        ) != 0
        {
             0i32
         } else if serverwantstcp(udpbuf.as_mut_ptr() as (*const u8), r as (u32)) != 0 {
             firsttcp(d)
         } else if serverfailed(udpbuf.as_mut_ptr() as (*const u8), r as (u32)) != 0 {
             (if (*d).udploop == 2u32 {
                  0i32
              } else {
                  nextudp(d)
              })
         } else {
             socketfree(d);
             (*d).packetlen = r as (u32);
             (*d).packet = alloc::alloc((*d).packetlen);
             (if (*d).packet.is_null() {
                  dns_transmit_free(d);
                  -1i32
              } else {
                  byte::copy((*d).packet, (*d).packetlen, udpbuf.as_mut_ptr());
                  queryfree(d);
                  1i32
              })
         })
    } else if (*d).tcpstate == 1i32 {
        (if socket_connected(fd) == 0 {
             nexttcp(d)
         } else {
             (*d).pos = 0u32;
             (*d).tcpstate = 2i32;
             0i32
         })
    } else if (*d).tcpstate == 2i32 {
        r = libc::write(
            fd,
            (*d).query.offset((*d).pos as (isize)) as (*const libc::c_void),
            (*d).querylen.wrapping_sub((*d).pos) as (usize),
        ) as (i32);
        (if r <= 0i32 {
             nexttcp(d)
         } else {
             (*d).pos = (*d).pos.wrapping_add(r as (u32));
             if (*d).pos == (*d).querylen {
                 let mut now: taia;
                 taia_now(&mut now as (*mut taia));
                 taia_uint(&mut (*d).deadline as (*mut taia), 10u32);
                 taia_add(
                    &mut (*d).deadline as (*mut taia),
                    &mut (*d).deadline as (*mut taia) as (*const taia),
                    &mut now as (*mut taia) as (*const taia),
                );
                 (*d).tcpstate = 3i32;
             }
             0i32
         })
    } else if (*d).tcpstate == 3i32 {
        r = libc::read(
            fd,
            &mut ch as (*mut u8) as (*mut libc::c_void),
            1usize,
        ) as (i32);
        (if r <= 0i32 {
             nexttcp(d)
         } else {
             (*d).packetlen = ch as (u32);
             (*d).tcpstate = 4i32;
             0i32
         })
    } else if (*d).tcpstate == 4i32 {
        r = libc::read(
            fd,
            &mut ch as (*mut u8) as (*mut libc::c_void),
            1usize,
        ) as (i32);
        (if r <= 0i32 {
             nexttcp(d)
         } else {
             (*d).packetlen = (*d).packetlen << 8i32;
             (*d).packetlen = (*d).packetlen.wrapping_add(ch as (u32));
             (*d).tcpstate = 5i32;
             (*d).pos = 0u32;
             (*d).packet = alloc::alloc((*d).packetlen);
             (if (*d).packet.is_null() {
                  dns_transmit_free(d);
                  -1i32
              } else {
                  0i32
              })
         })
    } else if (*d).tcpstate == 5i32 {
        r = libc::read(
            fd,
            (*d).packet.offset((*d).pos as (isize)) as (*mut libc::c_void),
            (*d).packetlen.wrapping_sub((*d).pos) as (usize),
        ) as (i32);
        (if r <= 0i32 {
             nexttcp(d)
         } else {
             (*d).pos = (*d).pos.wrapping_add(r as (u32));
             (if (*d).pos < (*d).packetlen {
                  0i32
              } else {
                  socketfree(d);
                  (if irrelevant(
                    d as (*const dns_transmit),
                    (*d).packet as (*const u8),
                    (*d).packetlen,
                ) != 0
                {
                       nexttcp(d)
                   } else if serverwantstcp((*d).packet as (*const u8), (*d).packetlen) != 0 {
                       nexttcp(d)
                   } else if serverfailed((*d).packet as (*const u8), (*d).packetlen) != 0 {
                       nexttcp(d)
                   } else {
                       queryfree(d);
                       1i32
                   })
              })
         })
    } else {
        0i32
    }
}
