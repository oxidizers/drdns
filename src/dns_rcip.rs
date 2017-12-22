use byte;
use libc;
use stralloc::StrAlloc;
use tai::Tai;
use taia::TaiA;

extern "C" {
    fn ip4_scan(arg1: *const u8, arg2: *mut u8) -> u32;
    fn openreadclose(arg1: *const u8, arg2: *mut StrAlloc, arg3: u32) -> i32;
}

static mut data: StrAlloc = StrAlloc {
    s: 0i32 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

static mut ok: i32 = 0i32;

static mut uses: u32 = 0u32;

static mut deadline: TaiA = TaiA {
    sec: Tai { x: 0usize },
    nano: 0usize,
    atto: 0usize,
};

static mut ip: [u8; 64] = [0u8; 64];

unsafe extern "C" fn init(mut ip: *mut u8) -> i32 {
    let mut _currentBlock;
    let mut i: i32;
    let mut j: i32;
    let mut iplen: i32 = 0i32;
    let mut x: *mut u8;
    x = libc::getenv((*b"DNSCACHEIP\0").as_ptr() as *const libc::c_char);
    if !x.is_null() {
        _currentBlock = 1;
    } else {
        _currentBlock = 5;
    }
    'loop1: loop {
        if _currentBlock == 1 {
            if !(iplen <= 60i32) {
                _currentBlock = 5;
                continue;
            }
            if *x as (i32) == b'.' as (i32) {
                x = x.offset(1isize);
                _currentBlock = 1;
            } else {
                i = ip4_scan(x as (*const u8), ip.offset(iplen as (isize))) as (i32);
                if i == 0 {
                    _currentBlock = 5;
                    continue;
                }
                x = x.offset(i as (isize));
                iplen = iplen + 4i32;
                _currentBlock = 1;
            }
        } else if iplen == 0 {
            _currentBlock = 6;
            break;
        } else {
            _currentBlock = 11;
            break;
        }
    }
    if _currentBlock == 6 {
        i = openreadclose(
            (*b"/etc/resolv.conf\0").as_ptr(),
            &mut data as (*mut StrAlloc),
            64u32,
        );
        if i == -1i32 {
            return -1i32;
        } else if i != 0 {
            if StrAlloc::append(&mut data as (*mut StrAlloc), (*b"\n\0").as_ptr()) == 0 {
                return -1i32;
            } else {
                i = 0i32;
                j = 0i32;
                'loop10: loop {
                    if !(j as (u32) < data.len) {
                        break;
                    }
                    if *data.s.offset(j as (isize)) as (i32) == b'\n' as (i32) {
                        if byte::diff(
                            (*b"nameserver \0").as_ptr() as (*mut u8),
                            11u32,
                            data.s.offset(i as (isize)),
                        ) == 0 ||
                            byte::diff(
                                (*b"nameserver\t\0").as_ptr() as (*mut u8),
                                11u32,
                                data.s.offset(i as (isize)),
                            ) == 0
                        {
                            i = i + 10i32;
                            'loop17: loop {
                                if !(*data.s.offset(i as (isize)) as (i32) == b' ' as (i32) ||
                                         *data.s.offset(i as (isize)) as (i32) == b'\t' as (i32))
                                {
                                    break;
                                }
                                i = i + 1;
                            }
                            if iplen <= 60i32 {
                                if ip4_scan(
                                    data.s.offset(i as (isize)) as (*const u8),
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

#[no_mangle]
pub unsafe extern "C" fn dns_resolvconfip(mut s: *mut u8) -> i32 {
    let mut now: TaiA;
    TaiA::now(&mut now as (*mut TaiA));
    if TaiA::less(
        &mut deadline as (*mut TaiA) as (*const TaiA),
        &mut now as (*mut TaiA) as (*const TaiA),
    ) != 0
    {
        ok = 0i32;
    }
    if uses == 0 {
        ok = 0i32;
    }
    if ok == 0 {
        if init(ip.as_mut_ptr()) == -1i32 {
            return -1i32;
        } else {
            TaiA::uint(&mut deadline as (*mut TaiA), 600u32);
            TaiA::add(
                &mut deadline as (*mut TaiA),
                &mut now as (*mut TaiA) as (*const TaiA),
                &mut deadline as (*mut TaiA) as (*const TaiA),
            );
            uses = 10000u32;
            ok = 1i32;
        }
    }
    uses = uses.wrapping_sub(1u32);
    byte::copy(s, 64u32, ip.as_mut_ptr());
    0i32
}
