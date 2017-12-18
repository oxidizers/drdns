extern {
    fn _exit(arg1 : i32);
    fn alloc(n : u32) -> *mut u8;
    fn alloc_free(x : *mut u8);
    fn byte_copy(to : *mut u8, n : u32, from : *mut u8);
    fn byte_diff(s : *mut u8, n : u32, t : *mut u8) -> i32;
    fn byte_zero(s : *mut u8, n : u32);
    fn tai_add(arg1 : *mut tai, arg2 : *const tai, arg3 : *const tai);
    fn tai_now(arg1 : *mut tai);
    fn tai_pack(arg1 : *mut u8, arg2 : *const tai);
    fn tai_sub(arg1 : *mut tai, arg2 : *const tai, arg3 : *const tai);
    fn tai_uint(arg1 : *mut tai, arg2 : u32);
    fn tai_unpack(arg1 : *const u8, arg2 : *mut tai);
    fn uint32_pack(arg1 : *mut u8, arg2 : u32);
    fn uint32_unpack(arg1 : *const u8, arg2 : *mut u32);
}

#[no_mangle]
pub static mut cache_motion : usize = 0usize;

static mut x : *mut u8 = 0i32 as (*mut u8);

static mut size : u32 = 0u32;

static mut hsize : u32 = 0u32;

static mut writer : u32 = 0u32;

static mut oldest : u32 = 0u32;

static mut unused : u32 = 0u32;

#[derive(Copy)]
#[repr(C)]
pub struct tai {
    pub x : usize,
}

impl Clone for tai {
    fn clone(&self) -> Self { *self }
}

unsafe extern fn hash(
    mut key : *const u8, mut keylen : u32
) -> u32 {
    let mut result : u32 = 5381u32;
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
    result = result & hsize.wrapping_sub(4u32);
    result
}

unsafe extern fn cache_impossible() {
    _exit(111i32);
}

unsafe extern fn get4(mut pos : u32) -> u32 {
    let mut result : u32;
    if pos > size.wrapping_sub(4u32) {
        cache_impossible();
    }
    uint32_unpack(
        x.offset(pos as (isize)) as (*const u8),
        &mut result as (*mut u32)
    );
    result
}

#[no_mangle]
pub unsafe extern fn cache_get(
    mut key : *const u8,
    mut keylen : u32,
    mut datalen : *mut u32,
    mut ttl : *mut u32
) -> *mut u8 {
    let mut _currentBlock;
    let mut expire : tai;
    let mut now : tai;
    let mut pos : u32;
    let mut prevpos : u32;
    let mut nextpos : u32;
    let mut u : u32;
    let mut loopvar : u32;
    let mut d : f64;
    if x.is_null() {
        0i32 as (*mut u8)
    } else if keylen > 1000u32 {
        0i32 as (*mut u8)
    } else {
        prevpos = hash(key,keylen);
        pos = get4(prevpos);
        loopvar = 0u32;
        'loop3: loop {
            if pos == 0 {
                _currentBlock = 4;
                break;
            }
            if get4(pos.wrapping_add(4u32)) == keylen {
                if pos.wrapping_add(20u32).wrapping_add(keylen) > size {
                    cache_impossible();
                }
                if byte_diff(
                       key as (*mut u8),
                       keylen,
                       x.offset(pos as (isize)).offset(20isize)
                   ) == 0 {
                    _currentBlock = 11;
                    break;
                }
            }
            nextpos = prevpos ^ get4(pos);
            prevpos = pos;
            pos = nextpos;
            if {
                   loopvar = loopvar.wrapping_add(1u32);
                   loopvar
               } > 100u32 {
                _currentBlock = 10;
                break;
            }
        }
        (if _currentBlock == 4 {
             0i32 as (*mut u8)
         } else if _currentBlock == 10 {
             0i32 as (*mut u8)
         } else {
             tai_unpack(
                 x.offset(pos as (isize)).offset(12isize) as (*const u8),
                 &mut expire as (*mut tai)
             );
             tai_now(&mut now as (*mut tai));
             (if (*(&mut expire as (*mut tai))).x < (*(&mut now as (*mut tai))).x {
                  0i32 as (*mut u8)
              } else {
                  tai_sub(
                      &mut expire as (*mut tai),
                      &mut expire as (*mut tai) as (*const tai),
                      &mut now as (*mut tai) as (*const tai)
                  );
                  d = (*(&mut expire as (*mut tai))).x as (f64);
                  if d > 604800i32 as (f64) {
                      d = 604800i32 as (f64);
                  }
                  *ttl = d as (u32);
                  u = get4(pos.wrapping_add(8u32));
                  if u > size.wrapping_sub(pos).wrapping_sub(20u32).wrapping_sub(
                             keylen
                         ) {
                      cache_impossible();
                  }
                  *datalen = u;
                  x.offset(pos as (isize)).offset(20isize).offset(keylen as (isize))
              })
         })
    }
}

