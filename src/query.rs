use alloc;
use byte;
use libc;

extern "C" {
    fn cache_get(arg1: *const u8, arg2: u32, arg3: *mut u32, arg4: *mut u32) -> *mut u8;
    fn cache_set(arg1: *const u8, arg2: u32, arg3: *const u8, arg4: u32, arg5: u32);
    fn case_diffb(arg1: *const u8, arg2: u32, arg3: *const u8) -> i32;
    fn case_lowerb(arg1: *mut u8, arg2: u32);
    fn dd(arg1: *const u8, arg2: *const u8, arg3: *mut u8) -> i32;
    fn dns_domain_copy(arg1: *mut *mut u8, arg2: *const u8) -> i32;
    fn dns_domain_equal(arg1: *const u8, arg2: *const u8) -> i32;
    fn dns_domain_free(arg1: *mut *mut u8);
    fn dns_domain_length(arg1: *const u8) -> u32;
    fn dns_domain_suffix(arg1: *const u8, arg2: *const u8) -> i32;
    fn dns_domain_suffixpos(arg1: *const u8, arg2: *const u8) -> u32;
    fn dns_packet_copy(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut u8, arg5: u32) -> u32;
    fn dns_packet_getname(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut *mut u8) -> u32;
    fn dns_packet_skipname(arg1: *const u8, arg2: u32, arg3: u32) -> u32;
    fn dns_sortip(arg1: *mut u8, arg2: u32);
    fn dns_transmit_free(arg1: *mut dns_transmit);
    fn dns_transmit_get(arg1: *mut dns_transmit, arg2: *const pollfd, arg3: *const taia) -> i32;
    fn dns_transmit_io(arg1: *mut dns_transmit, arg2: *mut pollfd, arg3: *mut taia);
    fn dns_transmit_start(
        arg1: *mut dns_transmit,
        arg2: *const u8,
        arg3: i32,
        arg4: *const u8,
        arg5: *const u8,
        arg6: *const u8,
    ) -> i32;
    static mut errno: i32;
    fn log_cachedanswer(arg1: *const u8, arg2: *const u8);
    fn log_cachedcname(arg1: *const u8, arg2: *const u8);
    fn log_cachedns(arg1: *const u8, arg2: *const u8);
    fn log_cachednxdomain(arg1: *const u8);
    fn log_lame(arg1: *const u8, arg2: *const u8, arg3: *const u8);
    fn log_nodata(arg1: *const u8, arg2: *const u8, arg3: *const u8, arg4: u32);
    fn log_nxdomain(arg1: *const u8, arg2: *const u8, arg3: u32);
    fn log_rr(
        arg1: *const u8,
        arg2: *const u8,
        arg3: *const u8,
        arg4: *const u8,
        arg5: u32,
        arg6: u32,
    );
    fn log_rrcname(arg1: *const u8, arg2: *const u8, arg3: *const u8, arg4: u32);
    fn log_rrmx(arg1: *const u8, arg2: *const u8, arg3: *const u8, arg4: *const u8, arg5: u32);
    fn log_rrns(arg1: *const u8, arg2: *const u8, arg3: *const u8, arg4: u32);
    fn log_rrptr(arg1: *const u8, arg2: *const u8, arg3: *const u8, arg4: u32);
    fn log_rrsoa(
        arg1: *const u8,
        arg2: *const u8,
        arg3: *const u8,
        arg4: *const u8,
        arg5: *const u8,
        arg6: u32,
    );
    fn log_servfail(arg1: *const u8);
    fn log_stats();
    fn log_tx(arg1: *const u8, arg2: *const u8, arg3: *const u8, arg4: *const u8, arg5: u32);
    fn response_addbytes(arg1: *const u8, arg2: u32) -> i32;
    fn response_addname(arg1: *const u8) -> i32;
    fn response_cname(arg1: *const u8, arg2: *const u8, arg3: u32) -> i32;
    fn response_nxdomain();
    fn response_query(arg1: *const u8, arg2: *const u8, arg3: *const u8) -> i32;
    fn response_rfinish(arg1: i32);
    fn response_rstart(arg1: *const u8, arg2: *const u8, arg3: u32) -> i32;
    fn response_servfail();
    fn roots(arg1: *mut u8, arg2: *mut u8) -> i32;
    fn roots_same(arg1: *mut u8, arg2: *mut u8) -> i32;
    fn uint16_unpack_big(arg1: *const u8, arg2: *mut u16);
    fn uint32_unpack_big(arg1: *const u8, arg2: *mut u32);
}

static mut flagforwardonly: i32 = 0i32;

#[no_mangle]
pub unsafe extern "C" fn query_forwardonly() {
    flagforwardonly = 1i32;
}

static mut save_buf: [u8; 8192] = [0u8; 8192];

static mut save_len: u32 = 0u32;

static mut save_ok: u32 = 0u32;

static mut t1: *mut u8 = 0i32 as (*mut u8);

static mut t2: *mut u8 = 0i32 as (*mut u8);

static mut t3: *mut u8 = 0i32 as (*mut u8);

static mut cname: *mut u8 = 0i32 as (*mut u8);

static mut referral: *mut u8 = 0i32 as (*mut u8);

static mut records: *mut u32 = 0i32 as (*mut u32);

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

#[derive(Copy)]
#[repr(C)]
pub struct query {
    pub loopvar: u32,
    pub level: u32,
    pub name: [*mut u8; 5],
    pub control: [*mut u8; 5],
    pub ns: [[*mut u8; 16]; 5],
    pub servers: [[u8; 64]; 5],
    pub alias: [*mut u8; 16],
    pub aliasttl: [u32; 16],
    pub localip: [u8; 4],
    pub type_: [u8; 2],
    pub class: [u8; 2],
    pub dt: dns_transmit,
}

impl Clone for query {
    fn clone(&self) -> Self {
        *self
    }
}

unsafe extern "C" fn cleanup(mut z: *mut query) {
    let mut j: i32;
    let mut k: i32;
    dns_transmit_free(&mut (*z).dt as (*mut dns_transmit));
    j = 0i32;
    'loop1: loop {
        if !(j < 16i32) {
            break;
        }
        dns_domain_free(&mut (*z).alias[j as (usize)] as (*mut *mut u8));
        j = j + 1;
    }
    j = 0i32;
    'loop3: loop {
        if !(j < 5i32) {
            break;
        }
        dns_domain_free(&mut (*z).name[j as (usize)] as (*mut *mut u8));
        k = 0i32;
        'loop6: loop {
            if !(k < 16i32) {
                break;
            }
            dns_domain_free(&mut (*z).ns[j as (usize)][k as (usize)] as (*mut *mut u8));
            k = k + 1;
        }
        j = j + 1;
    }
}

unsafe extern "C" fn globalip(mut d: *mut u8, mut ip: *mut u8) -> i32 {
    if dns_domain_equal(d as (*const u8), (*b"\tlocalhost\0\0").as_ptr()) != 0 {
        byte::copy(ip, 4u32, (*b"\x7F\0\0\x01\0").as_ptr() as (*mut u8));
        1i32
    } else if dd(d as (*const u8), (*b"\0").as_ptr(), ip) == 4i32 {
        1i32
    } else {
        0i32
    }
}

unsafe extern "C" fn rqa(mut z: *mut query) -> i32 {
    let mut _currentBlock;
    let mut i: i32;
    i = 16i32 - 1i32;
    'loop1: loop {
        if !(i >= 0i32) {
            _currentBlock = 2;
            break;
        }
        if !(*z).alias[i as (usize)].is_null() {
            _currentBlock = 7;
            break;
        }
        i = i - 1;
    }
    if _currentBlock == 2 {
        (if response_query(
            (*z).name[0usize] as (*const u8),
            (*z).type_.as_mut_ptr() as (*const u8),
            (*z).class.as_mut_ptr() as (*const u8),
        ) == 0
        {
             0i32
         } else {
             1i32
         })
    } else if response_query(
        (*z).alias[i as (usize)] as (*const u8),
        (*z).type_.as_mut_ptr() as (*const u8),
        (*z).class.as_mut_ptr() as (*const u8),
    ) == 0
    {
        0i32
    } else {
        'loop8: loop {
            if !(i > 0i32) {
                _currentBlock = 9;
                break;
            }
            if response_cname(
                (*z).alias[i as (usize)] as (*const u8),
                (*z).alias[(i - 1i32) as (usize)] as (*const u8),
                (*z).aliasttl[i as (usize)],
            ) == 0
            {
                _currentBlock = 14;
                break;
            }
            i = i - 1;
        }
        (if _currentBlock == 9 {
             (if response_cname(
                (*z).alias[0usize] as (*const u8),
                (*z).name[0usize] as (*const u8),
                (*z).aliasttl[0usize],
            ) == 0
            {
                  0i32
              } else {
                  1i32
              })
         } else {
             0i32
         })
    }
}

unsafe extern "C" fn typematch(mut rtype: *const u8, mut qtype: *const u8) -> i32 {
    (byte::diff(qtype as (*mut u8), 2u32, rtype as (*mut u8)) == 0 ||
         byte::diff(
            qtype as (*mut u8),
            2u32,
            (*b"\0\xFF\0").as_ptr() as (*mut u8),
        ) == 0) as (i32)
}

unsafe extern "C" fn ttlget(mut buf: *mut u8) -> u32 {
    let mut ttl: u32;
    uint32_unpack_big(buf as (*const u8), &mut ttl as (*mut u32));
    if ttl > 1000000000u32 {
        0u32
    } else if ttl > 604800u32 {
        604800u32
    } else {
        ttl
    }
}

unsafe extern "C" fn smaller(mut buf: *mut u8, mut len: u32, mut pos1: u32, mut pos2: u32) -> i32 {
    let mut header1: [u8; 12];
    let mut header2: [u8; 12];
    let mut r: i32;
    let mut len1: u32;
    let mut len2: u32;
    pos1 = dns_packet_getname(buf as (*const u8), len, pos1, &mut t1 as (*mut *mut u8));
    dns_packet_copy(buf as (*const u8), len, pos1, header1.as_mut_ptr(), 10u32);
    pos2 = dns_packet_getname(buf as (*const u8), len, pos2, &mut t2 as (*mut *mut u8));
    dns_packet_copy(buf as (*const u8), len, pos2, header2.as_mut_ptr(), 10u32);
    r = byte::diff(header1.as_mut_ptr(), 4u32, header2.as_mut_ptr());
    if r < 0i32 {
        1i32
    } else if r > 0i32 {
        0i32
    } else {
        len1 = dns_domain_length(t1 as (*const u8));
        len2 = dns_domain_length(t2 as (*const u8));
        (if len1 < len2 {
             1i32
         } else if len1 > len2 {
             0i32
         } else {
             r = case_diffb(t1 as (*const u8), len1, t2 as (*const u8));
             (if r < 0i32 {
                  1i32
              } else if r > 0i32 {
                  0i32
              } else if pos1 < pos2 {
                  1i32
              } else {
                  0i32
              })
         })
    }
}

