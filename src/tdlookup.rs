use byte;

extern "C" {
    fn case_lowerb(arg1: *mut u8, arg2: u32);
    fn cdb_find(arg1: *mut cdb, arg2: *const u8, arg3: u32) -> i32;
    fn cdb_findnext(arg1: *mut cdb, arg2: *const u8, arg3: u32) -> i32;
    fn cdb_findstart(arg1: *mut cdb);
    fn cdb_free(arg1: *mut cdb);
    fn cdb_init(arg1: *mut cdb, fd: i32);
    fn cdb_read(arg1: *mut cdb, arg2: *mut u8, arg3: u32, arg4: u32) -> i32;
    fn close(arg1: i32) -> i32;
    fn dns_domain_equal(arg1: *const u8, arg2: *const u8) -> i32;
    fn dns_domain_length(arg1: *const u8) -> u32;
    fn dns_packet_copy(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut u8, arg5: u32) -> u32;
    fn dns_packet_getname(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut *mut u8) -> u32;
    fn dns_packet_skipname(arg1: *const u8, arg2: u32, arg3: u32) -> u32;
    fn dns_random(arg1: u32) -> u32;
    fn open_read(arg1: *const u8) -> i32;
    static mut response: *mut u8;
    fn response_addbytes(arg1: *const u8, arg2: u32) -> i32;
    fn response_addname(arg1: *const u8) -> i32;
    static mut response_len: u32;
    fn response_nxdomain();
    fn response_rfinish(arg1: i32);
    fn response_rstart(arg1: *const u8, arg2: *const u8, arg3: u32) -> i32;
    fn tai_now(arg1: *mut Tai);
    fn tai_sub(arg1: *mut Tai, arg2: *const Tai, arg3: *const Tai);
    fn tai_unpack(arg1: *const u8, arg2: *mut Tai);
    fn uint16_unpack_big(arg1: *const u8, arg2: *mut u16);
    fn uint32_unpack_big(arg1: *const u8, arg2: *mut u32);
}

static mut d1: *mut u8 = 0 as (*mut u8);

static mut clientloc: [u8; 2] = [0u8; 2];

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

static mut now: Tai = Tai { x: 0usize };

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

static mut data: [u8; 32767] = [0u8; 32767];

static mut dlen: u32 = 0u32;

static mut dpos: u32 = 0u32;

static mut type_: [u8; 2] = [0u8; 2];

static mut ttl: u32 = 0u32;

