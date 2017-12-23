//! `dns/resolvconf.rs`: Functions for interacting with resolv.conf

use byte;
use ip4;
use libc;
use openreadclose::openreadclose;
use stralloc::StrAlloc;
use tai::Tai;
use taia::TaiA;

static mut DATA: StrAlloc = StrAlloc {
    s: 0i32 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

static mut OK: i32 = 0i32;

static mut USES: u32 = 0u32;

static mut DEADLINE: TaiA = TaiA {
    sec: Tai { x: 0usize },
    nano: 0usize,
    atto: 0usize,
};

static mut IP: [u8; 64] = [0u8; 64];

pub unsafe fn resolvconfip(s: *mut u8) -> i32 {
    let mut now: TaiA = ::std::mem::zeroed();
    TaiA::now(&mut now as (*mut TaiA));
    if TaiA::less(
        &mut DEADLINE as (*mut TaiA) as (*const TaiA),
        &mut now as (*mut TaiA) as (*const TaiA),
    ) != 0
    {
        OK = 0i32;
    }
    if USES == 0 {
        OK = 0i32;
    }
    if OK == 0 {
        if init(IP.as_mut_ptr()) == -1i32 {
            return -1i32;
        } else {
            TaiA::uint(&mut DEADLINE as (*mut TaiA), 600u32);
            TaiA::add(
                &mut DEADLINE as (*mut TaiA),
                &mut now as (*mut TaiA) as (*const TaiA),
                &mut DEADLINE as (*mut TaiA) as (*const TaiA),
            );
            USES = 10000u32;
            OK = 1i32;
        }
    }
    USES = USES.wrapping_sub(1u32);
    byte::copy(s, 64u32, IP.as_mut_ptr());
    0i32
}

unsafe fn init(ip: *mut u8) -> i32 {
    let mut current_block;
    let mut i: i32;
    let mut j: i32;
    let mut iplen: i32 = 0i32;
    let mut x = libc::getenv((*b"DNSCACHEIP\0").as_ptr() as *const libc::c_char);
    if !x.is_null() {
        current_block = 1;
    } else {
        current_block = 5;
    }
    'loop1: loop {
        if current_block == 1 {
            if !(iplen <= 60i32) {
                current_block = 5;
                continue;
            }
            if *x as (i32) == b'.' as (i32) {
                x = x.offset(1isize);
                current_block = 1;
            } else {
                i = ip4::scan(x as (*const u8), ip.offset(iplen as (isize))) as (i32);
                if i == 0 {
                    current_block = 5;
                    continue;
                }
                x = x.offset(i as (isize));
                iplen = iplen + 4i32;
                current_block = 1;
            }
        } else if iplen == 0 {
            current_block = 6;
            break;
        } else {
            current_block = 11;
            break;
        }
    }
    if current_block == 6 {
        i = openreadclose(
            (*b"/etc/resolv.conf\0").as_ptr(),
            &mut DATA as (*mut StrAlloc),
            64u32,
        );
        if i == -1i32 {
            return -1i32;
        } else if i != 0 {
            if StrAlloc::append(&mut DATA as (*mut StrAlloc), (*b"\n\0").as_ptr()) == 0 {
                return -1i32;
            } else {
                i = 0i32;
                j = 0i32;
                'loop10: loop {
                    if !(j as (u32) < DATA.len) {
                        break;
                    }
                    if *DATA.s.offset(j as (isize)) as (i32) == b'\n' as (i32) {
                        if byte::diff(
                            (*b"nameserver \0").as_ptr() as (*mut u8),
                            11u32,
                            DATA.s.offset(i as (isize)),
                        ) == 0 ||
                            byte::diff(
                                (*b"nameserver\t\0").as_ptr() as (*mut u8),
                                11u32,
                                DATA.s.offset(i as (isize)),
                            ) == 0
                        {
                            i = i + 10i32;
                            'loop17: loop {
                                if !(*DATA.s.offset(i as (isize)) as (i32) == b' ' as (i32) ||
                                         *DATA.s.offset(i as (isize)) as (i32) == b'\t' as (i32))
                                {
                                    break;
                                }
                                i = i + 1;
                            }
                            if iplen <= 60i32 {
                                if ip4::scan(
                                    DATA.s.offset(i as (isize)) as (*const u8),
                                    ip.offset(iplen as (isize)),
                                ) != 0
                                {
                                    if byte::diff(
                                        ip.offset(iplen as (isize)),
                                        4u32,
                                        (*b"\0\0\0\0\0").as_ptr() as (*mut u8),
                                    ) == 0
                                    {
                                        byte::copy(
                                            ip.offset(iplen as (isize)),
                                            4u32,
                                            (*b"\x7F\0\0\x01\0").as_ptr() as (*mut u8),
                                        );
                                    }
                                    iplen = iplen + 4i32;
                                }
                            }
                        }
                        i = j + 1i32;
                    }
                    j = j + 1;
                }
            }
        }
    }
    if iplen == 0 {
        byte::copy(ip, 4u32, (*b"\x7F\0\0\x01\0").as_ptr() as (*mut u8));
        iplen = 4i32;
    }
    byte::zero(ip.offset(iplen as (isize)), (64i32 - iplen) as (u32));
    0i32
}