unsafe extern "C" fn save_start() {
    save_len = 0u32;
    save_ok = 1u32;
}

unsafe extern "C" fn save_data(mut buf: *const u8, mut len: u32) {
    if save_ok == 0 {
    } else if len as (usize) >
               ::std::mem::size_of::<[u8; 8192]>().wrapping_sub(save_len as (usize))
    {
        save_ok = 0u32;
    } else {
        byte::copy(
            save_buf.as_mut_ptr().offset(save_len as (isize)),
            len,
            buf as (*mut u8),
        );
        save_len = save_len.wrapping_add(len);
    }
}

unsafe extern "C" fn cachegeneric(
    mut type_: *const u8,
    mut d: *const u8,
    mut data: *const u8,
    mut datalen: u32,
    mut ttl: u32,
) {
    let mut len: u32;
    let mut key: [u8; 257];
    len = dns_domain_length(d);
    if len > 255u32 {
    } else {
        byte::copy(key.as_mut_ptr(), 2u32, type_ as (*mut u8));
        byte::copy(key.as_mut_ptr().offset(2isize), len, d as (*mut u8));
        case_lowerb(key.as_mut_ptr().offset(2isize), len);
        cache_set(
            key.as_mut_ptr() as (*const u8),
            len.wrapping_add(2u32),
            data,
            datalen,
            ttl,
        );
    }
}

unsafe extern "C" fn save_finish(mut type_: *const u8, mut d: *const u8, mut ttl: u32) {
    if save_ok == 0 {
    } else {
        cachegeneric(
            type_,
            d,
            save_buf.as_mut_ptr() as (*const u8),
            save_len,
            ttl,
        );
    }
}

