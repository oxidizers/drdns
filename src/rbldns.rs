use byte;
use libc;
use uint32;

extern "C" {
    fn cdb_find(arg1: *mut cdb, arg2: *const u8, arg3: u32) -> i32;
    fn cdb_free(arg1: *mut cdb);
    fn cdb_init(arg1: *mut cdb, fd: i32);
    fn cdb_read(arg1: *mut cdb, arg2: *mut u8, arg3: u32, arg4: u32) -> i32;
    fn close(arg1: i32) -> i32;
    fn dd(arg1: *const u8, arg2: *const u8, arg3: *mut u8) -> i32;
    fn dns_domain_fromdot(arg1: *mut *mut u8, arg2: *const u8, arg3: u32) -> i32;
    fn env_get(arg1: *const u8) -> *mut u8;
    fn ip4_fmt(arg1: *mut u8, arg2: *const u8) -> u32;
    fn open_read(arg1: *const u8) -> i32;
    static mut response: *mut u8;
    fn response_addbytes(arg1: *const u8, arg2: u32) -> i32;
    fn response_nxdomain();
    fn response_rfinish(arg1: i32);
    fn response_rstart(arg1: *const u8, arg2: *const u8, arg3: u32) -> i32;
    fn strerr_die(
        arg1: i32,
        arg2: *const u8,
        arg3: *const u8,
        arg4: *const u8,
        arg5: *const u8,
        arg6: *const u8,
        arg7: *const u8,
        arg8: *const strerr,
    );
}

static mut base: *mut u8 = 0 as (*mut u8);

#[derive(Copy)]
#[repr(C)]
pub struct cdb {
    pub map: *mut u8,
    pub fd: i32,
    pub size: u32,
    pub loopvar: u32,
    pub khash: u32,
    pub kpos: u32,
    pub hpos: u32,
    pub hslots: u32,
    pub dpos: u32,
    pub dlen: u32,
}

impl Clone for cdb {
    fn clone(&self) -> Self {
        *self
    }
}