unsafe extern "C" fn find(mut d: *mut u8, mut flagwild: i32) -> i32 {
    let mut _currentBlock;
    let mut r: i32;
    let mut ch: u8;
    let mut cutoff: Tai;
    let mut ttd: [u8; 8];
    let mut ttlstr: [u8; 4];
    let mut recordloc: [u8; 2];
    let mut newttl: f64;
    'loop1: loop {
        r = cdb_findnext(
            &mut c as (*mut cdb),
            d as (*const u8),
            dns_domain_length(d as (*const u8)),
        );
        if r <= 0i32 {
            _currentBlock = 29;
            break;
        }
        dlen = (*(&mut c as (*mut cdb))).dlen;
        if dlen as (usize) > ::std::mem::size_of::<[u8; 32767]>() {
            _currentBlock = 28;
            break;
        }
        if cdb_read(
            &mut c as (*mut cdb),
            data.as_mut_ptr(),
            dlen,
            (*(&mut c as (*mut cdb))).dpos,
        ) == -1i32
        {
            _currentBlock = 27;
            break;
        }
        dpos = dns_packet_copy(
            data.as_mut_ptr() as (*const u8),
            dlen,
            0u32,
            type_.as_mut_ptr(),
            2u32,
        );
        if dpos == 0 {
            _currentBlock = 26;
            break;
        }
        dpos = dns_packet_copy(
            data.as_mut_ptr() as (*const u8),
            dlen,
            dpos,
            &mut ch as (*mut u8),
            1u32,
        );
        if dpos == 0 {
            _currentBlock = 25;
            break;
        }
        if ch as (i32) == b'=' as (i32) + 1i32 || ch as (i32) == b'*' as (i32) + 1i32 {
            ch = (ch as (i32) - 1) as (u8);
            dpos = dns_packet_copy(
                data.as_mut_ptr() as (*const u8),
                dlen,
                dpos,
                recordloc.as_mut_ptr(),
                2u32,
            );
            if dpos == 0 {
                _currentBlock = 24;
                break;
            }
            if byte::diff(recordloc.as_mut_ptr(), 2u32, clientloc.as_mut_ptr()) != 0 {
                continue;
            }
        }
        if flagwild != (ch as (i32) == b'*' as (i32)) as (i32) {
            continue;
        }
        dpos = dns_packet_copy(
            data.as_mut_ptr() as (*const u8),
            dlen,
            dpos,
            ttlstr.as_mut_ptr(),
            4u32,
        );
        if dpos == 0 {
            _currentBlock = 23;
            break;
        }
        uint32_unpack_big(ttlstr.as_mut_ptr() as (*const u8), &mut ttl as (*mut u32));
        dpos = dns_packet_copy(
            data.as_mut_ptr() as (*const u8),
            dlen,
            dpos,
            ttd.as_mut_ptr(),
            8u32,
        );
        if dpos == 0 {
            _currentBlock = 22;
            break;
        }
        if byte::diff(
            ttd.as_mut_ptr(),
            8u32,
            (*b"\0\0\0\0\0\0\0\0\0").as_ptr() as (*mut u8),
        ) == 0
        {
            _currentBlock = 21;
            break;
        }
        tai_unpack(ttd.as_mut_ptr() as (*const u8), &mut cutoff as (*mut Tai));
        if ttl == 0u32 {
            if !((*(&mut cutoff as (*mut Tai))).x < (*(&mut now as (*mut Tai))).x) {
                _currentBlock = 16;
                break;
            }
        } else if !!((*(&mut cutoff as (*mut Tai))).x < (*(&mut now as (*mut Tai))).x) {
            _currentBlock = 21;
            break;
        }
    }
    if _currentBlock == 16 {
        tai_sub(
            &mut cutoff as (*mut Tai),
            &mut cutoff as (*mut Tai) as (*const Tai),
            &mut now as (*mut Tai) as (*const Tai),
        );
        newttl = (*(&mut cutoff as (*mut Tai))).x as (f64);
        if newttl <= 2.0f64 {
            newttl = 2.0f64;
        }
        if newttl >= 3600.0f64 {
            newttl = 3600.0f64;
        }
        ttl = newttl as (u32);
    } else if _currentBlock == 22 {
        return -1i32;
    } else if _currentBlock == 23 {
        return -1i32;
    } else if _currentBlock == 24 {
        return -1i32;
    } else if _currentBlock == 25 {
        return -1i32;
    } else if _currentBlock == 26 {
        return -1i32;
    } else if _currentBlock == 27 {
        return -1i32;
    } else if _currentBlock == 28 {
        return -1i32;
    } else if _currentBlock == 29 {
        return r;
    }
    1i32
}

unsafe extern "C" fn doname() -> i32 {
    dpos = dns_packet_getname(
        data.as_mut_ptr() as (*const u8),
        dlen,
        dpos,
        &mut d1 as (*mut *mut u8),
    );
    if dpos == 0 {
        0i32
    } else {
        response_addname(d1 as (*const u8))
    }
}

unsafe extern "C" fn dobytes(mut len: u32) -> i32 {
    let mut buf: [u8; 20];
    if len > 20u32 {
        0i32
    } else {
        dpos = dns_packet_copy(
            data.as_mut_ptr() as (*const u8),
            dlen,
            dpos,
            buf.as_mut_ptr(),
            len,
        );
        (if dpos == 0 {
             0i32
         } else {
             response_addbytes(buf.as_mut_ptr() as (*const u8), len)
         })
    }
}

