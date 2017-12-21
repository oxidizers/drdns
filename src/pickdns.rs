use byte;
use cdb::Cdb;

extern "C" {
    fn case_lowerb(arg1: *mut u8, arg2: u32);
    fn close(arg1: i32) -> i32;
    fn dns_domain_length(arg1: *const u8) -> u32;
    fn dns_random_init(arg1: *const u8);
    fn dns_sortip(arg1: *mut u8, arg2: u32);
    fn open_read(arg1: *const u8) -> i32;
    static mut response: *mut u8;
    fn response_addbytes(arg1: *const u8, arg2: u32) -> i32;
    fn response_rfinish(arg1: i32);
    fn response_rstart(arg1: *const u8, arg2: *const u8, arg3: u32) -> i32;
}

#[no_mangle]
pub static mut fatal: *const u8 = (*b"pickdns: fatal: \0").as_ptr();

#[no_mangle]
pub static mut starting: *const u8 = (*b"starting pickdns\n\0").as_ptr();

static mut seed: [u8; 128] = [0u8; 128];

#[no_mangle]
pub unsafe extern "C" fn initialize() {
    dns_random_init(seed.as_mut_ptr() as (*const u8));
}

static mut c: Cdb = Cdb {
    map: 0 as (*mut u8),
    fd: 0i32,
    size: 0u32,
    loopvar: 0u32,
    khash: 0u32,
    kpos: 0u32,
    hpos: 0u32,
    hslots: 0u32,
    dpos: 0u32,
    dlen: 0u32,
};

static mut key: [u8; 258] = [0u8; 258];

static mut data: [u8; 512] = [0u8; 512];

unsafe extern "C" fn doit(mut q: *mut u8, mut qtype: *mut u8, mut ip: *mut u8) -> i32 {
    let mut _currentBlock;
    let mut r: i32;
    let mut dlen: u32;
    let mut qlen: u32;
    let mut flaga: i32;
    let mut flagmx: i32;
    qlen = dns_domain_length(q as (*const u8));
    if qlen > 255u32 {
        0i32
    } else {
        flaga = (byte::diff(qtype, 2u32, (*b"\0\x01\0").as_ptr() as (*mut u8)) == 0) as (i32);
        flagmx = (byte::diff(qtype, 2u32, (*b"\0\x0F\0").as_ptr() as (*mut u8)) == 0) as (i32);
        if byte::diff(qtype, 2u32, (*b"\0\xFF\0").as_ptr() as (*mut u8)) == 0 {
            flaga = {
                flagmx = 1i32;
                flagmx
            };
        }
        if !(flaga == 0 && (flagmx == 0)) {
            key[0usize] = b'%';
            byte::copy(key.as_mut_ptr().offset(1isize), 4u32, ip);
            r = Cdb::find(&mut c as (*mut Cdb), key.as_mut_ptr() as (*const u8), 5u32);
            if r == 0 {
                r = Cdb::find(&mut c as (*mut Cdb), key.as_mut_ptr() as (*const u8), 4u32);
            }
            if r == 0 {
                r = Cdb::find(&mut c as (*mut Cdb), key.as_mut_ptr() as (*const u8), 3u32);
            }
            if r == 0 {
                r = Cdb::find(&mut c as (*mut Cdb), key.as_mut_ptr() as (*const u8), 2u32);
            }
            if r == -1i32 {
                return 0i32;
            } else {
                key[0usize] = b'+';
                byte::zero(key.as_mut_ptr().offset(1isize), 2u32);
                if r != 0 && ((*(&mut c as (*mut Cdb))).dlen == 2u32) {
                    if Cdb::read(
                        &mut c as (*mut Cdb),
                        key.as_mut_ptr().offset(1isize),
                        2u32,
                        (*(&mut c as (*mut Cdb))).dpos,
                    ) == -1i32
                    {
                        return 0i32;
                    }
                }
                byte::copy(key.as_mut_ptr().offset(3isize), qlen, q);
                case_lowerb(key.as_mut_ptr().offset(3isize), qlen.wrapping_add(3u32));
                r = Cdb::find(
                    &mut c as (*mut Cdb),
                    key.as_mut_ptr() as (*const u8),
                    qlen.wrapping_add(3u32),
                );
                if r == 0 {
                    byte::zero(key.as_mut_ptr().offset(1isize), 2u32);
                    r = Cdb::find(
                        &mut c as (*mut Cdb),
                        key.as_mut_ptr() as (*const u8),
                        qlen.wrapping_add(3u32),
                    );
                }
                if !(r == 0) {
                    if r == -1i32 {
                        return 0i32;
                    } else {
                        dlen = (*(&mut c as (*mut Cdb))).dlen;
                        if dlen > 512u32 {
                            dlen = 512u32;
                        }
                        if Cdb::read(
                            &mut c as (*mut Cdb),
                            data.as_mut_ptr(),
                            dlen,
                            (*(&mut c as (*mut Cdb))).dpos,
                        ) == -1i32
                        {
                            return 0i32;
                        } else {
                            if flaga != 0 {
                                dns_sortip(data.as_mut_ptr(), dlen);
                                if dlen > 12u32 {
                                    dlen = 12u32;
                                }
                                'loop23: loop {
                                    if !(dlen >= 4u32) {
                                        _currentBlock = 24;
                                        break;
                                    }
                                    dlen = dlen.wrapping_sub(4u32);
                                    if response_rstart(
                                        q as (*const u8),
                                        (*b"\0\x01\0").as_ptr(),
                                        5u32,
                                    ) == 0
                                    {
                                        _currentBlock = 29;
                                        break;
                                    }
                                    if response_addbytes(
                                        data.as_mut_ptr().offset(dlen as (isize)) as
                                            (*const u8),
                                        4u32,
                                    ) == 0
                                    {
                                        _currentBlock = 28;
                                        break;
                                    }
                                    response_rfinish(6i32);
                                }
                                if _currentBlock == 24 {
                                } else if _currentBlock == 28 {
                                    return 0i32;
                                } else {
                                    return 0i32;
                                }
                            }
                            return 1i32;
                        }
                    }
                }
            }
        }
        let _rhs = !4i32;
        let _lhs = &mut *response.offset(2isize);
        *_lhs = (*_lhs as (i32) & _rhs) as (u8);
        let _rhs = !15i32;
        let _lhs = &mut *response.offset(3isize);
        *_lhs = (*_lhs as (i32) & _rhs) as (u8);
        let _rhs = 5i32;
        let _lhs = &mut *response.offset(3isize);
        *_lhs = (*_lhs as (i32) | _rhs) as (u8);
        1i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn respond(mut q: *mut u8, mut qtype: *mut u8, mut ip: *mut u8) -> i32 {
    let mut fd: i32;
    let mut result: i32;
    fd = open_read((*b"data.cdb\0").as_ptr());
    if fd == -1i32 {
        0i32
    } else {
        Cdb::init(&mut c as (*mut Cdb), fd);
        result = doit(q, qtype, ip);
        Cdb::free(&mut c as (*mut Cdb));
        close(fd);
        result
    }
}
