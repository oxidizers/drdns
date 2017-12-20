use byte;
use uint16;
use uint32;

extern "C" {
    fn dns_domain_equal(arg1: *const u8, arg2: *const u8) -> i32;
    fn dns_domain_length(arg1: *const u8) -> u32;
}

#[no_mangle]
pub static mut response: [u8; 65535] = [0u8; 65535];

#[no_mangle]
pub static mut response_len: u32 = 0u32;

static mut tctarget: u32 = 0u32;

static mut name: [[u8; 128]; 100] = [[0u8; 128]; 100];

static mut name_ptr: [u32; 100] = [0u32; 100];

static mut name_num: u32 = 0u32;

#[no_mangle]
pub unsafe extern "C" fn response_addbytes(mut buf: *const u8, mut len: u32) -> i32 {
    if len > 65535u32.wrapping_sub(response_len) {
        0i32
    } else {
        byte::copy(
            response.as_mut_ptr().offset(response_len as (isize)),
            len,
            buf as (*mut u8),
        );
        response_len = response_len.wrapping_add(len);
        1i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn response_addname(mut d: *const u8) -> i32 {
    let mut _currentBlock;
    let mut dlen: u32;
    let mut i: u32;
    let mut buf: [u8; 2];
    dlen = dns_domain_length(d);
    'loop1: loop {
        if *d == 0 {
            _currentBlock = 2;
            break;
        }
        i = 0u32;
        'loop4: loop {
            if !(i < name_num) {
                break;
            }
            if dns_domain_equal(d, name[i as (usize)].as_mut_ptr() as (*const u8)) != 0 {
                _currentBlock = 13;
                break 'loop1;
            }
            i = i.wrapping_add(1u32);
        }
        if dlen <= 128u32 {
            if name_num < 100u32 {
                byte::copy(name[name_num as (usize)].as_mut_ptr(), dlen, d as (*mut u8));
                name_ptr[name_num as (usize)] = response_len;
                name_num = name_num.wrapping_add(1u32);
            }
        }
        i = *d as (u32);
        i = i.wrapping_add(1u32);
        if response_addbytes(d, i) == 0 {
            _currentBlock = 10;
            break;
        }
        d = d.offset(i as (isize));
        dlen = dlen.wrapping_sub(i);
    }
    if _currentBlock == 2 {
        response_addbytes(d, 1u32)
    } else if _currentBlock == 10 {
        0i32
    } else {
        uint16::pack_big(
            buf.as_mut_ptr(),
            49152u32.wrapping_add(name_ptr[i as (usize)]) as (u16),
        );
        response_addbytes(buf.as_mut_ptr() as (*const u8), 2u32)
    }
}

#[no_mangle]
pub unsafe extern "C" fn response_query(
    mut q: *const u8,
    mut qtype: *const u8,
    mut qclass: *const u8,
) -> i32 {
    response_len = 0u32;
    name_num = 0u32;
    if response_addbytes((*b"\0\0\x81\x80\0\x01\0\0\0\0\0\0\0").as_ptr(), 12u32) == 0 {
        0i32
    } else if response_addname(q) == 0 {
        0i32
    } else if response_addbytes(qtype, 2u32) == 0 {
        0i32
    } else if response_addbytes(qclass, 2u32) == 0 {
        0i32
    } else {
        tctarget = response_len;
        1i32
    }
}

static mut dpos: u32 = 0u32;

static mut flaghidettl: i32 = 0i32;

#[no_mangle]
pub unsafe extern "C" fn response_hidettl() {
    flaghidettl = 1i32;
}

#[no_mangle]
pub unsafe extern "C" fn response_rstart(
    mut d: *const u8,
    mut type_: *const u8,
    mut ttl: u32,
) -> i32 {
    let mut ttlstr: [u8; 4];
    if response_addname(d) == 0 {
        0i32
    } else if response_addbytes(type_, 2u32) == 0 {
        0i32
    } else if response_addbytes((*b"\0\x01\0").as_ptr(), 2u32) == 0 {
        0i32
    } else {
        if flaghidettl != 0 {
            ttl = 0u32;
        }
        uint32::pack_big(ttlstr.as_mut_ptr(), ttl);
        (if response_addbytes(ttlstr.as_mut_ptr() as (*const u8), 4u32) == 0 {
             0i32
         } else if response_addbytes((*b"\0\0\0").as_ptr(), 2u32) == 0 {
             0i32
         } else {
             dpos = response_len;
             1i32
         })
    }
}

#[no_mangle]
pub unsafe extern "C" fn response_rfinish(mut x: i32) {
    uint16::pack_big(
        response.as_mut_ptr().offset(dpos as (isize)).offset(
            -2isize,
        ),
        response_len.wrapping_sub(dpos) as (u16),
    );
    if {
        let _rhs = 1;
        let _lhs = &mut response[(x + 1i32) as (usize)];
        *_lhs = (*_lhs as (i32) + _rhs) as (u8);
        *_lhs
    } == 0
    {
        let _rhs = 1;
        let _lhs = &mut response[x as (usize)];
        *_lhs = (*_lhs as (i32) + _rhs) as (u8);
    }
}

#[no_mangle]
pub unsafe extern "C" fn response_cname(mut c: *const u8, mut d: *const u8, mut ttl: u32) -> i32 {
    if response_rstart(c, (*b"\0\x05\0").as_ptr(), ttl) == 0 {
        0i32
    } else if response_addname(d) == 0 {
        0i32
    } else {
        response_rfinish(6i32);
        1i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn response_nxdomain() {
    let _rhs = 3i32;
    let _lhs = &mut response[3usize];
    *_lhs = (*_lhs as (i32) | _rhs) as (u8);
    let _rhs = 4i32;
    let _lhs = &mut response[2usize];
    *_lhs = (*_lhs as (i32) | _rhs) as (u8);
}

#[no_mangle]
pub unsafe extern "C" fn response_servfail() {
    let _rhs = 2i32;
    let _lhs = &mut response[3usize];
    *_lhs = (*_lhs as (i32) | _rhs) as (u8);
}

#[no_mangle]
pub unsafe extern "C" fn response_id(mut id: *const u8) {
    byte::copy(response.as_mut_ptr(), 2u32, id as (*mut u8));
}

#[no_mangle]
pub unsafe extern "C" fn response_tc() {
    let _rhs = 2i32;
    let _lhs = &mut response[2usize];
    *_lhs = (*_lhs as (i32) | _rhs) as (u8);
    response_len = tctarget;
}
