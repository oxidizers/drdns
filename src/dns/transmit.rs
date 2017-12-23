use alloc;
use byte;
use errno::{self, Errno};
use libc;
use socket;
use super::{domain, packet, random};
use taia::TaiA;
use uint16;

#[derive(Copy)]
#[repr(C)]
pub struct DnsTransmit {
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

impl Clone for DnsTransmit {
    fn clone(&self) -> Self {
        *self
    }
}

const TIMEOUTS: [u32; 4] = [1, 3, 11, 45];

impl DnsTransmit {
    pub unsafe fn free(d: *mut DnsTransmit) {
        DnsTransmit::queryfree(d);
        DnsTransmit::socketfree(d);
        DnsTransmit::packetfree(d);
    }

    pub unsafe fn start(
        d: *mut DnsTransmit,
        servers: *const u8,
        flagrecursive: i32,
        q: *const u8,
        qtype: *const u8,
        localip: *const u8,
    ) -> i32 {
        let len: u32;
        DnsTransmit::free(d);
        errno::set_errno(Errno(libc::EIO));
        len = domain::length(q);
        (*d).querylen = len.wrapping_add(18u32);
        (*d).query = alloc::alloc((*d).querylen);
        if (*d).query.is_null() {
            -1i32
        } else {
            uint16::pack_big((*d).query, len.wrapping_add(16u32) as (u16));
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
                DnsTransmit::firsttcp(d)
            } else {
                DnsTransmit::firstudp(d)
            })
        }
    }

    pub unsafe fn io(
        d: *mut DnsTransmit,
        x: *mut libc::pollfd,
        deadline: *mut TaiA,
    ) {
        (*x).fd = (*d).s1 - 1i32;
        let switch1 = (*d).tcpstate;
        if switch1 == 2i32 || switch1 == 1i32 {
            (*x).events = 0x4i16;
        } else if switch1 == 5i32 || switch1 == 4i32 || switch1 == 3i32 || switch1 == 0i32 {
            (*x).events = 0x1i16;
        }
        if TaiA::less(
            &mut (*d).deadline as (*mut TaiA) as (*const TaiA),
            deadline as (*const TaiA),
        ) != 0
        {
            *deadline = (*d).deadline;
        }
    }

    pub unsafe fn get(
        d: *mut DnsTransmit,
        x: *const libc::pollfd,
        when: *const TaiA,
    ) -> i32 {
        let mut udpbuf: [u8; 513] = [0u8; 513];
        let mut ch: u8 = 0;
        let r: i32;
        let fd: i32;
        errno::set_errno(Errno(libc::EIO));
        fd = (*d).s1 - 1i32;
        if (*x).revents == 0 {
            (if TaiA::less(when, &mut (*d).deadline as (*mut TaiA) as (*const TaiA)) != 0 {
                0i32
            } else {
                errno::set_errno(Errno(libc::ETIMEDOUT));
                (if (*d).tcpstate == 0i32 {
                    DnsTransmit::nextudp(d)
                } else {
                    DnsTransmit::nexttcp(d)
                })
            })
        } else if (*d).tcpstate == 0i32 {
            r = libc::recv(
                fd,
                udpbuf.as_mut_ptr() as (*mut ::libc::c_void),
                ::std::mem::size_of::<[u8; 513]>(),
                0i32,
            ) as (i32);
            (if r <= 0i32 {
                if errno::errno() == Errno(libc::ECONNREFUSED) {
                    if (*d).udploop == 2u32 {
                        return 0i32;
                    }
                }
                DnsTransmit::nextudp(d)
            } else if (r + 1i32) as (usize) > ::std::mem::size_of::<[u8; 513]>() {
                0i32
            } else if DnsTransmit::irrelevant(
                d as (*mut DnsTransmit),
                udpbuf.as_mut_ptr() as (*const u8),
                r as (u32),
            ) != 0
            {
                0i32
            } else if DnsTransmit::serverwantstcp(udpbuf.as_mut_ptr() as (*const u8), r as (u32)) != 0 {
                DnsTransmit::firsttcp(d)
            } else if DnsTransmit::serverfailed(udpbuf.as_mut_ptr() as (*const u8), r as (u32)) != 0 {
                (if (*d).udploop == 2u32 {
                    0i32
                } else {
                    DnsTransmit::nextudp(d)
                })
            } else {
                DnsTransmit::socketfree(d);
                (*d).packetlen = r as (u32);
                (*d).packet = alloc::alloc((*d).packetlen);
                (if (*d).packet.is_null() {
                    DnsTransmit::free(d);
                    -1i32
                } else {
                    byte::copy((*d).packet, (*d).packetlen, udpbuf.as_mut_ptr());
                    DnsTransmit::queryfree(d);
                    1i32
                })
            })
        } else if (*d).tcpstate == 1i32 {
            (if socket::connected(fd) == 0 {
                DnsTransmit::nexttcp(d)
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
                DnsTransmit::nexttcp(d)
            } else {
                (*d).pos = (*d).pos.wrapping_add(r as (u32));
                if (*d).pos == (*d).querylen {
                    let mut now: TaiA = ::std::mem::zeroed();
                    TaiA::now(&mut now as (*mut TaiA));
                    TaiA::uint(&mut (*d).deadline as (*mut TaiA), 10u32);
                    TaiA::add(
                        &mut (*d).deadline as (*mut TaiA),
                        &mut (*d).deadline as (*mut TaiA) as (*const TaiA),
                        &mut now as (*mut TaiA) as (*const TaiA),
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
                DnsTransmit::nexttcp(d)
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
                DnsTransmit::nexttcp(d)
            } else {
                (*d).packetlen = (*d).packetlen << 8i32;
                (*d).packetlen = (*d).packetlen.wrapping_add(ch as (u32));
                (*d).tcpstate = 5i32;
                (*d).pos = 0u32;
                (*d).packet = alloc::alloc((*d).packetlen);
                (if (*d).packet.is_null() {
                    DnsTransmit::free(d);
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
                DnsTransmit::nexttcp(d)
            } else {
                (*d).pos = (*d).pos.wrapping_add(r as (u32));
                (if (*d).pos < (*d).packetlen {
                    0i32
                } else {
                    DnsTransmit::socketfree(d);
                    (if DnsTransmit::irrelevant(
                        d as (*mut DnsTransmit),
                        (*d).packet as (*const u8),
                        (*d).packetlen,
                    ) != 0
                    {
                        DnsTransmit::nexttcp(d)
                    } else if DnsTransmit::serverwantstcp((*d).packet as (*const u8), (*d).packetlen) != 0 {
                        DnsTransmit::nexttcp(d)
                    } else if DnsTransmit::serverfailed((*d).packet as (*const u8), (*d).packetlen) != 0 {
                        DnsTransmit::nexttcp(d)
                    } else {
                        DnsTransmit::queryfree(d);
                        1i32
                    })
                })
            })
        } else {
            0i32
        }
    }

    unsafe fn queryfree(d: *mut DnsTransmit) {
        if (*d).query.is_null() {
        } else {
            alloc::free((*d).query);
            (*d).query = 0i32 as (*mut u8);
        }
    }

    unsafe fn socketfree(d: *mut DnsTransmit) {
        if (*d).s1 == 0 {
        } else {
            libc::close((*d).s1 - 1i32);
            (*d).s1 = 0i32;
        }
    }

    unsafe fn packetfree(d: *mut DnsTransmit) {
        if (*d).packet.is_null() {
        } else {
            alloc::free((*d).packet);
            (*d).packet = 0i32 as (*mut u8);
        }
    }

    unsafe fn randombind(d: *mut DnsTransmit) -> i32 {
        let current_block;
        let mut j: i32;
        j = 0i32;
        'loop1: loop {
            if !(j < 10i32) {
                current_block = 2;
                break;
            }
            if socket::bind4(
                (*d).s1 - 1i32,
                (*d).localip.as_mut_ptr(),
                1025u32.wrapping_add(random::random(64510u32)) as (u16),
            ) == 0i32
            {
                current_block = 7;
                break;
            }
            j = j + 1;
        }
        if current_block == 2 {
            (if socket::bind4((*d).s1 - 1i32, (*d).localip.as_mut_ptr(), 0u16) == 0i32 {
                0i32
            } else {
                -1i32
            })
        } else {
            0i32
        }
    }

    unsafe fn thistcp(d: *mut DnsTransmit) -> i32 {
        let current_block;
        let mut now: TaiA = ::std::mem::zeroed();
        let mut ip: *const u8;
        DnsTransmit::socketfree(d);
        DnsTransmit::packetfree(d);
        'loop1: loop {
            if !((*d).curserver < 16u32) {
                current_block = 2;
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
                *(*d).query.offset(2isize) = random::random(256u32) as (u8);
                *(*d).query.offset(3isize) = random::random(256u32) as (u8);
                (*d).s1 = 1i32 + socket::tcp();
                if (*d).s1 == 0 {
                    current_block = 13;
                    break;
                }
                if DnsTransmit::randombind(d) == -1i32 {
                    current_block = 12;
                    break;
                }
                TaiA::now(&mut now as (*mut TaiA));
                TaiA::uint(&mut (*d).deadline as (*mut TaiA), 10u32);
                TaiA::add(
                    &mut (*d).deadline as (*mut TaiA),
                    &mut (*d).deadline as (*mut TaiA) as (*const TaiA),
                    &mut now as (*mut TaiA) as (*const TaiA),
                );
                if socket::connect4((*d).s1 - 1i32, ip, 53u16) == 0i32 {
                    current_block = 11;
                    break;
                }
                if errno::errno() == Errno(libc::EINPROGRESS) || errno::errno() == Errno(libc::EWOULDBLOCK) {
                    current_block = 10;
                    break;
                }
                DnsTransmit::socketfree(d);
            }
            (*d).curserver = (*d).curserver.wrapping_add(1u32);
        }
        if current_block == 2 {
            DnsTransmit::free(d);
            -1i32
        } else if current_block == 10 {
            (*d).tcpstate = 1i32;
            0i32
        } else if current_block == 11 {
            (*d).tcpstate = 2i32;
            0i32
        } else if current_block == 12 {
            DnsTransmit::free(d);
            -1i32
        } else {
            DnsTransmit::free(d);
            -1i32
        }
    }

    unsafe fn firsttcp(d: *mut DnsTransmit) -> i32 {
        (*d).curserver = 0u32;
        DnsTransmit::thistcp(d)
    }

    unsafe fn thisudp(d: *mut DnsTransmit) -> i32 {
        let current_block;
        let mut ip: *const u8;
        DnsTransmit::socketfree(d);
        'loop1: loop {
            if !((*d).udploop < 4u32) {
                current_block = 2;
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
                    *(*d).query.offset(2isize) = random::random(256u32) as (u8);
                    *(*d).query.offset(3isize) = random::random(256u32) as (u8);
                    (*d).s1 = 1i32 + socket::udp();
                    if (*d).s1 == 0 {
                        current_block = 14;
                        break 'loop1;
                    }
                    if DnsTransmit::randombind(d) == -1i32 {
                        current_block = 13;
                        break 'loop1;
                    }
                    if socket::connect4((*d).s1 - 1i32, ip, 53u16) == 0i32 {
                        if libc::send(
                            (*d).s1 - 1i32,
                            (*d).query.offset(2isize) as (*const ::libc::c_void),
                            (*d).querylen.wrapping_sub(2u32) as (usize),
                            0i32,
                        ) as (usize) ==
                            (*d).querylen.wrapping_sub(2u32) as (usize)
                        {
                            current_block = 12;
                            break 'loop1;
                        }
                    }
                    DnsTransmit::socketfree(d);
                }
                (*d).curserver = (*d).curserver.wrapping_add(1u32);
            }
            (*d).udploop = (*d).udploop.wrapping_add(1u32);
            (*d).curserver = 0u32;
        }
        if current_block == 2 {
            DnsTransmit::free(d);
            -1i32
        } else if current_block == 12 {
            let mut now: TaiA = ::std::mem::zeroed();
            TaiA::now(&mut now as (*mut TaiA));
            TaiA::uint(
                &mut (*d).deadline as (*mut TaiA),
                TIMEOUTS[(*d).udploop as (usize)],
            );
            TaiA::add(
                &mut (*d).deadline as (*mut TaiA),
                &mut (*d).deadline as (*mut TaiA) as (*const TaiA),
                &mut now as (*mut TaiA) as (*const TaiA),
            );
            (*d).tcpstate = 0i32;
            0i32
        } else if current_block == 13 {
            DnsTransmit::free(d);
            -1i32
        } else {
            DnsTransmit::free(d);
            -1i32
        }
    }

    unsafe fn firstudp(d: *mut DnsTransmit) -> i32 {
        (*d).curserver = 0u32;
        DnsTransmit::thisudp(d)
    }

    unsafe fn nextudp(d: *mut DnsTransmit) -> i32 {
        (*d).curserver = (*d).curserver.wrapping_add(1u32);
        DnsTransmit::thisudp(d)
    }

    unsafe fn nexttcp(d: *mut DnsTransmit) -> i32 {
        (*d).curserver = (*d).curserver.wrapping_add(1u32);
        DnsTransmit::thistcp(d)
    }

    unsafe fn irrelevant(
        d: *mut DnsTransmit,
        buf: *const u8,
        len: u32,
    ) -> i32 {
        let mut out: [u8; 12] = [0u8; 12];
        let mut dn: *mut u8;
        let mut pos: u32;
        pos = packet::copy(buf, len, 0u32, out.as_mut_ptr(), 12u32);
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
            pos = packet::getname(buf, len, pos, &mut dn as (*mut *mut u8));
            (if pos == 0 {
                1i32
            } else if domain::equal(
                dn as (*const u8),
                (*d).query.offset(14isize) as (*const u8),
            ) == 0
            {
                alloc::free(dn);
                1i32
            } else {
                alloc::free(dn);
                pos = packet::copy(buf, len, pos, out.as_mut_ptr(), 4u32);
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

    unsafe fn serverwantstcp(buf: *const u8, len: u32) -> i32 {
        let mut out: [u8; 12] = [0u8; 12];
        if packet::copy(buf, len, 0u32, out.as_mut_ptr(), 12u32) == 0 {
            1i32
        } else if out[2usize] as (i32) & 2i32 != 0 {
            1i32
        } else {
            0i32
        }
    }

    unsafe fn serverfailed(buf: *const u8, len: u32) -> i32 {
        let mut out: [u8; 12] = [0u8; 12];
        let mut rcode: u32;
        if packet::copy(buf, len, 0u32, out.as_mut_ptr(), 12u32) == 0 {
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
}