unsafe extern "C" fn doit(mut z: *mut query, mut state: i32) -> i32 {
    let mut _currentBlock;
    let mut key: [u8; 257];
    let mut cached: *mut u8;
    let mut cachedlen: u32;
    let mut buf: *mut u8;
    let mut len: u32;
    let mut whichserver: *const u8;
    let mut header: [u8; 12];
    let mut misc: [u8; 20];
    let mut rcode: u32;
    let mut posanswers: u32;
    let mut numanswers: u16;
    let mut posauthority: u32;
    let mut numauthority: u16;
    let mut posglue: u32;
    let mut numglue: u16;
    let mut pos: u32;
    let mut pos2: u32;
    let mut datalen: u16;
    let mut control: *mut u8;
    let mut d: *mut u8;
    let mut dtype: *const u8;
    let mut dlen: u32;
    let mut flagout: i32;
    let mut flagcname: i32;
    let mut flagreferral: i32;
    let mut flagsoa: i32;
    let mut ttl: u32;
    let mut soattl: u32;
    let mut cnamettl: u32;
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut p: i32;
    let mut q: i32;
    errno = libc::EIO;
    if state == 1i32 {
        if {
            (*z).loopvar = (*z).loopvar.wrapping_add(1u32);
            (*z).loopvar
        } == 100u32
        {
            _currentBlock = 348;
        } else {
            buf = (*z).dt.packet;
            len = (*z).dt.packetlen;
            whichserver = (*z).dt.servers.offset(
                4u32.wrapping_mul((*z).dt.curserver) as
                    (isize),
            );
            control = (*z).control[(*z).level as (usize)];
            d = (*z).name[(*z).level as (usize)];
            dtype = if (*z).level != 0 {
                (*b"\0\x01\0").as_ptr()
            } else {
                (*z).type_.as_mut_ptr() as (*const u8)
            };
            pos = dns_packet_copy(buf as (*const u8), len, 0u32, header.as_mut_ptr(), 12u32);
            if pos == 0 {
                _currentBlock = 348;
            } else {
                pos = dns_packet_skipname(buf as (*const u8), len, pos);
                if pos == 0 {
                    _currentBlock = 348;
                } else {
                    pos = pos.wrapping_add(4u32);
                    posanswers = pos;
                    uint16_unpack_big(
                        header.as_mut_ptr().offset(6isize) as (*const u8),
                        &mut numanswers as (*mut u16),
                    );
                    uint16_unpack_big(
                        header.as_mut_ptr().offset(8isize) as (*const u8),
                        &mut numauthority as (*mut u16),
                    );
                    uint16_unpack_big(
                        header.as_mut_ptr().offset(10isize) as (*const u8),
                        &mut numglue as (*mut u16),
                    );
                    rcode = (header[3usize] as (i32) & 15i32) as (u32);
                    if rcode != 0 && (rcode != 3u32) {
                        _currentBlock = 348;
                    } else {
                        flagout = 0i32;
                        flagcname = 0i32;
                        flagreferral = 0i32;
                        flagsoa = 0i32;
                        soattl = 0u32;
                        cnamettl = 0u32;
                        j = 0i32;
                        'loop8: loop {
                            if !(j < numanswers as (i32)) {
                                _currentBlock = 9;
                                break;
                            }
                            pos = dns_packet_getname(
                                buf as (*const u8),
                                len,
                                pos,
                                &mut t1 as (*mut *mut u8),
                            );
                            if pos == 0 {
                                _currentBlock = 348;
                                break;
                            }
                            pos = dns_packet_copy(
                                buf as (*const u8),
                                len,
                                pos,
                                header.as_mut_ptr(),
                                10u32,
                            );
                            if pos == 0 {
                                _currentBlock = 348;
                                break;
                            }
                            if dns_domain_equal(t1 as (*const u8), d as (*const u8)) != 0 {
                                if byte::diff(
                                    header.as_mut_ptr().offset(2isize),
                                    2u32,
                                    (*b"\0\x01\0").as_ptr() as (*mut u8),
                                ) == 0
                                {
                                    if typematch(header.as_mut_ptr() as (*const u8), dtype) != 0 {
                                        flagout = 1i32;
                                    } else if typematch(
                                        header.as_mut_ptr() as (*const u8),
                                        (*b"\0\x05\0").as_ptr(),
                                    ) != 0
                                    {
                                        if dns_packet_getname(
                                            buf as (*const u8),
                                            len,
                                            pos,
                                            &mut cname as (*mut *mut u8),
                                        ) == 0
                                        {
                                            _currentBlock = 348;
                                            break;
                                        }
                                        flagcname = 1i32;
                                        cnamettl = ttlget(header.as_mut_ptr().offset(4isize));
                                    }
                                }
                            }
                            uint16_unpack_big(
                                header.as_mut_ptr().offset(8isize) as (*const u8),
                                &mut datalen as (*mut u16),
                            );
                            pos = pos.wrapping_add(datalen as (u32));
                            j = j + 1;
                        }
                        if _currentBlock == 348 {
                        } else {
                            posauthority = pos;
                            j = 0i32;
                            'loop10: loop {
                                if !(j < numauthority as (i32)) {
                                    _currentBlock = 11;
                                    break;
                                }
                                pos = dns_packet_getname(
                                    buf as (*const u8),
                                    len,
                                    pos,
                                    &mut t1 as (*mut *mut u8),
                                );
                                if pos == 0 {
                                    _currentBlock = 348;
                                    break;
                                }
                                pos = dns_packet_copy(
                                    buf as (*const u8),
                                    len,
                                    pos,
                                    header.as_mut_ptr(),
                                    10u32,
                                );
                                if pos == 0 {
                                    _currentBlock = 348;
                                    break;
                                }
                                if typematch(
                                    header.as_mut_ptr() as (*const u8),
                                    (*b"\0\x06\0").as_ptr(),
                                ) != 0
                                {
                                    flagsoa = 1i32;
                                    soattl = ttlget(header.as_mut_ptr().offset(4isize));
                                    if soattl > 3600u32 {
                                        soattl = 3600u32;
                                    }
                                } else if typematch(
                                    header.as_mut_ptr() as (*const u8),
                                    (*b"\0\x02\0").as_ptr(),
                                ) != 0
                                {
                                    flagreferral = 1i32;
                                    if dns_domain_copy(
                                        &mut referral as (*mut *mut u8),
                                        t1 as (*const u8),
                                    ) == 0
                                    {
                                        _currentBlock = 348;
                                        break;
                                    }
                                }
                                uint16_unpack_big(
                                    header.as_mut_ptr().offset(8isize) as (*const u8),
                                    &mut datalen as (*mut u16),
                                );
                                pos = pos.wrapping_add(datalen as (u32));
                                j = j + 1;
                            }
                            if _currentBlock == 348 {
                            } else {
                                posglue = pos;
                                if flagcname == 0 && (rcode == 0) && (flagout == 0) &&
                                    (flagreferral != 0) &&
                                    (flagsoa == 0)
                                {
                                    if dns_domain_equal(
                                        referral as (*const u8),
                                        control as (*const u8),
                                    ) != 0 ||
                                        dns_domain_suffix(
                                            referral as (*const u8),
                                            control as (*const u8),
                                        ) == 0
                                    {
                                        log_lame(
                                            whichserver,
                                            control as (*const u8),
                                            referral as (*const u8),
                                        );
                                        byte::zero(whichserver as (*mut u8), 4u32);
                                        _currentBlock = 183;
                                    } else {
                                        _currentBlock = 13;
                                    }
                                } else {
                                    _currentBlock = 13;
                                }
                                if _currentBlock == 183 {
                                } else {
                                    if !records.is_null() {
                                        alloc::free(records as (*mut u8));
                                        records = 0i32 as (*mut u32);
                                    }
                                    k = numanswers as (i32) + numauthority as (i32) +
                                        numglue as (i32);
                                    records = alloc::alloc((k as (usize)).wrapping_mul(
                                        ::std::mem::size_of::<u32>(),
                                    ) as (u32)) as
                                        (*mut u32);
                                    if records.is_null() {
                                        _currentBlock = 348;
                                    } else {
                                        pos = posanswers;
                                        j = 0i32;
                                        'loop17: loop {
                                            if !(j < k) {
                                                _currentBlock = 18;
                                                break;
                                            }
                                            *records.offset(j as (isize)) = pos;
                                            pos = dns_packet_getname(
                                                buf as (*const u8),
                                                len,
                                                pos,
                                                &mut t1 as (*mut *mut u8),
                                            );
                                            if pos == 0 {
                                                _currentBlock = 348;
                                                break;
                                            }
                                            pos = dns_packet_copy(
                                                buf as (*const u8),
                                                len,
                                                pos,
                                                header.as_mut_ptr(),
                                                10u32,
                                            );
                                            if pos == 0 {
                                                _currentBlock = 348;
                                                break;
                                            }
                                            uint16_unpack_big(
                                                header.as_mut_ptr().offset(8isize) as (*const u8),
                                                &mut datalen as (*mut u16),
                                            );
                                            pos = pos.wrapping_add(datalen as (u32));
                                            j = j + 1;
                                        }
                                        if _currentBlock == 348 {
                                        } else {
                                            i = {
                                                j = k;
                                                j
                                            };
                                            'loop19: loop {
                                                if !(j > 1i32) {
                                                    break;
                                                }
                                                if i > 1i32 {
                                                    i = i - 1;
                                                    pos = *records.offset((i - 1i32) as (isize));
                                                } else {
                                                    pos = *records.offset((j - 1i32) as (isize));
                                                    *records.offset((j - 1i32) as (isize)) =
                                                        *records.offset((i - 1i32) as (isize));
                                                    j = j - 1;
                                                }
                                                q = i;
                                                'loop169: loop {
                                                    if !({
                                                             p = q * 2i32;
                                                             p
                                                         } <
                                                             j)
                                                    {
                                                        break;
                                                    }
                                                    if smaller(
                                                        buf,
                                                        len,
                                                        *records.offset(p as (isize)),
                                                        *records.offset((p - 1i32) as (isize)),
                                                    ) ==
                                                        0
                                                    {
                                                        p = p + 1;
                                                    }
                                                    *records.offset((q - 1i32) as (isize)) =
                                                        *records.offset((p - 1i32) as (isize));
                                                    q = p;
                                                }
                                                if p == j {
                                                    *records.offset((q - 1i32) as (isize)) =
                                                        *records.offset((p - 1i32) as (isize));
                                                    q = p;
                                                }
                                                'loop172: loop {
                                                    if !(q > i &&
                                                             (smaller(
                                                            buf,
                                                            len,
                                                            *records.offset(
                                                                ({
                                                                     p = q / 2i32;
                                                                     p
                                                                 } -
                                                                     1i32) as
                                                                    (isize),
                                                            ),
                                                            pos,
                                                        ) !=
                                                                  0))
                                                    {
                                                        break;
                                                    }
                                                    *records.offset((q - 1i32) as (isize)) =
                                                        *records.offset((p - 1i32) as (isize));
                                                    q = p;
                                                }
                                                *records.offset((q - 1i32) as (isize)) = pos;
                                            }
                                            i = 0i32;
                                            'loop21: loop {
                                                if !(i < k) {
                                                    _currentBlock = 22;
                                                    break;
                                                }
                                                let mut type_ : [u8; 2];
                                                pos = dns_packet_getname(
                                                    buf as (*const u8),
                                                    len,
                                                    *records.offset(i as (isize)),
                                                    &mut t1 as (*mut *mut u8),
                                                );
                                                if pos == 0 {
                                                    _currentBlock = 348;
                                                    break;
                                                }
                                                pos = dns_packet_copy(
                                                    buf as (*const u8),
                                                    len,
                                                    pos,
                                                    header.as_mut_ptr(),
                                                    10u32,
                                                );
                                                if pos == 0 {
                                                    _currentBlock = 348;
                                                    break;
                                                }
                                                ttl = ttlget(header.as_mut_ptr().offset(4isize));
                                                byte::copy(
                                                    type_.as_mut_ptr(),
                                                    2u32,
                                                    header.as_mut_ptr(),
                                                );
                                                if byte::diff(
                                                    header.as_mut_ptr().offset(2isize),
                                                    2u32,
                                                    (*b"\0\x01\0").as_ptr() as (*mut u8),
                                                ) !=
                                                    0
                                                {
                                                    i = i + 1;
                                                } else {
                                                    j = i + 1i32;
                                                    'loop98: loop {
                                                        if !(j < k) {
                                                            break;
                                                        }
                                                        pos = dns_packet_getname(
                                                            buf as (*const u8),
                                                            len,
                                                            *records.offset(j as (isize)),
                                                            &mut t2 as (*mut *mut u8),
                                                        );
                                                        if pos == 0 {
                                                            _currentBlock = 348;
                                                            break 'loop21;
                                                        }
                                                        pos = dns_packet_copy(
                                                            buf as (*const u8),
                                                            len,
                                                            pos,
                                                            header.as_mut_ptr(),
                                                            10u32,
                                                        );
                                                        if pos == 0 {
                                                            _currentBlock = 348;
                                                            break 'loop21;
                                                        }
                                                        if dns_domain_equal(
                                                            t1 as (*const u8),
                                                            t2 as (*const u8),
                                                        ) ==
                                                            0
                                                        {
                                                            break;
                                                        }
                                                        if byte::diff(
                                                            header.as_mut_ptr(),
                                                            2u32,
                                                            type_.as_mut_ptr(),
                                                        ) !=
                                                            0
                                                        {
                                                            break;
                                                        }
                                                        if byte::diff(
                                                            header.as_mut_ptr().offset(2isize),
                                                            2u32,
                                                            (*b"\0\x01\0").as_ptr() as (*mut u8),
                                                        ) !=
                                                            0
                                                        {
                                                            break;
                                                        }
                                                        j = j + 1;
                                                    }
                                                    if dns_domain_suffix(
                                                        t1 as (*const u8),
                                                        control as (*const u8),
                                                    ) ==
                                                        0
                                                    {
                                                        i = j;
                                                    } else if roots_same(t1, control) == 0 {
                                                        i = j;
                                                    } else {
                                                        if !(byte::diff(
                                                            type_.as_mut_ptr(),
                                                            2u32,
                                                            (*b"\0\xFF\0").as_ptr() as (*mut u8),
                                                        ) ==
                                                                 0)
                                                        {
                                                            if !(byte::diff(
                                                                type_.as_mut_ptr(),
                                                                2u32,
                                                                (*b"\0\xFC\0").as_ptr() as
                                                                    (*mut u8),
                                                            ) ==
                                                                     0)
                                                            {
                                                                if byte::diff(
                                                                    type_.as_mut_ptr(),
                                                                    2u32,
                                                                    (*b"\0\x06\0").as_ptr() as
                                                                        (*mut u8),
                                                                ) ==
                                                                    0
                                                                {
                                                                    'loop153: loop {
                                                                        if !(i < j) {
                                                                            break;
                                                                        }
                                                                        pos = dns_packet_skipname(
                                                                            buf as (*const u8),
                                                                            len,
                                                                            *records.offset(
                                                                                i as (isize),
                                                                            ),
                                                                        );
                                                                        if pos == 0 {
                                                                            _currentBlock = 348;
                                                                            break 'loop21;
                                                                        }
                                                                        pos = dns_packet_getname(
                                                                            buf as (*const u8),
                                                                            len,
                                                                            pos.wrapping_add(
                                                                                10u32,
                                                                            ),
                                                                            &mut t2 as
                                                                                (*mut *mut u8),
                                                                        );
                                                                        if pos == 0 {
                                                                            _currentBlock = 348;
                                                                            break 'loop21;
                                                                        }
                                                                        pos = dns_packet_getname(
                                                                            buf as (*const u8),
                                                                            len,
                                                                            pos,
                                                                            &mut t3 as
                                                                                (*mut *mut u8),
                                                                        );
                                                                        if pos == 0 {
                                                                            _currentBlock = 348;
                                                                            break 'loop21;
                                                                        }
                                                                        pos = dns_packet_copy(
                                                                            buf as (*const u8),
                                                                            len,
                                                                            pos,
                                                                            misc.as_mut_ptr(),
                                                                            20u32,
                                                                        );
                                                                        if pos == 0 {
                                                                            _currentBlock = 348;
                                                                            break 'loop21;
                                                                        }
                                                                        if *records.offset(
                                                                            i as (isize),
                                                                        ) <
                                                                            posauthority
                                                                        {
                                                                            log_rrsoa(
                                                                                whichserver,
                                                                                t1 as (*const u8),
                                                                                t2 as (*const u8),
                                                                                t3 as (*const u8),
                                                                                misc.as_mut_ptr() as
                                                                                    (*const u8),
                                                                                ttl,
                                                                            );
                                                                        }
                                                                        i = i + 1;
                                                                    }
                                                                } else if byte::diff(
                                                                    type_.as_mut_ptr(),
                                                                    2u32,
                                                                    (*b"\0\x05\0").as_ptr() as
                                                                        (*mut u8),
                                                                ) ==
                                                                           0
                                                                {
                                                                    pos = dns_packet_skipname(
                                                                        buf as (*const u8),
                                                                        len,
                                                                        *records.offset(
                                                                            (j - 1i32) as
                                                                                (isize),
                                                                        ),
                                                                    );
                                                                    if pos == 0 {
                                                                        _currentBlock = 348;
                                                                        break;
                                                                    }
                                                                    pos = dns_packet_getname(
                                                                        buf as (*const u8),
                                                                        len,
                                                                        pos.wrapping_add(10u32),
                                                                        &mut t2 as (*mut *mut u8),
                                                                    );
                                                                    if pos == 0 {
                                                                        _currentBlock = 348;
                                                                        break;
                                                                    }
                                                                    log_rrcname(
                                                                        whichserver,
                                                                        t1 as (*const u8),
                                                                        t2 as (*const u8),
                                                                        ttl,
                                                                    );
                                                                    cachegeneric(
                                                                        (*b"\0\x05\0").as_ptr(),
                                                                        t1 as (*const u8),
                                                                        t2 as (*const u8),
                                                                        dns_domain_length(
                                                                            t2 as (*const u8),
                                                                        ),
                                                                        ttl,
                                                                    );
                                                                } else if byte::diff(
                                                                    type_.as_mut_ptr(),
                                                                    2u32,
                                                                    (*b"\0\x0C\0").as_ptr() as
                                                                        (*mut u8),
                                                                ) ==
                                                                           0
                                                                {
                                                                    save_start();
                                                                    'loop145: loop {
                                                                        if !(i < j) {
                                                                            break;
                                                                        }
                                                                        pos = dns_packet_skipname(
                                                                            buf as (*const u8),
                                                                            len,
                                                                            *records.offset(
                                                                                i as (isize),
                                                                            ),
                                                                        );
                                                                        if pos == 0 {
                                                                            _currentBlock = 348;
                                                                            break 'loop21;
                                                                        }
                                                                        pos = dns_packet_getname(
                                                                            buf as (*const u8),
                                                                            len,
                                                                            pos.wrapping_add(
                                                                                10u32,
                                                                            ),
                                                                            &mut t2 as
                                                                                (*mut *mut u8),
                                                                        );
                                                                        if pos == 0 {
                                                                            _currentBlock = 348;
                                                                            break 'loop21;
                                                                        }
                                                                        log_rrptr(
                                                                            whichserver,
                                                                            t1 as (*const u8),
                                                                            t2 as (*const u8),
                                                                            ttl,
                                                                        );
                                                                        save_data(
                                                                            t2 as (*const u8),
                                                                            dns_domain_length(
                                                                                t2 as (*const u8),
                                                                            ),
                                                                        );
                                                                        i = i + 1;
                                                                    }
                                                                    save_finish(
                                                                        (*b"\0\x0C\0").as_ptr(),
                                                                        t1 as (*const u8),
                                                                        ttl,
                                                                    );
                                                                } else if byte::diff(
                                                                    type_.as_mut_ptr(),
                                                                    2u32,
                                                                    (*b"\0\x02\0").as_ptr() as
                                                                        (*mut u8),
                                                                ) ==
                                                                           0
                                                                {
                                                                    save_start();
                                                                    'loop139: loop {
                                                                        if !(i < j) {
                                                                            break;
                                                                        }
                                                                        pos = dns_packet_skipname(
                                                                            buf as (*const u8),
                                                                            len,
                                                                            *records.offset(
                                                                                i as (isize),
                                                                            ),
                                                                        );
                                                                        if pos == 0 {
                                                                            _currentBlock = 348;
                                                                            break 'loop21;
                                                                        }
                                                                        pos = dns_packet_getname(
                                                                            buf as (*const u8),
                                                                            len,
                                                                            pos.wrapping_add(
                                                                                10u32,
                                                                            ),
                                                                            &mut t2 as
                                                                                (*mut *mut u8),
                                                                        );
                                                                        if pos == 0 {
                                                                            _currentBlock = 348;
                                                                            break 'loop21;
                                                                        }
                                                                        log_rrns(
                                                                            whichserver,
                                                                            t1 as (*const u8),
                                                                            t2 as (*const u8),
                                                                            ttl,
                                                                        );
                                                                        save_data(
                                                                            t2 as (*const u8),
                                                                            dns_domain_length(
                                                                                t2 as (*const u8),
                                                                            ),
                                                                        );
                                                                        i = i + 1;
                                                                    }
                                                                    save_finish(
                                                                        (*b"\0\x02\0").as_ptr(),
                                                                        t1 as (*const u8),
                                                                        ttl,
                                                                    );
                                                                } else if byte::diff(
                                                                    type_.as_mut_ptr(),
                                                                    2u32,
                                                                    (*b"\0\x0F\0").as_ptr() as
                                                                        (*mut u8),
                                                                ) ==
                                                                           0
                                                                {
                                                                    save_start();
                                                                    'loop132: loop {
                                                                        if !(i < j) {
                                                                            break;
                                                                        }
                                                                        pos = dns_packet_skipname(
                                                                            buf as (*const u8),
                                                                            len,
                                                                            *records.offset(
                                                                                i as (isize),
                                                                            ),
                                                                        );
                                                                        if pos == 0 {
                                                                            _currentBlock = 348;
                                                                            break 'loop21;
                                                                        }
                                                                        pos = dns_packet_copy(
                                                                            buf as (*const u8),
                                                                            len,
                                                                            pos.wrapping_add(
                                                                                10u32,
                                                                            ),
                                                                            misc.as_mut_ptr(),
                                                                            2u32,
                                                                        );
                                                                        if pos == 0 {
                                                                            _currentBlock = 348;
                                                                            break 'loop21;
                                                                        }
                                                                        pos = dns_packet_getname(
                                                                            buf as (*const u8),
                                                                            len,
                                                                            pos,
                                                                            &mut t2 as
                                                                                (*mut *mut u8),
                                                                        );
                                                                        if pos == 0 {
                                                                            _currentBlock = 348;
                                                                            break 'loop21;
                                                                        }
                                                                        log_rrmx(
                                                                            whichserver,
                                                                            t1 as (*const u8),
                                                                            t2 as (*const u8),
                                                                            misc.as_mut_ptr() as
                                                                                (*const u8),
                                                                            ttl,
                                                                        );
                                                                        save_data(
                                                                            misc.as_mut_ptr() as
                                                                                (*const u8),
                                                                            2u32,
                                                                        );
                                                                        save_data(
                                                                            t2 as (*const u8),
                                                                            dns_domain_length(
                                                                                t2 as (*const u8),
                                                                            ),
                                                                        );
                                                                        i = i + 1;
                                                                    }
                                                                    save_finish(
                                                                        (*b"\0\x0F\0").as_ptr(),
                                                                        t1 as (*const u8),
                                                                        ttl,
                                                                    );
                                                                } else if byte::diff(
                                                                    type_.as_mut_ptr(),
                                                                    2u32,
                                                                    (*b"\0\x01\0").as_ptr() as
                                                                        (*mut u8),
                                                                ) ==
                                                                           0
                                                                {
                                                                    save_start();
                                                                    'loop123: loop {
                                                                        if !(i < j) {
                                                                            break;
                                                                        }
                                                                        pos = dns_packet_skipname(
                                                                            buf as (*const u8),
                                                                            len,
                                                                            *records.offset(
                                                                                i as (isize),
                                                                            ),
                                                                        );
                                                                        if pos == 0 {
                                                                            _currentBlock = 348;
                                                                            break 'loop21;
                                                                        }
                                                                        pos = dns_packet_copy(
                                                                            buf as (*const u8),
                                                                            len,
                                                                            pos,
                                                                            header.as_mut_ptr(),
                                                                            10u32,
                                                                        );
                                                                        if pos == 0 {
                                                                            _currentBlock = 348;
                                                                            break 'loop21;
                                                                        }
                                                                        if byte::diff(
                                                                            header
                                                                                .as_mut_ptr()
                                                                                .offset(8isize),
                                                                            2u32,
                                                                            (*b"\0\x04\0")
                                                                                .as_ptr() as
                                                                                (*mut u8),
                                                                        ) ==
                                                                            0
                                                                        {
                                                                            pos = dns_packet_copy(
                                                                                buf as
                                                                                    (*const u8),
                                                                                len,
                                                                                pos,
                                                                                header
                                                                                    .as_mut_ptr(),
                                                                                4u32,
                                                                            );
                                                                            if pos == 0 {
                                                                                _currentBlock = 348;
                                                                                break 'loop21;
                                                                            }
                                                                            save_data(
                                                                                header
                                                                                    .as_mut_ptr() as
                                                                                    (*const u8),
                                                                                4u32,
                                                                            );
                                                                            log_rr(
                                                                                whichserver,
                                                                                t1 as (*const u8),
                                                                                (*b"\0\x01\0")
                                                                                    .as_ptr(),
                                                                                header
                                                                                    .as_mut_ptr() as
                                                                                    (*const u8),
                                                                                4u32,
                                                                                ttl,
                                                                            );
                                                                        }
                                                                        i = i + 1;
                                                                    }
                                                                    save_finish(
                                                                        (*b"\0\x01\0").as_ptr(),
                                                                        t1 as (*const u8),
                                                                        ttl,
                                                                    );
                                                                } else {
                                                                    save_start();
                                                                    'loop116: loop {
                                                                        if !(i < j) {
                                                                            break;
                                                                        }
                                                                        pos = dns_packet_skipname(
                                                                            buf as (*const u8),
                                                                            len,
                                                                            *records.offset(
                                                                                i as (isize),
                                                                            ),
                                                                        );
                                                                        if pos == 0 {
                                                                            _currentBlock = 348;
                                                                            break 'loop21;
                                                                        }
                                                                        pos = dns_packet_copy(
                                                                            buf as (*const u8),
                                                                            len,
                                                                            pos,
                                                                            header.as_mut_ptr(),
                                                                            10u32,
                                                                        );
                                                                        if pos == 0 {
                                                                            _currentBlock = 348;
                                                                            break 'loop21;
                                                                        }
                                                                        uint16_unpack_big(
                                                                            header
                                                                                .as_mut_ptr()
                                                                                .offset(8isize) as
                                                                                (*const u8),
                                                                            &mut datalen as (*mut u16),
                                                                        );
                                                                        if datalen as (u32) >
                                                                            len.wrapping_sub(pos)
                                                                        {
                                                                            _currentBlock = 348;
                                                                            break 'loop21;
                                                                        }
                                                                        save_data(
                                                                            header
                                                                                .as_mut_ptr()
                                                                                .offset(8isize) as
                                                                                (*const u8),
                                                                            2u32,
                                                                        );
                                                                        save_data(
                                                                            buf.offset(
                                                                                pos as (isize),
                                                                            ) as
                                                                                (*const u8),
                                                                            datalen as (u32),
                                                                        );
                                                                        log_rr(
                                                                            whichserver,
                                                                            t1 as (*const u8),
                                                                            type_.as_mut_ptr() as
                                                                                (*const u8),
                                                                            buf.offset(
                                                                                pos as (isize),
                                                                            ) as
                                                                                (*const u8),
                                                                            datalen as (u32),
                                                                            ttl,
                                                                        );
                                                                        i = i + 1;
                                                                    }
                                                                    save_finish(
                                                                        type_.as_mut_ptr() as
                                                                            (*const u8),
                                                                        t1 as (*const u8),
                                                                        ttl,
                                                                    );
                                                                }
                                                            }
                                                        }
                                                        i = j;
                                                    }
                                                }
                                            }
                                            if _currentBlock == 348 {
                                            } else {
                                                alloc::free(records as (*mut u8));
                                                records = 0i32 as (*mut u32);
                                                if flagcname != 0 {
                                                    ttl = cnamettl;
                                                    _currentBlock = 285;
                                                } else if rcode == 3u32 {
                                                    log_nxdomain(
                                                        whichserver,
                                                        d as (*const u8),
                                                        soattl,
                                                    );
                                                    cachegeneric(
                                                        (*b"\0\xFF\0").as_ptr(),
                                                        d as (*const u8),
                                                        (*b"\0").as_ptr(),
                                                        0u32,
                                                        soattl,
                                                    );
                                                    _currentBlock = 301;
                                                } else {
                                                    if flagout == 0 && (flagsoa != 0) {
                                                        if byte::diff(
                                                            (*b"\0\xFF\0").as_ptr() as (*mut u8),
                                                            2u32,
                                                            dtype as (*mut u8),
                                                        ) !=
                                                            0
                                                        {
                                                            if byte::diff(
                                                                (*b"\0\xFC\0").as_ptr() as
                                                                    (*mut u8),
                                                                2u32,
                                                                dtype as (*mut u8),
                                                            ) !=
                                                                0
                                                            {
                                                                if byte::diff(
                                                                    (*b"\0\x05\0").as_ptr() as
                                                                        (*mut u8),
                                                                    2u32,
                                                                    dtype as (*mut u8),
                                                                ) !=
                                                                    0
                                                                {
                                                                    save_start();
                                                                    save_finish(
                                                                        dtype,
                                                                        d as (*const u8),
                                                                        soattl,
                                                                    );
                                                                    log_nodata(
                                                                        whichserver,
                                                                        d as (*const u8),
                                                                        dtype,
                                                                        soattl,
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    }
                                                    log_stats();
                                                    if flagout != 0 || flagsoa != 0 ||
                                                        flagreferral == 0
                                                    {
                                                        if (*z).level != 0 {
                                                            pos = posanswers;
                                                            j = 0i32;
                                                            'loop78: loop {
                                                                if !(j < numanswers as (i32)) {
                                                                    _currentBlock = 323;
                                                                    break;
                                                                }
                                                                pos = dns_packet_getname(
                                                                    buf as (*const u8),
                                                                    len,
                                                                    pos,
                                                                    &mut t1 as (*mut *mut u8),
                                                                );
                                                                if pos == 0 {
                                                                    _currentBlock = 348;
                                                                    break;
                                                                }
                                                                pos = dns_packet_copy(
                                                                    buf as (*const u8),
                                                                    len,
                                                                    pos,
                                                                    header.as_mut_ptr(),
                                                                    10u32,
                                                                );
                                                                if pos == 0 {
                                                                    _currentBlock = 348;
                                                                    break;
                                                                }
                                                                uint16_unpack_big(
                                                                    header.as_mut_ptr().offset(
                                                                        8isize,
                                                                    ) as
                                                                        (*const u8),
                                                                    &mut datalen as (*mut u16),
                                                                );
                                                                if dns_domain_equal(
                                                                    t1 as (*const u8),
                                                                    d as (*const u8),
                                                                ) !=
                                                                    0
                                                                {
                                                                    if typematch(
                                                                        header.as_mut_ptr() as
                                                                            (*const u8),
                                                                        (*b"\0\x01\0").as_ptr(),
                                                                    ) !=
                                                                        0
                                                                    {
                                                                        if byte::diff(
                                                                            header
                                                                                .as_mut_ptr()
                                                                                .offset(2isize),
                                                                            2u32,
                                                                            (*b"\0\x01\0")
                                                                                .as_ptr() as
                                                                                (*mut u8),
                                                                        ) ==
                                                                            0
                                                                        {
                                                                            if datalen as (i32) ==
                                                                                4i32
                                                                            {
                                                                                k = 0i32;
                                                                                'loop86: loop {
                                                                                    if !(k <
                                                                                        64i32)
                                                                                    {
                                                                                        _currentBlock = 90;
                                                                                        break;
                                                                                    }
                                                                                    if byte::diff(
                                                                                           (*z).servers[
                                                                                               (*z).level.wrapping_sub(
                                                                                                   1u32
                                                                                               ) as (usize)
                                                                                           ].as_mut_ptr(
                                                                                           ).offset(
                                                                                               k as (isize)
                                                                                           ),
                                                                                           4u32,
                                                                                           (*b"\0\0\0\0\0").as_ptr(
                                                                                           ) as (*mut u8)
                                                                                       ) == 0 {
                                                                                        _currentBlock = 89;
                                                                                        break;
                                                                                    }
                                                                                    k = k + 4i32;
                                                                                }
                                                                                if _currentBlock == 90 {
                                                                                } else if dns_packet_copy(
                                                                                              buf as (*const u8),
                                                                                              len,
                                                                                              pos,
                                                                                              (*z).servers[
                                                                                                  (*z).level.wrapping_sub(
                                                                                                      1u32
                                                                                                  ) as (usize)
                                                                                              ].as_mut_ptr(
                                                                                              ).offset(
                                                                                                  k as (isize)
                                                                                              ),
                                                                                              4u32
                                                                                          ) == 0 {
                                                                                    _currentBlock = 348;
                                                                                    break;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                pos = pos.wrapping_add(
                                                                    datalen as (u32),
                                                                );
                                                                j = j + 1;
                                                            }
                                                        } else if rqa(z) == 0 {
                                                            _currentBlock = 348;
                                                        } else {
                                                            pos = posanswers;
                                                            j = 0i32;
                                                            'loop49: loop {
                                                                if !(j < numanswers as (i32)) {
                                                                    _currentBlock = 50;
                                                                    break;
                                                                }
                                                                pos = dns_packet_getname(
                                                                    buf as (*const u8),
                                                                    len,
                                                                    pos,
                                                                    &mut t1 as (*mut *mut u8),
                                                                );
                                                                if pos == 0 {
                                                                    _currentBlock = 348;
                                                                    break;
                                                                }
                                                                pos = dns_packet_copy(
                                                                    buf as (*const u8),
                                                                    len,
                                                                    pos,
                                                                    header.as_mut_ptr(),
                                                                    10u32,
                                                                );
                                                                if pos == 0 {
                                                                    _currentBlock = 348;
                                                                    break;
                                                                }
                                                                ttl = ttlget(
                                                                    header.as_mut_ptr().offset(
                                                                        4isize,
                                                                    ),
                                                                );
                                                                uint16_unpack_big(
                                                                    header.as_mut_ptr().offset(
                                                                        8isize,
                                                                    ) as
                                                                        (*const u8),
                                                                    &mut datalen as (*mut u16),
                                                                );
                                                                if dns_domain_equal(
                                                                    t1 as (*const u8),
                                                                    d as (*const u8),
                                                                ) !=
                                                                    0
                                                                {
                                                                    if byte::diff(
                                                                        header
                                                                            .as_mut_ptr()
                                                                            .offset(2isize),
                                                                        2u32,
                                                                        (*b"\0\x01\0").as_ptr() as
                                                                            (*mut u8),
                                                                    ) ==
                                                                        0
                                                                    {
                                                                        if typematch(
                                                                            header.as_mut_ptr() as
                                                                                (*const u8),
                                                                            dtype,
                                                                        ) !=
                                                                            0
                                                                        {
                                                                            if response_rstart(
                                                                                t1 as (*const u8),
                                                                                header
                                                                                    .as_mut_ptr() as
                                                                                    (*const u8),
                                                                                ttl,
                                                                            ) ==
                                                                                0
                                                                            {
                                                                                _currentBlock = 348;
                                                                                break;
                                                                            }
                                                                            if typematch(
                                                                                   header.as_mut_ptr(
                                                                                   ) as (*const u8),
                                                                                   (*b"\0\x02\0").as_ptr(
                                                                                   )
                                                                               ) != 0 || typematch(
                                                                                             header.as_mut_ptr(
                                                                                             ) as (*const u8),
                                                                                             (*b"\0\x05\0").as_ptr(
                                                                                             )
                                                                                         ) != 0 || typematch(
                                                                                                       header.as_mut_ptr(
                                                                                                       ) as (*const u8),
                                                                                                       (*b"\0\x0C\0").as_ptr(
                                                                                                       )
                                                                                                   ) != 0 {
                                                                                if dns_packet_getname(
                                                                                       buf as (*const u8),
                                                                                       len,
                                                                                       pos,
                                                                                       &mut t2 as (*mut *mut u8)
                                                                                   ) == 0 {
                                                                                    _currentBlock = 348;
                                                                                    break;
                                                                                }
                                                                                if response_addname(
                                                                                       t2 as (*const u8)
                                                                                   ) == 0 {
                                                                                    _currentBlock = 348;
                                                                                    break;
                                                                                }
                                                                            } else if typematch(
                                                                                          header.as_mut_ptr(
                                                                                          ) as (*const u8),
                                                                                          (*b"\0\x0F\0").as_ptr(
                                                                                          )
                                                                                      ) != 0 {
                                                                                pos2 = dns_packet_copy(
                                                                                           buf as (*const u8),
                                                                                           len,
                                                                                           pos,
                                                                                           misc.as_mut_ptr(
                                                                                           ),
                                                                                           2u32
                                                                                       );
                                                                                if pos2 == 0 {
                                                                                    _currentBlock = 348;
                                                                                    break;
                                                                                }
                                                                                if response_addbytes(
                                                                                       misc.as_mut_ptr(
                                                                                       ) as (*const u8),
                                                                                       2u32
                                                                                   ) == 0 {
                                                                                    _currentBlock = 348;
                                                                                    break;
                                                                                }
                                                                                if dns_packet_getname(
                                                                                       buf as (*const u8),
                                                                                       len,
                                                                                       pos2,
                                                                                       &mut t2 as (*mut *mut u8)
                                                                                   ) == 0 {
                                                                                    _currentBlock = 348;
                                                                                    break;
                                                                                }
                                                                                if response_addname(
                                                                                       t2 as (*const u8)
                                                                                   ) == 0 {
                                                                                    _currentBlock = 348;
                                                                                    break;
                                                                                }
                                                                            } else if typematch(
                                                                                          header.as_mut_ptr(
                                                                                          ) as (*const u8),
                                                                                          (*b"\0\x06\0").as_ptr(
                                                                                          )
                                                                                      ) != 0 {
                                                                                pos2 = dns_packet_getname(
                                                                                           buf as (*const u8),
                                                                                           len,
                                                                                           pos,
                                                                                           &mut t2 as (*mut *mut u8)
                                                                                       );
                                                                                if pos2 == 0 {
                                                                                    _currentBlock = 348;
                                                                                    break;
                                                                                }
                                                                                if response_addname(
                                                                                       t2 as (*const u8)
                                                                                   ) == 0 {
                                                                                    _currentBlock = 348;
                                                                                    break;
                                                                                }
                                                                                pos2 = dns_packet_getname(
                                                                                           buf as (*const u8),
                                                                                           len,
                                                                                           pos2,
                                                                                           &mut t3 as (*mut *mut u8)
                                                                                       );
                                                                                if pos2 == 0 {
                                                                                    _currentBlock = 348;
                                                                                    break;
                                                                                }
                                                                                if response_addname(
                                                                                       t3 as (*const u8)
                                                                                   ) == 0 {
                                                                                    _currentBlock = 348;
                                                                                    break;
                                                                                }
                                                                                pos2 = dns_packet_copy(
                                                                                           buf as (*const u8),
                                                                                           len,
                                                                                           pos2,
                                                                                           misc.as_mut_ptr(
                                                                                           ),
                                                                                           20u32
                                                                                       );
                                                                                if pos2 == 0 {
                                                                                    _currentBlock = 348;
                                                                                    break;
                                                                                }
                                                                                if response_addbytes(
                                                                                       misc.as_mut_ptr(
                                                                                       ) as (*const u8),
                                                                                       20u32
                                                                                   ) == 0 {
                                                                                    _currentBlock = 348;
                                                                                    break;
                                                                                }
                                                                            } else {
                                                                                if pos.wrapping_add(
                                                                                       datalen as (u32)
                                                                                   ) > len {
                                                                                    _currentBlock = 348;
                                                                                    break;
                                                                                }
                                                                                if response_addbytes(
                                                                                       buf.offset(
                                                                                           pos as (isize)
                                                                                       ) as (*const u8),
                                                                                       datalen as (u32)
                                                                                   ) == 0 {
                                                                                    _currentBlock = 348;
                                                                                    break;
                                                                                }
                                                                            }
                                                                            response_rfinish(6i32);
                                                                        }
                                                                    }
                                                                }
                                                                pos = pos.wrapping_add(
                                                                    datalen as (u32),
                                                                );
                                                                j = j + 1;
                                                            }
                                                            if _currentBlock == 348 {
                                                            } else {
                                                                cleanup(z);
                                                                return 1i32;
                                                            }
                                                        }
                                                    } else if dns_domain_suffix(
                                                        d as (*const u8),
                                                        referral as (*const u8),
                                                    ) ==
                                                               0
                                                    {
                                                        _currentBlock = 348;
                                                    } else {
                                                        control = d.offset(dns_domain_suffixpos(
                                                            d as (*const u8),
                                                            referral as (*const u8),
                                                        ) as
                                                            (isize));
                                                        (*z).control[(*z).level as (usize)] =
                                                            control;
                                                        byte::zero(
                                                            (*z).servers[(*z).level as (usize)]
                                                                .as_mut_ptr(),
                                                            64u32,
                                                        );
                                                        j = 0i32;
                                                        'loop32: loop {
                                                            if !(j < 16i32) {
                                                                break;
                                                            }
                                                            dns_domain_free(
                                                                &mut (*z).ns[(*z).level as
                                                                                 (usize)]
                                                                    [j as (usize)] as
                                                                    (*mut *mut u8),
                                                            );
                                                            j = j + 1;
                                                        }
                                                        k = 0i32;
                                                        pos = posauthority;
                                                        j = 0i32;
                                                        'loop34: loop {
                                                            if !(j < numauthority as (i32)) {
                                                                _currentBlock = 183;
                                                                break;
                                                            }
                                                            pos = dns_packet_getname(
                                                                buf as (*const u8),
                                                                len,
                                                                pos,
                                                                &mut t1 as (*mut *mut u8),
                                                            );
                                                            if pos == 0 {
                                                                _currentBlock = 348;
                                                                break;
                                                            }
                                                            pos = dns_packet_copy(
                                                                buf as (*const u8),
                                                                len,
                                                                pos,
                                                                header.as_mut_ptr(),
                                                                10u32,
                                                            );
                                                            if pos == 0 {
                                                                _currentBlock = 348;
                                                                break;
                                                            }
                                                            uint16_unpack_big(
                                                                header.as_mut_ptr().offset(
                                                                    8isize,
                                                                ) as
                                                                    (*const u8),
                                                                &mut datalen as (*mut u16),
                                                            );
                                                            if dns_domain_equal(
                                                                referral as (*const u8),
                                                                t1 as (*const u8),
                                                            ) !=
                                                                0
                                                            {
                                                                if typematch(
                                                                    header.as_mut_ptr() as
                                                                        (*const u8),
                                                                    (*b"\0\x02\0").as_ptr(),
                                                                ) !=
                                                                    0
                                                                {
                                                                    if byte::diff(
                                                                        header
                                                                            .as_mut_ptr()
                                                                            .offset(2isize),
                                                                        2u32,
                                                                        (*b"\0\x01\0").as_ptr() as
                                                                            (*mut u8),
                                                                    ) ==
                                                                        0
                                                                    {
                                                                        if k < 16i32 {
                                                                            if dns_packet_getname(
                                                                                   buf as (*const u8),
                                                                                   len,
                                                                                   pos,
                                                                                   &mut (*z).ns[
                                                                                            (*z).level as (usize)
                                                                                        ][  {
                                                                                                let _old
                                                                                                    = k;
                                                                                                k = k + 1;
                                                                                                _old
                                                                                            } as (usize)
                                                                                        ] as (*mut *mut u8)
                                                                               ) == 0 {
                                                                                _currentBlock = 348;
                                                                                break;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            pos =
                                                                pos.wrapping_add(datalen as (u32));
                                                            j = j + 1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else if state == -1i32 {
        log_servfail((*z).name[(*z).level as (usize)] as (*const u8));
        _currentBlock = 194;
    } else {
        _currentBlock = 203;
    }
    'loop183: loop {
        if _currentBlock == 183 {
            j = 0i32;
            'loop184: loop {
                if !(j < 16i32) {
                    _currentBlock = 185;
                    break;
                }
                if !(*z).ns[(*z).level as (usize)][j as (usize)].is_null() {
                    if (*z).level.wrapping_add(1u32) < 5u32 {
                        _currentBlock = 201;
                        break;
                    }
                    dns_domain_free(
                        &mut (*z).ns[(*z).level as (usize)][j as (usize)] as (*mut *mut u8),
                    );
                }
                j = j + 1;
            }
            if _currentBlock == 185 {
                j = 0i32;
                'loop186: loop {
                    if !(j < 64i32) {
                        break;
                    }
                    if byte::diff(
                        (*z).servers[(*z).level as (usize)].as_mut_ptr().offset(
                            j as (isize),
                        ),
                        4u32,
                        (*b"\0\0\0\0\0").as_ptr() as (*mut u8),
                    ) != 0
                    {
                        break;
                    }
                    j = j + 4i32;
                }
                if j == 64i32 {
                    _currentBlock = 194;
                    continue;
                }
                dns_sortip((*z).servers[(*z).level as (usize)].as_mut_ptr(), 64u32);
                if (*z).level != 0 {
                    log_tx(
                        (*z).name[(*z).level as (usize)] as (*const u8),
                        (*b"\0\x01\0").as_ptr(),
                        (*z).control[(*z).level as (usize)] as (*const u8),
                        (*z).servers[(*z).level as (usize)].as_mut_ptr() as (*const u8),
                        (*z).level,
                    );
                    if dns_transmit_start(
                        &mut (*z).dt as (*mut dns_transmit),
                        (*z).servers[(*z).level as (usize)].as_mut_ptr() as (*const u8),
                        flagforwardonly,
                        (*z).name[(*z).level as (usize)] as (*const u8),
                        (*b"\0\x01\0").as_ptr(),
                        (*z).localip.as_mut_ptr() as (*const u8),
                    ) == -1i32
                    {
                        _currentBlock = 348;
                    } else {
                        _currentBlock = 193;
                        break;
                    }
                } else {
                    log_tx(
                        (*z).name[0usize] as (*const u8),
                        (*z).type_.as_mut_ptr() as (*const u8),
                        (*z).control[0usize] as (*const u8),
                        (*z).servers[0usize].as_mut_ptr() as (*const u8),
                        0u32,
                    );
                    if dns_transmit_start(
                        &mut (*z).dt as (*mut dns_transmit),
                        (*z).servers[0usize].as_mut_ptr() as (*const u8),
                        flagforwardonly,
                        (*z).name[0usize] as (*const u8),
                        (*z).type_.as_mut_ptr() as (*const u8),
                        (*z).localip.as_mut_ptr() as (*const u8),
                    ) == -1i32
                    {
                        _currentBlock = 348;
                    } else {
                        _currentBlock = 193;
                        break;
                    }
                }
            } else {
                if dns_domain_copy(
                    &mut (*z).name[(*z).level.wrapping_add(1u32) as (usize)] as (*mut *mut u8),
                    (*z).ns[(*z).level as (usize)][j as (usize)] as (*const u8),
                ) == 0
                {
                    _currentBlock = 348;
                    continue;
                }
                dns_domain_free(
                    &mut (*z).ns[(*z).level as (usize)][j as (usize)] as (*mut *mut u8),
                );
                (*z).level = (*z).level.wrapping_add(1u32);
                _currentBlock = 203;
            }
        } else if _currentBlock == 194 {
            if (*z).level != 0 {
                _currentBlock = 323;
                continue;
            }
            if rqa(z) == 0 {
                _currentBlock = 348;
            } else {
                _currentBlock = 196;
                break;
            }
        } else if _currentBlock == 203 {
            if {
                (*z).loopvar = (*z).loopvar.wrapping_add(1u32);
                (*z).loopvar
            } == 100u32
            {
                _currentBlock = 348;
                continue;
            }
            d = (*z).name[(*z).level as (usize)];
            dtype = if (*z).level != 0 {
                (*b"\0\x01\0").as_ptr()
            } else {
                (*z).type_.as_mut_ptr() as (*const u8)
            };
            dlen = dns_domain_length(d as (*const u8));
            if globalip(d, misc.as_mut_ptr()) != 0 {
                if (*z).level != 0 {
                    k = 0i32;
                    'loop319: loop {
                        if !(k < 64i32) {
                            _currentBlock = 323;
                            continue 'loop183;
                        }
                        if byte::diff(
                            (*z).servers[(*z).level.wrapping_sub(1u32) as (usize)]
                                .as_mut_ptr()
                                .offset(k as (isize)),
                            4u32,
                            (*b"\0\0\0\0\0").as_ptr() as (*mut u8),
                        ) == 0
                        {
                            break;
                        }
                        k = k + 4i32;
                    }
                    byte::copy(
                        (*z).servers[(*z).level.wrapping_sub(1u32) as (usize)]
                            .as_mut_ptr()
                            .offset(k as (isize)),
                        4u32,
                        misc.as_mut_ptr(),
                    );
                    _currentBlock = 323;
                } else {
                    if rqa(z) == 0 {
                        _currentBlock = 348;
                        continue;
                    }
                    if typematch((*b"\0\x01\0").as_ptr(), dtype) == 0 {
                        _currentBlock = 317;
                        break;
                    }
                    if response_rstart(d as (*const u8), (*b"\0\x01\0").as_ptr(), 655360u32) == 0 {
                        _currentBlock = 348;
                        continue;
                    }
                    if response_addbytes(misc.as_mut_ptr() as (*const u8), 4u32) == 0 {
                        _currentBlock = 348;
                    } else {
                        _currentBlock = 316;
                        break;
                    }
                }
            } else if dns_domain_equal(
                d as (*const u8),
                (*b"\x011\x010\x010\x03127\x07in-addr\x04arpa\0\0").as_ptr(),
            ) != 0
            {
                if (*z).level != 0 {
                    _currentBlock = 323;
                    continue;
                }
                if rqa(z) == 0 {
                    _currentBlock = 348;
                    continue;
                }
                if typematch((*b"\0\x0C\0").as_ptr(), dtype) == 0 {
                    _currentBlock = 310;
                    break;
                }
                if response_rstart(d as (*const u8), (*b"\0\x0C\0").as_ptr(), 655360u32) == 0 {
                    _currentBlock = 348;
                    continue;
                }
                if response_addname((*b"\tlocalhost\0\0").as_ptr()) == 0 {
                    _currentBlock = 348;
                } else {
                    _currentBlock = 309;
                    break;
                }
            } else {
                if dlen <= 255u32 {
                    byte::copy(key.as_mut_ptr(), 2u32, (*b"\0\xFF\0").as_ptr() as (*mut u8));
                    byte::copy(key.as_mut_ptr().offset(2isize), dlen, d);
                    case_lowerb(key.as_mut_ptr().offset(2isize), dlen);
                    cached = cache_get(
                        key.as_mut_ptr() as (*const u8),
                        dlen.wrapping_add(2u32),
                        &mut cachedlen as (*mut u32),
                        &mut ttl as (*mut u32),
                    );
                    if !cached.is_null() {
                        log_cachednxdomain(d as (*const u8));
                        _currentBlock = 301;
                        continue;
                    } else {
                        byte::copy(key.as_mut_ptr(), 2u32, (*b"\0\x05\0").as_ptr() as (*mut u8));
                        cached = cache_get(
                            key.as_mut_ptr() as (*const u8),
                            dlen.wrapping_add(2u32),
                            &mut cachedlen as (*mut u32),
                            &mut ttl as (*mut u32),
                        );
                        if !cached.is_null() {
                            if typematch((*b"\0\x05\0").as_ptr(), dtype) != 0 {
                                log_cachedanswer(d as (*const u8), (*b"\0\x05\0").as_ptr());
                                if rqa(z) == 0 {
                                    _currentBlock = 348;
                                    continue;
                                }
                                if response_cname(
                                    (*z).name[0usize] as (*const u8),
                                    cached as (*const u8),
                                    ttl,
                                ) == 0
                                {
                                    _currentBlock = 348;
                                    continue;
                                } else {
                                    _currentBlock = 299;
                                    break;
                                }
                            } else {
                                log_cachedcname(d as (*const u8), cached as (*const u8));
                                if dns_domain_copy(
                                    &mut cname as (*mut *mut u8),
                                    cached as (*const u8),
                                ) == 0
                                {
                                    _currentBlock = 348;
                                    continue;
                                } else {
                                    _currentBlock = 285;
                                    continue;
                                }
                            }
                        } else {
                            if typematch((*b"\0\x02\0").as_ptr(), dtype) != 0 {
                                byte::copy(
                                    key.as_mut_ptr(),
                                    2u32,
                                    (*b"\0\x02\0").as_ptr() as (*mut u8),
                                );
                                cached = cache_get(
                                    key.as_mut_ptr() as (*const u8),
                                    dlen.wrapping_add(2u32),
                                    &mut cachedlen as (*mut u32),
                                    &mut ttl as (*mut u32),
                                );
                                if !cached.is_null() &&
                                    (cachedlen != 0 ||
                                         byte::diff(
                                            dtype as (*mut u8),
                                            2u32,
                                            (*b"\0\xFF\0").as_ptr() as (*mut u8),
                                        ) != 0)
                                {
                                    log_cachedanswer(d as (*const u8), (*b"\0\x02\0").as_ptr());
                                    if rqa(z) == 0 {
                                        _currentBlock = 348;
                                        continue;
                                    }
                                    pos = 0u32;
                                    'loop278: loop {
                                        if {
                                            pos = dns_packet_getname(
                                                cached as (*const u8),
                                                cachedlen,
                                                pos,
                                                &mut t2 as (*mut *mut u8),
                                            );
                                            pos
                                        } == 0
                                        {
                                            _currentBlock = 279;
                                            break 'loop183;
                                        }
                                        if response_rstart(
                                            d as (*const u8),
                                            (*b"\0\x02\0").as_ptr(),
                                            ttl,
                                        ) == 0
                                        {
                                            _currentBlock = 348;
                                            continue 'loop183;
                                        }
                                        if response_addname(t2 as (*const u8)) == 0 {
                                            _currentBlock = 348;
                                            continue 'loop183;
                                        }
                                        response_rfinish(6i32);
                                    }
                                }
                            }
                            if typematch((*b"\0\x0C\0").as_ptr(), dtype) != 0 {
                                byte::copy(
                                    key.as_mut_ptr(),
                                    2u32,
                                    (*b"\0\x0C\0").as_ptr() as (*mut u8),
                                );
                                cached = cache_get(
                                    key.as_mut_ptr() as (*const u8),
                                    dlen.wrapping_add(2u32),
                                    &mut cachedlen as (*mut u32),
                                    &mut ttl as (*mut u32),
                                );
                                if !cached.is_null() &&
                                    (cachedlen != 0 ||
                                         byte::diff(
                                            dtype as (*mut u8),
                                            2u32,
                                            (*b"\0\xFF\0").as_ptr() as (*mut u8),
                                        ) != 0)
                                {
                                    log_cachedanswer(d as (*const u8), (*b"\0\x0C\0").as_ptr());
                                    if rqa(z) == 0 {
                                        _currentBlock = 348;
                                        continue;
                                    }
                                    pos = 0u32;
                                    'loop271: loop {
                                        if {
                                            pos = dns_packet_getname(
                                                cached as (*const u8),
                                                cachedlen,
                                                pos,
                                                &mut t2 as (*mut *mut u8),
                                            );
                                            pos
                                        } == 0
                                        {
                                            _currentBlock = 272;
                                            break 'loop183;
                                        }
                                        if response_rstart(
                                            d as (*const u8),
                                            (*b"\0\x0C\0").as_ptr(),
                                            ttl,
                                        ) == 0
                                        {
                                            _currentBlock = 348;
                                            continue 'loop183;
                                        }
                                        if response_addname(t2 as (*const u8)) == 0 {
                                            _currentBlock = 348;
                                            continue 'loop183;
                                        }
                                        response_rfinish(6i32);
                                    }
                                }
                            }
                            if typematch((*b"\0\x0F\0").as_ptr(), dtype) != 0 {
                                byte::copy(
                                    key.as_mut_ptr(),
                                    2u32,
                                    (*b"\0\x0F\0").as_ptr() as (*mut u8),
                                );
                                cached = cache_get(
                                    key.as_mut_ptr() as (*const u8),
                                    dlen.wrapping_add(2u32),
                                    &mut cachedlen as (*mut u32),
                                    &mut ttl as (*mut u32),
                                );
                                if !cached.is_null() &&
                                    (cachedlen != 0 ||
                                         byte::diff(
                                            dtype as (*mut u8),
                                            2u32,
                                            (*b"\0\xFF\0").as_ptr() as (*mut u8),
                                        ) != 0)
                                {
                                    log_cachedanswer(d as (*const u8), (*b"\0\x0F\0").as_ptr());
                                    if rqa(z) == 0 {
                                        _currentBlock = 348;
                                        continue;
                                    }
                                    pos = 0u32;
                                    'loop262: loop {
                                        if {
                                            pos = dns_packet_copy(
                                                cached as (*const u8),
                                                cachedlen,
                                                pos,
                                                misc.as_mut_ptr(),
                                                2u32,
                                            );
                                            pos
                                        } == 0
                                        {
                                            _currentBlock = 268;
                                            break 'loop183;
                                        }
                                        pos = dns_packet_getname(
                                            cached as (*const u8),
                                            cachedlen,
                                            pos,
                                            &mut t2 as (*mut *mut u8),
                                        );
                                        if pos == 0 {
                                            _currentBlock = 268;
                                            break 'loop183;
                                        }
                                        if response_rstart(
                                            d as (*const u8),
                                            (*b"\0\x0F\0").as_ptr(),
                                            ttl,
                                        ) == 0
                                        {
                                            _currentBlock = 348;
                                            continue 'loop183;
                                        }
                                        if response_addbytes(
                                            misc.as_mut_ptr() as (*const u8),
                                            2u32,
                                        ) == 0
                                        {
                                            _currentBlock = 348;
                                            continue 'loop183;
                                        }
                                        if response_addname(t2 as (*const u8)) == 0 {
                                            _currentBlock = 348;
                                            continue 'loop183;
                                        }
                                        response_rfinish(6i32);
                                    }
                                }
                            }
                            if typematch((*b"\0\x01\0").as_ptr(), dtype) != 0 {
                                byte::copy(
                                    key.as_mut_ptr(),
                                    2u32,
                                    (*b"\0\x01\0").as_ptr() as (*mut u8),
                                );
                                cached = cache_get(
                                    key.as_mut_ptr() as (*const u8),
                                    dlen.wrapping_add(2u32),
                                    &mut cachedlen as (*mut u32),
                                    &mut ttl as (*mut u32),
                                );
                                if !cached.is_null() &&
                                    (cachedlen != 0 ||
                                         byte::diff(
                                            dtype as (*mut u8),
                                            2u32,
                                            (*b"\0\xFF\0").as_ptr() as (*mut u8),
                                        ) != 0)
                                {
                                    if (*z).level != 0 {
                                        log_cachedanswer(d as (*const u8), (*b"\0\x01\0").as_ptr());
                                        'loop253: loop {
                                            if !(cachedlen >= 4u32) {
                                                _currentBlock = 323;
                                                continue 'loop183;
                                            }
                                            k = 0i32;
                                            'loop255: loop {
                                                if !(k < 64i32) {
                                                    _currentBlock = 259;
                                                    break;
                                                }
                                                if byte::diff(
                                                    (*z).servers[(*z).level.wrapping_sub(1u32) as
                                                                     (usize)]
                                                        .as_mut_ptr()
                                                        .offset(k as (isize)),
                                                    4u32,
                                                    (*b"\0\0\0\0\0").as_ptr() as (*mut u8),
                                                ) ==
                                                    0
                                                {
                                                    _currentBlock = 258;
                                                    break;
                                                }
                                                k = k + 4i32;
                                            }
                                            if _currentBlock == 258 {
                                                byte::copy(
                                                    (*z).servers[(*z).level.wrapping_sub(1u32) as
                                                                     (usize)]
                                                        .as_mut_ptr()
                                                        .offset(k as (isize)),
                                                    4u32,
                                                    cached,
                                                );
                                            }
                                            cached = cached.offset(4isize);
                                            cachedlen = cachedlen.wrapping_sub(4u32);
                                        }
                                    } else {
                                        log_cachedanswer(d as (*const u8), (*b"\0\x01\0").as_ptr());
                                        if rqa(z) == 0 {
                                            _currentBlock = 348;
                                            continue;
                                        }
                                        'loop247: loop {
                                            if !(cachedlen >= 4u32) {
                                                _currentBlock = 248;
                                                break 'loop183;
                                            }
                                            if response_rstart(
                                                d as (*const u8),
                                                (*b"\0\x01\0").as_ptr(),
                                                ttl,
                                            ) == 0
                                            {
                                                _currentBlock = 348;
                                                continue 'loop183;
                                            }
                                            if response_addbytes(cached as (*const u8), 4u32) == 0 {
                                                _currentBlock = 348;
                                                continue 'loop183;
                                            }
                                            response_rfinish(6i32);
                                            cached = cached.offset(4isize);
                                            cachedlen = cachedlen.wrapping_sub(4u32);
                                        }
                                    }
                                }
                            }
                            if typematch((*b"\0\xFF\0").as_ptr(), dtype) == 0 &&
                                (typematch((*b"\0\xFC\0").as_ptr(), dtype) == 0) &&
                                (typematch((*b"\0\x05\0").as_ptr(), dtype) == 0) &&
                                (typematch((*b"\0\x02\0").as_ptr(), dtype) == 0) &&
                                (typematch((*b"\0\x0C\0").as_ptr(), dtype) == 0) &&
                                (typematch((*b"\0\x01\0").as_ptr(), dtype) == 0) &&
                                (typematch((*b"\0\x0F\0").as_ptr(), dtype) == 0)
                            {
                                byte::copy(key.as_mut_ptr(), 2u32, dtype as (*mut u8));
                                cached = cache_get(
                                    key.as_mut_ptr() as (*const u8),
                                    dlen.wrapping_add(2u32),
                                    &mut cachedlen as (*mut u32),
                                    &mut ttl as (*mut u32),
                                );
                                if !cached.is_null() &&
                                    (cachedlen != 0 ||
                                         byte::diff(
                                            dtype as (*mut u8),
                                            2u32,
                                            (*b"\0\xFF\0").as_ptr() as (*mut u8),
                                        ) != 0)
                                {
                                    log_cachedanswer(d as (*const u8), dtype);
                                    if rqa(z) == 0 {
                                        _currentBlock = 348;
                                        continue;
                                    }
                                    'loop239: loop {
                                        if !(cachedlen >= 2u32) {
                                            _currentBlock = 240;
                                            break 'loop183;
                                        }
                                        uint16_unpack_big(
                                            cached as (*const u8),
                                            &mut datalen as (*mut u16),
                                        );
                                        cached = cached.offset(2isize);
                                        cachedlen = cachedlen.wrapping_sub(2u32);
                                        if datalen as (u32) > cachedlen {
                                            _currentBlock = 348;
                                            continue 'loop183;
                                        }
                                        if response_rstart(d as (*const u8), dtype, ttl) == 0 {
                                            _currentBlock = 348;
                                            continue 'loop183;
                                        }
                                        if response_addbytes(
                                            cached as (*const u8),
                                            datalen as (u32),
                                        ) == 0
                                        {
                                            _currentBlock = 348;
                                            continue 'loop183;
                                        }
                                        response_rfinish(6i32);
                                        cached = cached.offset(datalen as (isize));
                                        cachedlen = cachedlen.wrapping_sub(datalen as (u32));
                                    }
                                }
                            }
                        }
                    }
                }
                'loop219: loop {
                    if roots((*z).servers[(*z).level as (usize)].as_mut_ptr(), d) != 0 {
                        _currentBlock = 233;
                        break;
                    }
                    if flagforwardonly == 0 && ((*z).level < 2u32) {
                        if dlen < 255u32 {
                            byte::copy(key.as_mut_ptr(), 2u32, (*b"\0\x02\0").as_ptr() as (*mut u8));
                            byte::copy(key.as_mut_ptr().offset(2isize), dlen, d);
                            case_lowerb(key.as_mut_ptr().offset(2isize), dlen);
                            cached = cache_get(
                                key.as_mut_ptr() as (*const u8),
                                dlen.wrapping_add(2u32),
                                &mut cachedlen as (*mut u32),
                                &mut ttl as (*mut u32),
                            );
                            if !cached.is_null() && (cachedlen != 0) {
                                _currentBlock = 225;
                                break;
                            }
                        }
                    }
                    if *d == 0 {
                        _currentBlock = 348;
                        continue 'loop183;
                    }
                    j = 1u32.wrapping_add(*d as (u32)) as (i32);
                    dlen = dlen.wrapping_sub(j as (u32));
                    d = d.offset(j as (isize));
                }
                if _currentBlock == 225 {
                    (*z).control[(*z).level as (usize)] = d;
                    byte::zero((*z).servers[(*z).level as (usize)].as_mut_ptr(), 64u32);
                    j = 0i32;
                    'loop226: loop {
                        if !(j < 16i32) {
                            break;
                        }
                        dns_domain_free(
                            &mut (*z).ns[(*z).level as (usize)][j as (usize)] as (*mut *mut u8),
                        );
                        j = j + 1;
                    }
                    pos = 0u32;
                    j = 0i32;
                    'loop228: loop {
                        if {
                            pos = dns_packet_getname(
                                cached as (*const u8),
                                cachedlen,
                                pos,
                                &mut t1 as (*mut *mut u8),
                            );
                            pos
                        } == 0
                        {
                            _currentBlock = 183;
                            break;
                        }
                        log_cachedns(d as (*const u8), t1 as (*const u8));
                        if !(j < 16i32) {
                            continue;
                        }
                        if dns_domain_copy(
                            &mut (*z).ns[(*z).level as (usize)][{
                                                                    let _old = j;
                                                                    j = j + 1;
                                                                    _old
                                                                } as
                                                                    (usize)] as
                                (*mut *mut u8),
                            t1 as (*const u8),
                        ) == 0
                        {
                            _currentBlock = 348;
                            break;
                        }
                    }
                } else {
                    j = 0i32;
                    'loop234: loop {
                        if !(j < 16i32) {
                            break;
                        }
                        dns_domain_free(
                            &mut (*z).ns[(*z).level as (usize)][j as (usize)] as (*mut *mut u8),
                        );
                        j = j + 1;
                    }
                    (*z).control[(*z).level as (usize)] = d;
                    _currentBlock = 183;
                }
            }
        } else if _currentBlock == 285 {
            if (*z).level == 0 {
                if !(*z).alias[(16i32 - 1i32) as (usize)].is_null() {
                    _currentBlock = 348;
                    continue;
                }
                j = 16i32 - 1i32;
                'loop288: loop {
                    if !(j > 0i32) {
                        break;
                    }
                    (*z).alias[j as (usize)] = (*z).alias[(j - 1i32) as (usize)];
                    j = j - 1;
                }
                j = 16i32 - 1i32;
                'loop290: loop {
                    if !(j > 0i32) {
                        break;
                    }
                    (*z).aliasttl[j as (usize)] = (*z).aliasttl[(j - 1i32) as (usize)];
                    j = j - 1;
                }
                (*z).alias[0usize] = (*z).name[0usize];
                (*z).aliasttl[0usize] = ttl;
                (*z).name[0usize] = 0i32 as (*mut u8);
            }
            if dns_domain_copy(
                &mut (*z).name[(*z).level as (usize)] as (*mut *mut u8),
                cname as (*const u8),
            ) == 0
            {
                _currentBlock = 348;
            } else {
                _currentBlock = 203;
            }
        } else if _currentBlock == 301 {
            if (*z).level != 0 {
                _currentBlock = 323;
                continue;
            }
            if rqa(z) == 0 {
                _currentBlock = 348;
            } else {
                _currentBlock = 303;
                break;
            }
        } else if _currentBlock == 323 {
            dns_domain_free(&mut (*z).name[(*z).level as (usize)] as (*mut *mut u8));
            j = 0i32;
            'loop324: loop {
                if !(j < 16i32) {
                    break;
                }
                dns_domain_free(
                    &mut (*z).ns[(*z).level as (usize)][j as (usize)] as (*mut *mut u8),
                );
                j = j + 1;
            }
            (*z).level = (*z).level.wrapping_sub(1u32);
            _currentBlock = 183;
        } else {
            cleanup(z);
            if !records.is_null() {
                _currentBlock = 349;
                break;
            } else {
                _currentBlock = 350;
                break;
            }
        }
    }
    if _currentBlock == 193 {
        return 0i32;
    } else if _currentBlock == 196 {
        response_servfail();
        cleanup(z);
        return 1i32;
    } else if _currentBlock == 240 {
        cleanup(z);
        return 1i32;
    } else if _currentBlock == 248 {
        cleanup(z);
        return 1i32;
    } else if _currentBlock == 268 {
        cleanup(z);
        return 1i32;
    } else if _currentBlock == 272 {
        cleanup(z);
        return 1i32;
    } else if _currentBlock == 279 {
        cleanup(z);
        return 1i32;
    } else if _currentBlock == 299 {
        cleanup(z);
        return 1i32;
    } else if _currentBlock == 303 {
        response_nxdomain();
        cleanup(z);
        return 1i32;
    } else if _currentBlock == 309 {
        response_rfinish(6i32);
        _currentBlock = 310;
    } else if _currentBlock == 316 {
        response_rfinish(6i32);
        _currentBlock = 317;
    } else if _currentBlock == 349 {
        alloc::free(records as (*mut u8));
        records = 0i32 as (*mut u32);
        _currentBlock = 350;
    }
    if _currentBlock == 310 {
        cleanup(z);
        log_stats();
        1i32
    } else if _currentBlock == 317 {
        cleanup(z);
        1i32
    } else {
        -1i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn query_start(
    mut z: *mut query,
    mut dn: *mut u8,
    mut type_: *mut u8,
    mut class: *mut u8,
    mut localip: *mut u8,
) -> i32 {
    if byte::diff(type_, 2u32, (*b"\0\xFC\0").as_ptr() as (*mut u8)) == 0 {
        errno = libc::EPERM;
        -1i32
    } else {
        cleanup(z);
        (*z).level = 0u32;
        (*z).loopvar = 0u32;
        (if dns_domain_copy(&mut (*z).name[0usize] as (*mut *mut u8), dn as (*const u8)) == 0 {
             -1i32
         } else {
             byte::copy((*z).type_.as_mut_ptr(), 2u32, type_);
             byte::copy((*z).class.as_mut_ptr(), 2u32, class);
             byte::copy((*z).localip.as_mut_ptr(), 4u32, localip);
             doit(z, 0i32)
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
pub unsafe extern "C" fn query_get(
    mut z: *mut query,
    mut x: *mut pollfd,
    mut stamp: *mut taia,
) -> i32 {
    let switch1 = dns_transmit_get(
        &mut (*z).dt as (*mut dns_transmit),
        x as (*const pollfd),
        stamp as (*const taia),
    );
    if switch1 == -1i32 {
        doit(z, -1i32)
    } else if switch1 == 1i32 {
        doit(z, 1i32)
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn query_io(mut z: *mut query, mut x: *mut pollfd, mut deadline: *mut taia) {
    dns_transmit_io(&mut (*z).dt as (*mut dns_transmit), x, deadline);
}