static mut c: cdb = cdb {
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

static mut key: [u8; 5] = [0u8; 5];

static mut data: [u8; 120] = [0u8; 120];

unsafe extern "C" fn doit(mut q: *mut u8, mut qtype: *mut u8) -> i32 {
    let mut _currentBlock;
    let mut flaga: i32;
    let mut flagtxt: i32;
    let mut ch: u8;
    let mut reverseip: [u8; 4];
    let mut ip: [u8; 4];
    let mut ipnum: u32;
    let mut r: i32;
    let mut dlen: u32;
    let mut i: i32;
    flaga = (byte::diff(qtype, 2u32, (*b"\0\x01\0").as_ptr() as (*mut u8)) == 0) as (i32);
    flagtxt = (byte::diff(qtype, 2u32, (*b"\0\x10\0").as_ptr() as (*mut u8)) == 0) as (i32);
    if byte::diff(qtype, 2u32, (*b"\0\xFF\0").as_ptr() as (*mut u8)) == 0 {
        flaga = {
            flagtxt = 1i32;
            flagtxt
        };
    }
    if !(flaga == 0 && (flagtxt == 0)) {
        if !(dd(
            q as (*const u8),
            base as (*const u8),
            reverseip.as_mut_ptr(),
        ) != 4i32)
        {
            uint32::unpack(
                reverseip.as_mut_ptr() as (*const u8),
                &mut ipnum as (*mut u32),
            );
            uint32::pack_big(ip.as_mut_ptr(), ipnum);
            i = 0i32;
            'loop5: loop {
                if !(i <= 24i32) {
                    _currentBlock = 9;
                    break;
                }
                ipnum = ipnum >> i;
                ipnum = ipnum << i;
                uint32::pack_big(key.as_mut_ptr(), ipnum);
                key[4usize] = (32i32 - i) as (u8);
                r = cdb_find(&mut c as (*mut cdb), key.as_mut_ptr() as (*const u8), 5u32);
                if r == -1i32 {
                    _currentBlock = 36;
                    break;
                }
                if r != 0 {
                    _currentBlock = 9;
                    break;
                }
                i = i + 1;
            }
            if _currentBlock == 9 {
                if r == 0 {
                    response_nxdomain();
                    return 1i32;
                } else {
                    r = cdb_find(&mut c as (*mut cdb), (*b"\0").as_ptr(), 0u32);
                    if r == -1i32 {
                        return 0i32;
                    } else {
                        if r != 0 &&
                            ({
                                 dlen = (*(&mut c as (*mut cdb))).dlen;
                                 dlen
                             } >= 4u32)
                        {
                            if dlen > 100u32 {
                                dlen = 100u32;
                            }
                            if cdb_read(
                                &mut c as (*mut cdb),
                                data.as_mut_ptr(),
                                dlen,
                                (*(&mut c as (*mut cdb))).dpos,
                            ) == -1i32
                            {
                                return 0i32;
                            }
                        } else {
                            dlen = 12u32;
                            byte::copy(
                                data.as_mut_ptr(),
                                dlen,
                                (*b"\x7F\0\0\x02Listed $\0").as_ptr() as (*mut u8),
                            );
                        }
                        if dlen >= 5u32 &&
                            (data[dlen.wrapping_sub(1u32) as (usize)] as (i32) == b'$' as (i32))
                        {
                            dlen = dlen.wrapping_sub(1u32);
                            dlen = dlen.wrapping_add(ip4_fmt(
                                data.as_mut_ptr().offset(dlen as (isize)),
                                ip.as_mut_ptr() as (*const u8),
                            ));
                        }
                        if flaga != 0 {
                            if response_rstart(
                                q as (*const u8),
                                (*b"\0\x01\0").as_ptr(),
                                2048u32,
                            ) == 0
                            {
                                return 0i32;
                            } else if response_addbytes(
                                data.as_mut_ptr() as (*const u8),
                                4u32,
                            ) == 0
                            {
                                return 0i32;
                            } else {
                                response_rfinish(6i32);
                            }
                        }
                        if flagtxt != 0 {
                            if response_rstart(
                                q as (*const u8),
                                (*b"\0\x10\0").as_ptr(),
                                2048u32,
                            ) == 0
                            {
                                return 0i32;
                            } else {
                                ch = dlen.wrapping_sub(4u32) as (u8);
                                if response_addbytes(
                                    &mut ch as (*mut u8) as (*const u8),
                                    1u32,
                                ) == 0
                                {
                                    return 0i32;
                                } else if response_addbytes(
                                    data.as_mut_ptr().offset(4isize) as (*const u8),
                                    dlen.wrapping_sub(4u32),
                                ) == 0
                                {
                                    return 0i32;
                                } else {
                                    response_rfinish(6i32);
                                }
                            }
                        }
                        return 1i32;
                    }
                }
            } else {
                return 0i32;
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

#[no_mangle]
pub unsafe extern "C" fn respond(mut q: *mut u8, mut qtype: *mut u8, mut ip: *mut u8) -> i32 {
    let mut fd: i32;
    let mut result: i32;
    fd = open_read((*b"data.cdb\0").as_ptr());
    if fd == -1i32 {
        0i32
    } else {
        cdb_init(&mut c as (*mut cdb), fd);
        result = doit(q, qtype);
        cdb_free(&mut c as (*mut cdb));
        close(fd);
        result
    }
}

#[no_mangle]
pub static mut fatal: *const u8 = (*b"rbldns: fatal: \0").as_ptr();

#[no_mangle]
pub static mut starting: *const u8 = (*b"starting rbldns\n\0").as_ptr();

#[derive(Copy)]
#[repr(C)]
pub struct strerr {
    pub who: *mut strerr,
    pub x: *const u8,
    pub y: *const u8,
    pub z: *const u8,
}

impl Clone for strerr {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn initialize() {
    let mut x: *mut u8;
    x = env_get((*b"BASE\0").as_ptr());
    if x.is_null() {
        strerr_die(
            111i32,
            fatal,
            (*b"$BASE not set\0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const strerr),
        );
    }
    if dns_domain_fromdot(
        &mut base as (*mut *mut u8),
        x as (*const u8),
        libc::strlen(x as *const i8) as u32,
    ) == 0
    {
        strerr_die(
            111i32,
            fatal,
            (*b"unable to parse $BASE\0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const strerr),
        );
    }
}
