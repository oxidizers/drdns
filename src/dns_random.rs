extern {
    fn getpid() -> i32;
    fn getppid() -> i32;
    fn taia_now(arg1 : *mut taia);
    fn taia_pack(arg1 : *mut u8, arg2 : *const taia);
    fn uint32_unpack(arg1 : *const u8, arg2 : *mut u32);
}

static mut seed : [u32; 32] = [0u32; 32];

static mut in_ : [u32; 12] = [0u32; 12];

static mut out : [u32; 8] = [0u32; 8];

static mut outleft : i32 = 0i32;

#[derive(Copy)]
#[repr(C)]
pub struct tai {
    pub x : usize,
}

impl Clone for tai {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct taia {
    pub sec : tai,
    pub nano : usize,
    pub atto : usize,
}

impl Clone for taia {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn dns_random_init(mut data : *const u8) {
    let mut i : i32;
    let mut t : taia;
    let mut tpack : [u8; 16];
    i = 0i32;
    'loop1: loop {
        if !(i < 32i32) {
            break;
        }
        uint32_unpack(
            data.offset((4i32 * i) as (isize)),
            seed.as_mut_ptr().offset(i as (isize))
        );
        i = i + 1;
    }
    taia_now(&mut t as (*mut taia));
    taia_pack(
        tpack.as_mut_ptr(),
        &mut t as (*mut taia) as (*const taia)
    );
    i = 0i32;
    'loop3: loop {
        if !(i < 4i32) {
            break;
        }
        uint32_unpack(
            tpack.as_mut_ptr().offset((4i32 * i) as (isize)) as (*const u8),
            in_.as_mut_ptr().offset(4isize).offset(i as (isize))
        );
        i = i + 1;
    }
    in_[8usize] = getpid() as (u32);
    in_[9usize] = getppid() as (u32);
}

unsafe extern fn surf() {
    let mut t : [u32; 12];
    let mut x : u32;
    let mut sum : u32 = 0u32;
    let mut r : i32;
    let mut i : i32;
    let mut loop : i32;
    i = 0i32;
    'loop1: loop {
        if !(i < 12i32) {
            break;
        }
        t[i as (usize)] = in_[i as (usize)] ^ seed[(12i32 + i) as (usize)];
        i = i + 1;
    }
    i = 0i32;
    'loop3: loop {
        if !(i < 8i32) {
            break;
        }
        out[i as (usize)] = seed[(24i32 + i) as (usize)];
        i = i + 1;
    }
    x = t[11usize];
    loop = 0i32;
    'loop5: loop {
        if !(loop < 2i32) {
            break;
        }
        r = 0i32;
        'loop8: loop {
            if !(r < 16i32) {
                break;
            }
            sum = sum.wrapping_add(0x9e3779b9u32);
            x = {
                    let _rhs
                        = (x ^ seed[0usize]).wrapping_add(
                              sum
                          ) ^ (x << 5i32 | x >> 32i32 - 5i32);
                    let _lhs = &mut t[0usize];
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                    *_lhs
                };
            x = {
                    let _rhs
                        = (x ^ seed[1usize]).wrapping_add(
                              sum
                          ) ^ (x << 7i32 | x >> 32i32 - 7i32);
                    let _lhs = &mut t[1usize];
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                    *_lhs
                };
            x = {
                    let _rhs
                        = (x ^ seed[2usize]).wrapping_add(
                              sum
                          ) ^ (x << 9i32 | x >> 32i32 - 9i32);
                    let _lhs = &mut t[2usize];
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                    *_lhs
                };
            x = {
                    let _rhs
                        = (x ^ seed[3usize]).wrapping_add(
                              sum
                          ) ^ (x << 13i32 | x >> 32i32 - 13i32);
                    let _lhs = &mut t[3usize];
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                    *_lhs
                };
            x = {
                    let _rhs
                        = (x ^ seed[4usize]).wrapping_add(
                              sum
                          ) ^ (x << 5i32 | x >> 32i32 - 5i32);
                    let _lhs = &mut t[4usize];
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                    *_lhs
                };
            x = {
                    let _rhs
                        = (x ^ seed[5usize]).wrapping_add(
                              sum
                          ) ^ (x << 7i32 | x >> 32i32 - 7i32);
                    let _lhs = &mut t[5usize];
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                    *_lhs
                };
            x = {
                    let _rhs
                        = (x ^ seed[6usize]).wrapping_add(
                              sum
                          ) ^ (x << 9i32 | x >> 32i32 - 9i32);
                    let _lhs = &mut t[6usize];
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                    *_lhs
                };
            x = {
                    let _rhs
                        = (x ^ seed[7usize]).wrapping_add(
                              sum
                          ) ^ (x << 13i32 | x >> 32i32 - 13i32);
                    let _lhs = &mut t[7usize];
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                    *_lhs
                };
            x = {
                    let _rhs
                        = (x ^ seed[8usize]).wrapping_add(
                              sum
                          ) ^ (x << 5i32 | x >> 32i32 - 5i32);
                    let _lhs = &mut t[8usize];
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                    *_lhs
                };
            x = {
                    let _rhs
                        = (x ^ seed[9usize]).wrapping_add(
                              sum
                          ) ^ (x << 7i32 | x >> 32i32 - 7i32);
                    let _lhs = &mut t[9usize];
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                    *_lhs
                };
            x = {
                    let _rhs
                        = (x ^ seed[10usize]).wrapping_add(
                              sum
                          ) ^ (x << 9i32 | x >> 32i32 - 9i32);
                    let _lhs = &mut t[10usize];
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                    *_lhs
                };
            x = {
                    let _rhs
                        = (x ^ seed[11usize]).wrapping_add(
                              sum
                          ) ^ (x << 13i32 | x >> 32i32 - 13i32);
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
            let _lhs = &mut out[i as (usize)];
            *_lhs = *_lhs ^ _rhs;
            i = i + 1;
        }
        loop = loop + 1;
    }
}

#[no_mangle]
pub unsafe extern fn dns_random(mut n : u32) -> u32 {
    if n == 0 {
        0u32
    } else {
        if outleft == 0 {
            if {
                   let _rhs = 1;
                   let _lhs = &mut in_[0usize];
                   *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                   *_lhs
               } == 0 {
                if {
                       let _rhs = 1;
                       let _lhs = &mut in_[1usize];
                       *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                       *_lhs
                   } == 0 {
                    if {
                           let _rhs = 1;
                           let _lhs = &mut in_[2usize];
                           *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                           *_lhs
                       } == 0 {
                        let _rhs = 1;
                        let _lhs = &mut in_[3usize];
                        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                    }
                }
            }
            surf();
            outleft = 8i32;
        }
        out[
            {
                outleft = outleft - 1;
                outleft
            } as (usize)
        ].wrapping_rem(
            n
        )
    }
}
