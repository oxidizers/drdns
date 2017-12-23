use alloc;
use byte;
use libc;
use tai::Tai;
use uint32;

pub static mut MOTION: usize = 0usize;

static mut X: *mut u8 = 0i32 as (*mut u8);
static mut SIZE: u32 = 0u32;
static mut HSIZE: u32 = 0u32;
static mut WRITER: u32 = 0u32;
static mut OLDEST: u32 = 0u32;
static mut UNUSED: u32 = 0u32;

pub unsafe fn init(mut cachesize: u32) -> i32 {
    if !X.is_null() {
        alloc::free(X);
        X = 0i32 as (*mut u8);
    }
    if cachesize > 1000000000u32 {
        cachesize = 1000000000u32;
    }
    if cachesize < 100u32 {
        cachesize = 100u32;
    }
    SIZE = cachesize;
    HSIZE = 4u32;
    'loop7: loop {
        if !(HSIZE <= SIZE >> 5i32) {
            break;
        }
        HSIZE = HSIZE << 1i32;
    }
    X = alloc::alloc(SIZE);
    if X.is_null() {
        0i32
    } else {
        byte::zero(X, SIZE);
        WRITER = HSIZE;
        OLDEST = SIZE;
        UNUSED = SIZE;
        1i32
    }
}

pub unsafe fn get(key: *const u8, keylen: u32, datalen: *mut u32, ttl: *mut u32) -> *mut u8 {
    let current_block;
    let mut expire: Tai = ::std::mem::zeroed();
    let mut now: Tai = ::std::mem::zeroed();
    let mut pos: u32;
    let mut prevpos: u32;
    let mut nextpos: u32;
    let u: u32;
    let mut loopvar: u32;
    let mut d: f64;
    if X.is_null() {
        0i32 as (*mut u8)
    } else if keylen > 1000u32 {
        0i32 as (*mut u8)
    } else {
        prevpos = hash(key, keylen);
        pos = get4(prevpos);
        loopvar = 0u32;
        'loop3: loop {
            if pos == 0 {
                current_block = 4;
                break;
            }
            if get4(pos.wrapping_add(4u32)) == keylen {
                if pos.wrapping_add(20u32).wrapping_add(keylen) > SIZE {
                    impossible();
                }
                if byte::diff(
                    key as (*mut u8),
                    keylen,
                    X.offset(pos as (isize)).offset(20isize),
                ) == 0
                {
                    current_block = 11;
                    break;
                }
            }
            nextpos = prevpos ^ get4(pos);
            prevpos = pos;
            pos = nextpos;
            if {
                loopvar = loopvar.wrapping_add(1u32);
                loopvar
            } > 100u32
            {
                current_block = 10;
                break;
            }
        }
        (if current_block == 4 {
             0i32 as (*mut u8)
         } else if current_block == 10 {
             0i32 as (*mut u8)
         } else {
             Tai::unpack(
                X.offset(pos as (isize)).offset(12isize) as (*const u8),
                &mut expire as (*mut Tai),
            );
             Tai::now(&mut now as (*mut Tai));
             (if (*(&mut expire as (*mut Tai))).x < (*(&mut now as (*mut Tai))).x {
                  0i32 as (*mut u8)
              } else {
                  Tai::sub(
                    &mut expire as (*mut Tai),
                    &mut expire as (*mut Tai) as (*const Tai),
                    &mut now as (*mut Tai) as (*const Tai),
                );
                  d = (*(&mut expire as (*mut Tai))).x as (f64);
                  if d > 604800i32 as (f64) {
                      d = 604800i32 as (f64);
                  }
                  *ttl = d as (u32);
                  u = get4(pos.wrapping_add(8u32));
                  if u >
                      SIZE.wrapping_sub(pos).wrapping_sub(20u32).wrapping_sub(
                        keylen,
                    )
                {
                      impossible();
                  }
                  *datalen = u;
                  X.offset(pos as (isize)).offset(20isize).offset(
                    keylen as (isize),
                )
              })
         })
    }
}

