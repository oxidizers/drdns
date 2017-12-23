//! `dns/random.rs`: DNS randomization utilities

use taia::TaiA;
use libc;
use uint32;

static mut SEED: [u32; 32] = [0u32; 32];

static mut IN: [u32; 12] = [0u32; 12];

static mut OUT: [u32; 8] = [0u32; 8];

static mut OUTLEFT: i32 = 0i32;

pub unsafe fn init(data: *const u8) {
    let mut i: i32;
    let mut t: TaiA = ::std::mem::zeroed();
    let mut tpack: [u8; 16] = [0u8; 16];
    i = 0i32;
    'loop1: loop {
        if !(i < 32i32) {
            break;
        }
        uint32::unpack(
            data.offset((4i32 * i) as (isize)),
            SEED.as_mut_ptr().offset(i as (isize)),
        );
        i = i + 1;
    }
    TaiA::now(&mut t as (*mut TaiA));
    TaiA::pack(tpack.as_mut_ptr(), &mut t as (*mut TaiA) as (*const TaiA));
    i = 0i32;
    'loop3: loop {
        if !(i < 4i32) {
            break;
        }
        uint32::unpack(
            tpack.as_mut_ptr().offset((4i32 * i) as (isize)) as (*const u8),
            IN.as_mut_ptr().offset(4isize).offset(i as (isize)),
        );
        i = i + 1;
    }
    IN[8usize] = libc::getpid() as (u32);
    IN[9usize] = libc::getppid() as (u32);
}

pub unsafe fn random(n: u32) -> u32 {
    if n == 0 {
        0u32
    } else {
        if OUTLEFT == 0 {
            if {
                let _rhs = 1;
                let _lhs = &mut IN[0usize];
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                *_lhs
            } == 0
            {
                if {
                    let _rhs = 1;
                    let _lhs = &mut IN[1usize];
                    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                    *_lhs
                } == 0
                {
                    if {
                        let _rhs = 1;
                        let _lhs = &mut IN[2usize];
                        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                        *_lhs
                    } == 0
                    {
                        let _rhs = 1;
                        let _lhs = &mut IN[3usize];
                        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                    }
                }
            }
            surf();
            OUTLEFT = 8i32;
        }
        OUT[{
                OUTLEFT = OUTLEFT - 1;
                OUTLEFT
            } as (usize)]
            .wrapping_rem(n)
    }
}

unsafe fn surf() {
    let mut t: [u32; 12] = [0u32; 12];
    let mut x: u32;
    let mut sum: u32 = 0u32;
    let mut r: i32;
    let mut i: i32;
    let mut loopvar: i32;
    i = 0i32;
    'loop1: loop {
        if !(i < 12i32) {
            break;
        }
        t[i as (usize)] = IN[i as (usize)] ^ SEED[(12i32 + i) as (usize)];
        i = i + 1;
    }
    i = 0i32;
    'loop3: loop {
        if !(i < 8i32) {
            break;
        }
        OUT[i as (usize)] = SEED[(24i32 + i) as (usize)];
        i = i + 1;
    }
    x = t[11usize];
    loopvar = 0i32;
    'loop5: loop {
        if !(loopvar < 2i32) {
            break;
        }
        r = 0i32;
        'loop8: loop {
            if !(r < 16i32) {
                break;
            }
            sum = sum.wrapping_add(0x9e3779b9u32);
            x = {
                let _rhs = (x ^ SEED[0usize]).wrapping_add(sum) ^ (x << 5i32 | x >> 32i32 - 5i32);
                let _lhs = &mut t[0usize];
                *_lhs = (*_lhs).wrapping_add(_rhs);
                *_lhs
            };
            x = {
                let _rhs = (x ^ SEED[1usize]).wrapping_add(sum) ^ (x << 7i32 | x >> 32i32 - 7i32);
                let _lhs = &mut t[1usize];
                *_lhs = (*_lhs).wrapping_add(_rhs);
                *_lhs
            };
            x = {
                let _rhs = (x ^ SEED[2usize]).wrapping_add(sum) ^ (x << 9i32 | x >> 32i32 - 9i32);
                let _lhs = &mut t[2usize];
                *_lhs = (*_lhs).wrapping_add(_rhs);
                *_lhs
            };
            x = {
                let _rhs = (x ^ SEED[3usize]).wrapping_add(sum) ^ (x << 13i32 | x >> 32i32 - 13i32);
                let _lhs = &mut t[3usize];
                *_lhs = (*_lhs).wrapping_add(_rhs);
                *_lhs
            };
            x = {
                let _rhs = (x ^ SEED[4usize]).wrapping_add(sum) ^ (x << 5i32 | x >> 32i32 - 5i32);
                let _lhs = &mut t[4usize];
                *_lhs = (*_lhs).wrapping_add(_rhs);
                *_lhs
            };
            x = {
                let _rhs = (x ^ SEED[5usize]).wrapping_add(sum) ^ (x << 7i32 | x >> 32i32 - 7i32);
                let _lhs = &mut t[5usize];
                *_lhs = (*_lhs).wrapping_add(_rhs);
                *_lhs
            };
            x = {
                let _rhs = (x ^ SEED[6usize]).wrapping_add(sum) ^ (x << 9i32 | x >> 32i32 - 9i32);
                let _lhs = &mut t[6usize];
                *_lhs = (*_lhs).wrapping_add(_rhs);
                *_lhs
            };
            x = {
                let _rhs = (x ^ SEED[7usize]).wrapping_add(sum) ^ (x << 13i32 | x >> 32i32 - 13i32);
                let _lhs = &mut t[7usize];
                *_lhs = (*_lhs).wrapping_add(_rhs);
                *_lhs
            };
            x = {
                let _rhs = (x ^ SEED[8usize]).wrapping_add(sum) ^ (x << 5i32 | x >> 32i32 - 5i32);
                let _lhs = &mut t[8usize];
                *_lhs = (*_lhs).wrapping_add(_rhs);
                *_lhs
            };
            x = {
                let _rhs = (x ^ SEED[9usize]).wrapping_add(sum) ^ (x << 7i32 | x >> 32i32 - 7i32);
                let _lhs = &mut t[9usize];
                *_lhs = (*_lhs).wrapping_add(_rhs);
                *_lhs
            };
            x = {
                let _rhs = (x ^ SEED[10usize]).wrapping_add(sum) ^ (x << 9i32 | x >> 32i32 - 9i32);
                let _lhs = &mut t[10usize];
                *_lhs = (*_lhs).wrapping_add(_rhs);
                *_lhs
            };
            x = {
                let _rhs = (x ^ SEED[11usize]).wrapping_add(sum) ^
                    (x << 13i32 | x >> 32i32 - 13i32);
                let _lhs = &mut t[11usize];
                *_lhs = (*_lhs).wrapping_add(_rhs);
                *_lhs
            };
            r = r + 1;
        }
        i = 0i32;
        'loop10: loop {
            if !(i < 8i32) {
                break;
            }
            let _rhs = t[(i + 4i32) as (usize)];
            let _lhs = &mut OUT[i as (usize)];
            *_lhs = *_lhs ^ _rhs;
            i = i + 1;
        }
        loopvar = loopvar + 1;
    }
}