unsafe extern fn set4(mut pos : u32, mut u : u32) {
    if pos > size.wrapping_sub(4u32) {
        cache_impossible();
    }
    uint32_pack(x.offset(pos as (isize)),u);
}

#[no_mangle]
pub unsafe extern fn cache_set(
    mut key : *const u8,
    mut keylen : u32,
    mut data : *const u8,
    mut datalen : u32,
    mut ttl : u32
) {
    let mut _currentBlock;
    let mut now : tai;
    let mut expire : tai;
    let mut entrylen : u32;
    let mut keyhash : u32;
    let mut pos : u32;
    if x.is_null() {
    } else if keylen > 1000u32 {
    } else if datalen > 1000000u32 {
    } else if ttl == 0 {
    } else {
        if ttl > 604800u32 {
            ttl = 604800u32;
        }
        entrylen = keylen.wrapping_add(datalen).wrapping_add(20u32);
        'loop7: loop {
            if !(writer.wrapping_add(entrylen) > oldest) {
                _currentBlock = 8;
                break;
            }
            if oldest == unused {
                if writer <= hsize {
                    _currentBlock = 18;
                    break;
                }
                unused = writer;
                oldest = hsize;
                writer = hsize;
            }
            pos = get4(oldest);
            set4(pos,get4(pos) ^ oldest);
            oldest = oldest.wrapping_add(
                         get4(oldest.wrapping_add(4u32)).wrapping_add(
                             get4(oldest.wrapping_add(8u32))
                         ).wrapping_add(
                             20u32
                         )
                     );
            if oldest > unused {
                cache_impossible();
            }
            if !(oldest == unused) {
                continue;
            }
            unused = size;
            oldest = size;
        }
        (if _currentBlock == 8 {
             keyhash = hash(key,keylen);
             tai_now(&mut now as (*mut tai));
             tai_uint(&mut expire as (*mut tai),ttl);
             tai_add(
                 &mut expire as (*mut tai),
                 &mut expire as (*mut tai) as (*const tai),
                 &mut now as (*mut tai) as (*const tai)
             );
             pos = get4(keyhash);
             if pos != 0 {
                 set4(pos,get4(pos) ^ keyhash ^ writer);
             }
             set4(writer,pos ^ keyhash);
             set4(writer.wrapping_add(4u32),keylen);
             set4(writer.wrapping_add(8u32),datalen);
             tai_pack(
                 x.offset(writer as (isize)).offset(12isize),
                 &mut expire as (*mut tai) as (*const tai)
             );
             byte_copy(
                 x.offset(writer as (isize)).offset(20isize),
                 keylen,
                 key as (*mut u8)
             );
             byte_copy(
                 x.offset(writer as (isize)).offset(20isize).offset(
                     keylen as (isize)
                 ),
                 datalen,
                 data as (*mut u8)
             );
             set4(keyhash,writer);
             writer = writer.wrapping_add(entrylen);
             cache_motion = cache_motion.wrapping_add(entrylen as (usize));
         })
    }
}

#[no_mangle]
pub unsafe extern fn cache_init(mut cachesize : u32) -> i32 {
    if !x.is_null() {
        alloc_free(x);
        x = 0i32 as (*mut u8);
    }
    if cachesize > 1000000000u32 {
        cachesize = 1000000000u32;
    }
    if cachesize < 100u32 {
        cachesize = 100u32;
    }
    size = cachesize;
    hsize = 4u32;
    'loop7: loop {
        if !(hsize <= size >> 5i32) {
            break;
        }
        hsize = hsize << 1i32;
    }
    x = alloc(size);
    if x.is_null() {
        0i32
    } else {
        byte_zero(x,size);
        writer = hsize;
        oldest = size;
        unused = size;
        1i32
    }
}
