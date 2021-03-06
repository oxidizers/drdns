//! `cdb/cdb.rs`: C DataBase (CDB) file reader

use byte;
use errno::{self, Errno};
use libc;
use uint32;
use super::hash as cdb_hash;

/// C DataBase file reader
#[derive(Copy)]
#[repr(C)]
pub struct Cdb {
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

impl Clone for Cdb {
    fn clone(&self) -> Self {
        *self
    }
}

impl Cdb {
    pub unsafe fn free(c: *mut Cdb) {
        if !(*c).map.is_null() {
            libc::munmap((*c).map as *mut libc::c_void, (*c).size as usize);
            (*c).map = 0i32 as *mut u8;
        }
    }

    pub unsafe fn findstart(c: *mut Cdb) {
        (*c).loopvar = 0u32;
    }

    pub unsafe fn init(c: *mut Cdb, fd: i32) {
        let mut st: libc::stat = ::std::mem::zeroed();

        let x: *mut u8;
        Cdb::free(c);
        Cdb::findstart(c);
        (*c).fd = fd;
        if libc::fstat(fd, &mut st as (*mut libc::stat)) == 0i32 {
            if st.st_size as (usize) <= 0xffffffffusize {
                x = libc::mmap(
                    0 as *mut libc::c_void,
                    st.st_size as (usize),
                    0x1,
                    0x1,
                    fd,
                    0,
                ) as (*mut u8);
                if !x.offset(1isize).is_null() {
                    (*c).size = st.st_size as (u32);
                    (*c).map = x;
                }
            }
        }
    }

    pub unsafe fn read(c: *mut Cdb, mut buf: *mut u8, mut len: u32, pos: u32) -> i32 {
        let current_block;
        if !(*c).map.is_null() {
            if pos > (*c).size || (*c).size.wrapping_sub(pos) < len {
                current_block = 14;
            } else {
                byte::copy(buf, len, (*c).map.offset(pos as (isize)));
                current_block = 13;
            }
        } else if libc::lseek((*c).fd, pos as i64, 0) == -1 {
            return -1i32;
        } else {
            'loop2: loop {
                if !(len > 0u32) {
                    current_block = 13;
                    break;
                }
                let mut r: i32;
                'loop4: loop {
                    r = libc::read((*c).fd, buf as *mut libc::c_void, len as (usize)) as (i32);
                    if !(r == -1i32 && (errno::errno() == Errno(libc::EINTR))) {
                        break;
                    }
                }
                if r == -1i32 {
                    current_block = 9;
                    break;
                }
                if r == 0i32 {
                    current_block = 14;
                    break;
                }
                buf = buf.offset(r as (isize));
                len = len.wrapping_sub(r as (u32));
            }
            if current_block == 13 {
            } else if current_block == 14 {
            } else {
                return -1i32;
            }
        }
        if current_block == 13 {
            0i32
        } else {
            errno::set_errno(Errno(libc::EPROTO));
            -1i32
        }
    }

    pub unsafe fn findnext(c: *mut Cdb, key: *const u8, len: u32) -> i32 {
        let current_block;
        let mut buf = [0u8; 8];
        let mut pos = 0u32;
        let mut u = 0u32;
        if (*c).loopvar == 0 {
            u = cdb_hash(key, len);
            if Cdb::read(c, buf.as_mut_ptr(), 8u32, u << 3i32 & 2047u32) == -1i32 {
                return -1i32;
            } else {
                uint32::unpack(
                    buf.as_mut_ptr().offset(4isize) as (*const u8),
                    &mut (*c).hslots as (*mut u32),
                );
                if (*c).hslots == 0 {
                    return 0i32;
                } else {
                    uint32::unpack(
                        buf.as_mut_ptr() as (*const u8),
                        &mut (*c).hpos as (*mut u32),
                    );
                    (*c).khash = u;
                    u = u >> 8i32;
                    u = u.wrapping_rem((*c).hslots);
                    u = u << 3i32;
                    (*c).kpos = (*c).hpos.wrapping_add(u);
                }
            }
        }
        'loop4: loop {
            if !((*c).loopvar < (*c).hslots) {
                current_block = 5;
                break;
            }
            if Cdb::read(c, buf.as_mut_ptr(), 8u32, (*c).kpos) == -1i32 {
                current_block = 20;
                break;
            }
            uint32::unpack(
                buf.as_mut_ptr().offset(4isize) as (*const u8),
                &mut pos as (*mut u32),
            );
            if pos == 0 {
                current_block = 19;
                break;
            }
            (*c).loopvar = (*c).loopvar.wrapping_add(1u32);
            (*c).kpos = (*c).kpos.wrapping_add(8u32);
            if (*c).kpos == (*c).hpos.wrapping_add((*c).hslots << 3i32) {
                (*c).kpos = (*c).hpos;
            }
            uint32::unpack(buf.as_mut_ptr() as (*const u8), &mut u as (*mut u32));
            if !(u == (*c).khash) {
                continue;
            }
            if Cdb::read(c, buf.as_mut_ptr(), 8u32, pos) == -1i32 {
                current_block = 18;
                break;
            }
            uint32::unpack(buf.as_mut_ptr() as (*const u8), &mut u as (*mut u32));
            if !(u == len) {
                continue;
            }
            let switch1 = Self::switch(c, key, len, pos.wrapping_add(8u32));
            if switch1 == 1i32 {
                current_block = 17;
                break;
            }
            if switch1 == -1i32 {
                current_block = 16;
                break;
            }
        }
        if current_block == 5 {
            0i32
        } else if current_block == 16 {
            -1i32
        } else if current_block == 17 {
            uint32::unpack(
                buf.as_mut_ptr().offset(4isize) as (*const u8),
                &mut (*c).dlen as (*mut u32),
            );
            (*c).dpos = pos.wrapping_add(8u32).wrapping_add(len);
            1i32
        } else if current_block == 18 {
            -1i32
        } else if current_block == 19 {
            0i32
        } else {
            -1i32
        }
    }

    pub unsafe fn find(c: *mut Cdb, key: *const u8, len: u32) -> i32 {
        Cdb::findstart(c);
        Cdb::findnext(c, key, len)
    }

    unsafe fn switch(c: *mut Cdb, mut key: *const u8, mut len: u32, mut pos: u32) -> i32 {
        let current_block;
        let mut buf = [0u8; 32];
        let mut n: i32;
        'loop1: loop {
            if !(len > 0u32) {
                current_block = 2;
                break;
            }
            n = ::std::mem::size_of::<[u8; 32]>() as (i32);
            if n as (u32) > len {
                n = len as (i32);
            }
            if Cdb::read(c, buf.as_mut_ptr(), n as (u32), pos) == -1i32 {
                current_block = 9;
                break;
            }
            if byte::diff(buf.as_mut_ptr(), n as (u32), key as (*mut u8)) != 0 {
                current_block = 8;
                break;
            }
            pos = pos.wrapping_add(n as (u32));
            key = key.offset(n as (isize));
            len = len.wrapping_sub(n as (u32));
        }
        if current_block == 2 {
            1i32
        } else if current_block == 8 {
            0i32
        } else {
            -1i32
        }
    }
}