pub unsafe fn set(key: *const u8, keylen: u32, data: *const u8, datalen: u32, mut ttl: u32) {
    let current_block;
    let mut now: Tai = ::std::mem::zeroed();
    let mut expire: Tai = ::std::mem::zeroed();
    let entrylen: u32;
    let keyhash: u32;
    let mut pos: u32;
    if X.is_null() {
    } else if keylen > 1000u32 {
    } else if datalen > 1000000u32 {
    } else if ttl == 0 {
    } else {
        if ttl > 604800u32 {
            ttl = 604800u32;
        }
        entrylen = keylen.wrapping_add(datalen).wrapping_add(20u32);
        'loop7: loop {
            if !(WRITER.wrapping_add(entrylen) > OLDEST) {
                current_block = 8;
                break;
            }
            if OLDEST == UNUSED {
                if WRITER <= HSIZE {
                    current_block = 18;
                    break;
                }
                UNUSED = WRITER;
                OLDEST = HSIZE;
                WRITER = HSIZE;
            }
            pos = get4(OLDEST);
            set4(pos, get4(pos) ^ OLDEST);
            OLDEST = OLDEST.wrapping_add(
                get4(OLDEST.wrapping_add(4u32))
                    .wrapping_add(get4(OLDEST.wrapping_add(8u32)))
                    .wrapping_add(20u32),
            );
            if OLDEST > UNUSED {
                impossible();
            }
            if !(OLDEST == UNUSED) {
                continue;
            }
            UNUSED = SIZE;
            OLDEST = SIZE;
        }
        (if current_block == 8 {
             keyhash = hash(key, keylen);
             Tai::now(&mut now as (*mut Tai));
             Tai::uint(&mut expire as (*mut Tai), ttl);
             Tai::add(
                &mut expire as (*mut Tai),
                &mut expire as (*mut Tai) as (*const Tai),
                &mut now as (*mut Tai) as (*const Tai),
            );
             pos = get4(keyhash);
             if pos != 0 {
                 set4(pos, get4(pos) ^ keyhash ^ WRITER);
             }
             set4(WRITER, pos ^ keyhash);
             set4(WRITER.wrapping_add(4u32), keylen);
             set4(WRITER.wrapping_add(8u32), datalen);
             Tai::pack(
                X.offset(WRITER as (isize)).offset(12isize),
                &mut expire as (*mut Tai) as (*const Tai),
            );
             byte::copy(
                X.offset(WRITER as (isize)).offset(20isize),
                keylen,
                key as (*mut u8),
            );
             byte::copy(
                X.offset(WRITER as (isize)).offset(20isize).offset(
                    keylen as (isize),
                ),
                datalen,
                data as (*mut u8),
            );
             set4(keyhash, WRITER);
             WRITER = WRITER.wrapping_add(entrylen);
             MOTION = MOTION.wrapping_add(entrylen as (usize));
         })
    }
}

unsafe fn hash(mut key: *const u8, mut keylen: u32) -> u32 {
    let mut result: u32 = 5381u32;
    'loop1: loop {
        if keylen == 0 {
            break;
        }
        result = (result << 5i32).wrapping_add(result);
        result = result ^ *key as (u32);
        key = key.offset(1isize);
        keylen = keylen.wrapping_sub(1u32);
    }
    result = result << 2i32;
    result = result & HSIZE.wrapping_sub(4u32);
    result
}

unsafe fn impossible() {
    libc::_exit(111);
}

unsafe fn get4(pos: u32) -> u32 {
    let mut result: u32 = 0;
    if pos > SIZE.wrapping_sub(4u32) {
        impossible();
    }
    uint32::unpack(
        X.offset(pos as (isize)) as (*const u8),
        &mut result as (*mut u32),
    );
    result
}

unsafe fn set4(pos: u32, u: u32) {
    if pos > SIZE.wrapping_sub(4u32) {
        impossible();
    }
    uint32::pack(X.offset(pos as (isize)), u);
}