unsafe extern "C" fn want(mut owner: *const u8, mut type_: *const u8) -> i32 {
    let mut _currentBlock;
    let mut pos: u32;
    static mut d: *mut u8 = 0 as (*mut u8);
    let mut x: [u8; 10];
    let mut datalen: u16;
    pos = dns_packet_skipname(response as (*const u8), response_len, 12u32);
    if pos == 0 {
        0i32
    } else {
        pos = pos.wrapping_add(4u32);
        'loop2: loop {
            if !(pos < response_len) {
                _currentBlock = 3;
                break;
            }
            pos = dns_packet_getname(
                response as (*const u8),
                response_len,
                pos,
                &mut d as (*mut *mut u8),
            );
            if pos == 0 {
                _currentBlock = 11;
                break;
            }
            pos = dns_packet_copy(
                response as (*const u8),
                response_len,
                pos,
                x.as_mut_ptr(),
                10u32,
            );
            if pos == 0 {
                _currentBlock = 10;
                break;
            }
            if dns_domain_equal(d as (*const u8), owner) != 0 {
                if byte::diff(type_ as (*mut u8), 2u32, x.as_mut_ptr()) == 0 {
                    _currentBlock = 9;
                    break;
                }
            }
            uint16_unpack_big(
                x.as_mut_ptr().offset(8isize) as (*const u8),
                &mut datalen as (*mut u16),
            );
            pos = pos.wrapping_add(datalen as (u32));
        }
        (if _currentBlock == 3 {
             1i32
         } else if _currentBlock == 9 {
             0i32
         } else if _currentBlock == 10 {
             0i32
         } else {
             0i32
         })
    }
}

