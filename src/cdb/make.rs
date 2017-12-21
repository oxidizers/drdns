use alloc;
use buffer::{self, Buffer};
use errno::{self, Errno};
use libc;
use uint32;

extern "C" {
    fn cdb_hash(arg1: *const u8, arg2: u32) -> u32;
    fn seek_set(arg1: i32, arg2: usize) -> i32;
}

#[derive(Copy)]
#[repr(C)]
pub struct cdb_hp {
    pub h: u32,
    pub p: u32,
}

impl Clone for cdb_hp {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct cdb_hplist {
    pub hp: [cdb_hp; 1000],
    pub next: *mut cdb_hplist,
    pub num: i32,
}

impl Clone for cdb_hplist {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct cdb_make {
    pub bspace: [u8; 8192],
    pub final_: [u8; 2048],
    pub count: [u32; 256],
    pub start: [u32; 256],
    pub head: *mut cdb_hplist,
    pub split: *mut cdb_hp,
    pub hash: *mut cdb_hp,
    pub numentries: u32,
    pub b: Buffer,
    pub pos: u32,
    pub fd: i32,
}

impl Clone for cdb_make {
    fn clone(&self) -> Self {
        *self
    }
}

pub unsafe extern "C" fn cdb_make_start(c: *mut cdb_make, fd: i32) -> i32 {
    (*c).head = 0i32 as (*mut cdb_hplist);
    (*c).split = 0i32 as (*mut cdb_hp);
    (*c).hash = 0i32 as (*mut cdb_hp);
    (*c).numentries = 0u32;
    (*c).fd = fd;
    (*c).pos = ::std::mem::size_of::<[u8; 2048]>() as (u32);
    Buffer::init(
        &mut (*c).b as (*mut Buffer),
        buffer::unixwrite as buffer::Op,
        fd,
        (*c).bspace.as_mut_ptr(),
        ::std::mem::size_of::<[u8; 8192]>() as (u32),
    );
    seek_set(fd, (*c).pos as (usize))
}

unsafe extern "C" fn posplus(c: *mut cdb_make, len: u32) -> i32 {
    let newpos: u32 = (*c).pos.wrapping_add(len);
    if newpos < len {
        errno::set_errno(Errno(libc::ENOMEM));
        -1i32
    } else {
        (*c).pos = newpos;
        0i32
    }
}

pub unsafe extern "C" fn cdb_make_addend(
    c: *mut cdb_make,
    keylen: u32,
    datalen: u32,
    h: u32,
) -> i32 {
    let mut head: *mut cdb_hplist;
    head = (*c).head;
    if head.is_null() || (*head).num >= 1000i32 {
        head = alloc::alloc(::std::mem::size_of::<cdb_hplist>() as (u32)) as (*mut cdb_hplist);
        if head.is_null() {
            return -1i32;
        } else {
            (*head).num = 0i32;
            (*head).next = (*c).head as (*mut cdb_hplist);
            (*c).head = head;
        }
    }
    (*head).hp[(*head).num as (usize)].h = h;
    (*head).hp[(*head).num as (usize)].p = (*c).pos;
    (*head).num = (*head).num + 1;
    (*c).numentries = (*c).numentries.wrapping_add(1u32);
    if posplus(c, 8u32) == -1i32 {
        -1i32
    } else if posplus(c, keylen) == -1i32 {
        -1i32
    } else if posplus(c, datalen) == -1i32 {
        -1i32
    } else {
        0i32
    }
}

pub unsafe extern "C" fn cdb_make_addbegin(
    c: *mut cdb_make,
    keylen: u32,
    datalen: u32,
) -> i32 {
    let mut buf = [0u8; 8];
    uint32::pack(buf.as_mut_ptr(), keylen);
    uint32::pack(buf.as_mut_ptr().offset(4isize), datalen);
    Buffer::putalign(&mut (*c).b as (*mut Buffer), buf.as_ptr(), 8u32)
}

pub unsafe extern "C" fn cdb_make_add(
    c: *mut cdb_make,
    key: *const u8,
    keylen: u32,
    data: *const u8,
    datalen: u32,
) -> i32 {
    if cdb_make_addbegin(c, keylen, datalen) == -1i32 {
        -1i32
    } else if Buffer::putalign(&mut (*c).b as (*mut Buffer), key, keylen) == -1i32 {
        -1i32
    } else if Buffer::putalign(&mut (*c).b as (*mut Buffer), data, datalen) == -1i32 {
        -1i32
    } else {
        cdb_make_addend(c, keylen, datalen, cdb_hash(key, keylen))
    }
}

pub unsafe extern "C" fn cdb_make_finish(c: *mut cdb_make) -> i32 {
    let current_block;
    let mut buf = [0u8; 8];
    let mut i: i32;
    let mut len: u32;
    let mut u: u32;
    let mut memsize: u32;
    let mut count: u32;
    let mut where_: u32;
    let mut x: *mut cdb_hplist;
    let mut hp: *mut cdb_hp;
    i = 0i32;
    'loop1: loop {
        if !(i < 256i32) {
            break;
        }
        (*c).count[i as (usize)] = 0u32;
        i = i + 1;
    }
    x = (*c).head;
    'loop3: loop {
        if x.is_null() {
            break;
        }
        i = (*x).num;
        'loop51: loop {
            if {
                let _old = i;
                i = i - 1;
                _old
            } == 0
            {
                break;
            }
            let _rhs = 1;
            let _lhs = &mut (*c).count[(255u32 & (*x).hp[i as (usize)].h) as (usize)];
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
        x = (*x).next as (*mut cdb_hplist);
    }
    memsize = 1u32;
    i = 0i32;
    'loop5: loop {
        if !(i < 256i32) {
            break;
        }
        u = (*c).count[i as (usize)].wrapping_mul(2u32);
        if u > memsize {
            memsize = u;
        }
        i = i + 1;
    }
    memsize = memsize.wrapping_add((*c).numentries);
    u = 0u32.wrapping_sub(1u32);
    u = (u as (usize)).wrapping_div(::std::mem::size_of::<cdb_hp>()) as (u32);
    if memsize > u {
        errno::set_errno(Errno(libc::ENOMEM));
        -1i32
    } else {
        (*c).split = alloc::alloc((memsize as (usize)).wrapping_mul(
            ::std::mem::size_of::<cdb_hp>(),
        ) as (u32)) as (*mut cdb_hp);
        (if (*c).split.is_null() {
             -1i32
         } else {
             (*c).hash = (*c).split.offset((*c).numentries as (isize));
             u = 0u32;
             i = 0i32;
             'loop9: loop {
                 if !(i < 256i32) {
                     break;
                 }
                 u = u.wrapping_add((*c).count[i as (usize)]);
                 (*c).start[i as (usize)] = u;
                 i = i + 1;
             }
             x = (*c).head;
             'loop11: loop {
                 if x.is_null() {
                     break;
                 }
                 i = (*x).num;
                 'loop40: loop {
                     if {
                         let _old = i;
                         i = i - 1;
                         _old
                     } == 0
                    {
                         break;
                     }
                     *(*c).split.offset({
                        let _rhs = 1;
                        let _lhs = &mut (*c).start[(255u32 & (*x).hp[i as (usize)].h) as (usize)];
                        *_lhs = (*_lhs).wrapping_sub(_rhs as (u32));
                        *_lhs
                    } as (isize)) = (*x).hp[i as (usize)];
                 }
                 x = (*x).next as (*mut cdb_hplist);
             }
             i = 0i32;
             'loop13: loop {
                 if !(i < 256i32) {
                     current_block = 14;
                     break;
                 }
                 count = (*c).count[i as (usize)];
                 len = count.wrapping_add(count);
                 uint32::pack(
                    (*c).final_.as_mut_ptr().offset((8i32 * i) as (isize)),
                    (*c).pos,
                );
                 uint32::pack(
                    (*c)
                        .final_
                        .as_mut_ptr()
                        .offset((8i32 * i) as (isize))
                        .offset(4isize),
                    len,
                );
                 u = 0u32;
                 'loop20: loop {
                     if !(u < len) {
                         break;
                     }
                     (*(*c).hash.offset(u as (isize))).h = {
                         let _rhs = 0i32;
                         let _lhs = &mut (*(*c).hash.offset(u as (isize))).p;
                         *_lhs = _rhs as (u32);
                         *_lhs
                     };
                     u = u.wrapping_add(1u32);
                 }
                 hp = (*c).split.offset((*c).start[i as (usize)] as (isize));
                 u = 0u32;
                 'loop22: loop {
                     if !(u < count) {
                         break;
                     }
                     where_ = ((*hp).h >> 8i32).wrapping_rem(len);
                     'loop32: loop {
                         if (*(*c).hash.offset(where_ as (isize))).p == 0 {
                             break;
                         }
                         if !({
                                  where_ = where_.wrapping_add(1u32);
                                  where_
                              } == len)
                        {
                             continue;
                         }
                         where_ = 0u32;
                     }
                     *(*c).hash.offset(where_ as (isize)) = *{
                         let _old = hp;
                         hp = hp.offset(1isize);
                         _old
                     };
                     u = u.wrapping_add(1u32);
                 }
                 u = 0u32;
                 'loop24: loop {
                     if !(u < len) {
                         break;
                     }
                     uint32::pack(buf.as_mut_ptr(), (*(*c).hash.offset(u as (isize))).h);
                     uint32::pack(
                        buf.as_mut_ptr().offset(4isize),
                        (*(*c).hash.offset(u as (isize))).p,
                    );
                     if Buffer::putalign(
                        &mut (*c).b as (*mut Buffer),
                        buf.as_mut_ptr() as (*const u8),
                        8u32,
                    ) == -1i32
                    {
                         current_block = 30;
                         break 'loop13;
                     }
                     if posplus(c, 8u32) == -1i32 {
                         current_block = 29;
                         break 'loop13;
                     }
                     u = u.wrapping_add(1u32);
                 }
                 i = i + 1;
             }
             (if current_block == 14 {
                  (if Buffer::flush(&mut (*c).b as (*mut Buffer)) == -1i32 {
                       -1i32
                   } else if seek_set((*c).fd, 0usize) == -1i32 {
                       -1i32
                   } else {
                       Buffer::putflush(
                        &mut (*c).b as (*mut Buffer),
                        (*c).final_.as_mut_ptr() as (*const u8),
                        ::std::mem::size_of::<[u8; 2048]>() as (u32),
                    )
                   })
              } else if current_block == 29 {
                  -1i32
              } else {
                  -1i32
              })
         })
    }
}