unsafe extern "C" fn doit(mut q: *mut u8, mut qtype: *mut u8) -> i32 {
    let mut _currentBlock;
    let mut bpos: u32;
    let mut anpos: u32;
    let mut aupos: u32;
    let mut arpos: u32;
    let mut control: *mut u8;
    let mut wild: *mut u8;
    let mut flaggavesoa: i32;
    let mut flagfound: i32;
    let mut r: i32;
    let mut flagns: i32;
    let mut flagauthoritative: i32;
    let mut x: [u8; 20];
    let mut u16: u16;
    let mut addr: [[u8; 4]; 8];
    let mut addrnum: i32;
    let mut addrttl: u32;
    let mut i: i32;
    anpos = response_len;
    control = q;
    'loop1: loop {
        flagns = 0i32;
        flagauthoritative = 0i32;
        cdb_findstart(&mut c as (*mut cdb));
        'loop2: loop {
            if {
                r = find(control, 0i32);
                r
            } == 0
            {
                break;
            }
            if r == -1i32 {
                _currentBlock = 118;
                break 'loop1;
            }
            if byte::diff(
                type_.as_mut_ptr(),
                2u32,
                (*b"\0\x06\0").as_ptr() as (*mut u8),
            ) == 0
            {
                flagauthoritative = 1i32;
            }
            if !(byte::diff(
                type_.as_mut_ptr(),
                2u32,
                (*b"\0\x02\0").as_ptr() as (*mut u8),
            ) == 0)
            {
                continue;
            }
            flagns = 1i32;
        }
        if flagns != 0 {
            _currentBlock = 7;
            break;
        }
        if *control == 0 {
            _currentBlock = 6;
            break;
        }
        control = control.offset(*control as (isize));
        control = control.offset(1isize);
    }
    if _currentBlock == 6 {
        0i32
    } else if _currentBlock == 7 {
        if flagauthoritative == 0 {
            let _rhs = !4i32;
            let _lhs = &mut *response.offset(2isize);
            *_lhs = (*_lhs as (i32) & _rhs) as (u8);
        } else {
            flaggavesoa = 0i32;
            flagfound = 0i32;
            wild = q;
            'loop9: loop {
                addrnum = 0i32;
                addrttl = 0u32;
                cdb_findstart(&mut c as (*mut cdb));
                'loop10: loop {
                    if {
                        r = find(wild, (wild != q) as (i32));
                        r
                    } == 0
                    {
                        break;
                    }
                    if r == -1i32 {
                        _currentBlock = 57;
                        break 'loop9;
                    }
                    flagfound = 1i32;
                    if flaggavesoa != 0 &&
                        (byte::diff(
                            type_.as_mut_ptr(),
                            2u32,
                            (*b"\0\x06\0").as_ptr() as (*mut u8),
                        ) == 0)
                    {
                        continue;
                    }
                    if byte::diff(type_.as_mut_ptr(), 2u32, qtype) != 0 &&
                        (byte::diff(qtype, 2u32, (*b"\0\xFF\0").as_ptr() as (*mut u8)) != 0) &&
                        (byte::diff(
                            type_.as_mut_ptr(),
                            2u32,
                            (*b"\0\x05\0").as_ptr() as (*mut u8),
                        ) != 0)
                    {
                        continue;
                    }
                    if byte::diff(
                        type_.as_mut_ptr(),
                        2u32,
                        (*b"\0\x01\0").as_ptr() as (*mut u8),
                    ) == 0 && (dlen.wrapping_sub(dpos) == 4u32)
                    {
                        addrttl = ttl;
                        i = dns_random((addrnum + 1i32) as (u32)) as (i32);
                        if i < 8i32 {
                            if i < addrnum && (addrnum < 8i32) {
                                byte::copy(
                                    addr[addrnum as (usize)].as_mut_ptr(),
                                    4u32,
                                    addr[i as (usize)].as_mut_ptr(),
                                );
                            }
                            byte::copy(
                                addr[i as (usize)].as_mut_ptr(),
                                4u32,
                                data.as_mut_ptr().offset(dpos as (isize)),
                            );
                        }
                        if !(addrnum < 1000000i32) {
                            continue;
                        }
                        addrnum = addrnum + 1;
                    } else {
                        if response_rstart(
                            q as (*const u8),
                            type_.as_mut_ptr() as (*const u8),
                            ttl,
                        ) == 0
                        {
                            _currentBlock = 50;
                            break 'loop9;
                        }
                        if byte::diff(
                            type_.as_mut_ptr(),
                            2u32,
                            (*b"\0\x02\0").as_ptr() as (*mut u8),
                        ) == 0 ||
                            byte::diff(
                                type_.as_mut_ptr(),
                                2u32,
                                (*b"\0\x05\0").as_ptr() as (*mut u8),
                            ) == 0 ||
                            byte::diff(
                                type_.as_mut_ptr(),
                                2u32,
                                (*b"\0\x0C\0").as_ptr() as (*mut u8),
                            ) == 0
                        {
                            if doname() == 0 {
                                _currentBlock = 49;
                                break 'loop9;
                            }
                        } else if byte::diff(
                            type_.as_mut_ptr(),
                            2u32,
                            (*b"\0\x0F\0").as_ptr() as (*mut u8),
                        ) == 0
                        {
                            if dobytes(2u32) == 0 {
                                _currentBlock = 46;
                                break 'loop9;
                            }
                            if doname() == 0 {
                                _currentBlock = 45;
                                break 'loop9;
                            }
                        } else if byte::diff(
                            type_.as_mut_ptr(),
                            2u32,
                            (*b"\0\x06\0").as_ptr() as (*mut u8),
                        ) == 0
                        {
                            if doname() == 0 {
                                _currentBlock = 42;
                                break 'loop9;
                            }
                            if doname() == 0 {
                                _currentBlock = 41;
                                break 'loop9;
                            }
                            if dobytes(20u32) == 0 {
                                _currentBlock = 40;
                                break 'loop9;
                            }
                            flaggavesoa = 1i32;
                        } else if response_addbytes(
                            data.as_mut_ptr().offset(dpos as (isize)) as (*const u8),
                            dlen.wrapping_sub(dpos),
                        ) == 0
                        {
                            _currentBlock = 35;
                            break 'loop9;
                        }
                        response_rfinish(6i32);
                    }
                }
                i = 0i32;
                'loop12: loop {
                    if !(i < addrnum) {
                        break;
                    }
                    if i < 8i32 {
                        if response_rstart(q as (*const u8), (*b"\0\x01\0").as_ptr(), addrttl) ==
                            0
                        {
                            _currentBlock = 25;
                            break 'loop9;
                        }
                        if response_addbytes(
                            addr[i as (usize)].as_mut_ptr() as (*const u8),
                            4u32,
                        ) == 0
                        {
                            _currentBlock = 24;
                            break 'loop9;
                        }
                        response_rfinish(6i32);
                    }
                    i = i + 1;
                }
                if flagfound != 0 {
                    _currentBlock = 17;
                    break;
                }
                if wild == control {
                    _currentBlock = 17;
                    break;
                }
                if *wild == 0 {
                    _currentBlock = 17;
                    break;
                }
                wild = wild.offset(*wild as (isize));
                wild = wild.offset(1isize);
            }
            if _currentBlock == 17 {
                if flagfound == 0 {
                    response_nxdomain();
                }
            } else if _currentBlock == 24 {
                return 0i32;
            } else if _currentBlock == 25 {
                return 0i32;
            } else if _currentBlock == 35 {
                return 0i32;
            } else if _currentBlock == 40 {
                return 0i32;
            } else if _currentBlock == 41 {
                return 0i32;
            } else if _currentBlock == 42 {
                return 0i32;
            } else if _currentBlock == 45 {
                return 0i32;
            } else if _currentBlock == 46 {
                return 0i32;
            } else if _currentBlock == 49 {
                return 0i32;
            } else if _currentBlock == 50 {
                return 0i32;
            } else {
                return 0i32;
            }
        }
        aupos = response_len;
        if flagauthoritative != 0 && (aupos == anpos) {
            cdb_findstart(&mut c as (*mut cdb));
            'loop72: loop {
                if {
                    r = find(control, 0i32);
                    r
                } == 0
                {
                    _currentBlock = 80;
                    break;
                }
                if r == -1i32 {
                    _currentBlock = 112;
                    break;
                }
                if byte::diff(
                    type_.as_mut_ptr(),
                    2u32,
                    (*b"\0\x06\0").as_ptr() as (*mut u8),
                ) == 0
                {
                    _currentBlock = 75;
                    break;
                }
            }
            if _currentBlock == 80 {
            } else if _currentBlock == 75 {
                if response_rstart(control as (*const u8), (*b"\0\x06\0").as_ptr(), ttl) == 0 {
                    return 0i32;
                } else if doname() == 0 {
                    return 0i32;
                } else if doname() == 0 {
                    return 0i32;
                } else if dobytes(20u32) == 0 {
                    return 0i32;
                } else {
                    response_rfinish(8i32);
                }
            } else {
                return 0i32;
            }
        } else if want(control as (*const u8), (*b"\0\x02\0").as_ptr()) != 0 {
            cdb_findstart(&mut c as (*mut cdb));
            'loop62: loop {
                if {
                    r = find(control, 0i32);
                    r
                } == 0
                {
                    _currentBlock = 80;
                    break;
                }
                if r == -1i32 {
                    _currentBlock = 70;
                    break;
                }
                if !(byte::diff(
                    type_.as_mut_ptr(),
                    2u32,
                    (*b"\0\x02\0").as_ptr() as (*mut u8),
                ) == 0)
                {
                    continue;
                }
                if response_rstart(control as (*const u8), (*b"\0\x02\0").as_ptr(), ttl) == 0 {
                    _currentBlock = 69;
                    break;
                }
                if doname() == 0 {
                    _currentBlock = 68;
                    break;
                }
                response_rfinish(8i32);
            }
            if _currentBlock == 80 {
            } else if _currentBlock == 68 {
                return 0i32;
            } else if _currentBlock == 69 {
                return 0i32;
            } else {
                return 0i32;
            }
        }
        arpos = response_len;
        bpos = anpos;
        'loop81: loop {
            if !(bpos < arpos) {
                _currentBlock = 82;
                break;
            }
            bpos = dns_packet_skipname(response as (*const u8), arpos, bpos);
            if bpos == 0 {
                _currentBlock = 107;
                break;
            }
            bpos = dns_packet_copy(response as (*const u8), arpos, bpos, x.as_mut_ptr(), 10u32);
            if bpos == 0 {
                _currentBlock = 106;
                break;
            }
            if byte::diff(x.as_mut_ptr(), 2u32, (*b"\0\x02\0").as_ptr() as (*mut u8)) == 0 ||
                byte::diff(x.as_mut_ptr(), 2u32, (*b"\0\x0F\0").as_ptr() as (*mut u8)) == 0
            {
                if byte::diff(x.as_mut_ptr(), 2u32, (*b"\0\x02\0").as_ptr() as (*mut u8)) == 0 {
                    if dns_packet_getname(
                        response as (*const u8),
                        arpos,
                        bpos,
                        &mut d1 as (*mut *mut u8),
                    ) == 0
                    {
                        _currentBlock = 105;
                        break;
                    }
                } else if dns_packet_getname(
                    response as (*const u8),
                    arpos,
                    bpos.wrapping_add(2u32),
                    &mut d1 as (*mut *mut u8),
                ) == 0
                {
                    _currentBlock = 91;
                    break;
                }
                case_lowerb(d1, dns_domain_length(d1 as (*const u8)));
                if want(d1 as (*const u8), (*b"\0\x01\0").as_ptr()) != 0 {
                    cdb_findstart(&mut c as (*mut cdb));
                    'loop95: loop {
                        if {
                            r = find(d1, 0i32);
                            r
                        } == 0
                        {
                            break;
                        }
                        if r == -1i32 {
                            _currentBlock = 104;
                            break 'loop81;
                        }
                        if !(byte::diff(
                            type_.as_mut_ptr(),
                            2u32,
                            (*b"\0\x01\0").as_ptr() as (*mut u8),
                        ) == 0)
                        {
                            continue;
                        }
                        if response_rstart(d1 as (*const u8), (*b"\0\x01\0").as_ptr(), ttl) == 0 {
                            _currentBlock = 103;
                            break 'loop81;
                        }
                        if dobytes(4u32) == 0 {
                            _currentBlock = 102;
                            break 'loop81;
                        }
                        response_rfinish(10i32);
                    }
                }
            }
            uint16_unpack_big(
                x.as_mut_ptr().offset(8isize) as (*const u8),
                &mut u16 as (*mut u16),
            );
            bpos = bpos.wrapping_add(u16 as (u32));
        }
        (if _currentBlock == 82 {
             if flagauthoritative != 0 && (response_len > 512u32) {
                 byte::zero(response.offset(10isize), 2u32);
                 response_len = arpos;
                 if response_len > 512u32 {
                     byte::zero(response.offset(8isize), 2u32);
                     response_len = aupos;
                 }
             }
             1i32
         } else if _currentBlock == 91 {
             0i32
         } else if _currentBlock == 102 {
             0i32
         } else if _currentBlock == 103 {
             0i32
         } else if _currentBlock == 104 {
             0i32
         } else if _currentBlock == 105 {
             0i32
         } else if _currentBlock == 106 {
             0i32
         } else {
             0i32
         })
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn respond(mut q: *mut u8, mut qtype: *mut u8, mut ip: *mut u8) -> i32 {
    let mut fd: i32;
    let mut r: i32;
    let mut key: [u8; 6];
    tai_now(&mut now as (*mut Tai));
    fd = open_read((*b"data.cdb\0").as_ptr());
    if fd == -1i32 {
        0i32
    } else {
        cdb_init(&mut c as (*mut cdb), fd);
        byte::zero(clientloc.as_mut_ptr(), 2u32);
        key[0usize] = 0u8;
        key[1usize] = b'%';
        byte::copy(key.as_mut_ptr().offset(2isize), 4u32, ip);
        r = cdb_find(&mut c as (*mut cdb), key.as_mut_ptr() as (*const u8), 6u32);
        if r == 0 {
            r = cdb_find(&mut c as (*mut cdb), key.as_mut_ptr() as (*const u8), 5u32);
        }
        if r == 0 {
            r = cdb_find(&mut c as (*mut cdb), key.as_mut_ptr() as (*const u8), 4u32);
        }
        if r == 0 {
            r = cdb_find(&mut c as (*mut cdb), key.as_mut_ptr() as (*const u8), 3u32);
        }
        if r == 0 {
            r = cdb_find(&mut c as (*mut cdb), key.as_mut_ptr() as (*const u8), 2u32);
        }
        (if r == -1i32 {
             0i32
         } else {
             if r != 0 && ((*(&mut c as (*mut cdb))).dlen == 2u32) {
                 if cdb_read(
                    &mut c as (*mut cdb),
                    clientloc.as_mut_ptr(),
                    2u32,
                    (*(&mut c as (*mut cdb))).dpos,
                ) == -1i32
                {
                     return 0i32;
                 }
             }
             r = doit(q, qtype);
             cdb_free(&mut c as (*mut cdb));
             close(fd);
             r
         })
    }
}
