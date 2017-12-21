//! `cdb/cdb.rs`: C DataBase (CDB) file reader

use byte;
use errno::{self, Errno};
use libc;
use uint32;
use super::hash as cdb_hash;

extern "C" {
    fn fstat(arg1: i32, arg2: *mut stat) -> i32;
    fn mmap(
        arg1: *mut ::std::os::raw::c_void,
        arg2: usize,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        arg6: isize,
    ) -> *mut ::std::os::raw::c_void;
    fn munmap(arg1: *mut ::std::os::raw::c_void, arg2: usize) -> i32;
}

/// C DataBase file reader
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

#[derive(Copy)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: isize,
    pub tv_nsec: isize,
}

impl Clone for timespec {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct stat {
    pub st_dev: i32,
    pub st_mode: u16,
    pub st_nlink: u16,
    pub st_ino: usize,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: i32,
    pub st_atimespec: timespec,
    pub st_mtimespec: timespec,
    pub st_ctimespec: timespec,
    pub st_birthtimespec: timespec,
    pub st_size: isize,
    pub st_blocks: isize,
    pub st_blksize: i32,
    pub st_flags: u32,
    pub st_gen: u32,
    pub st_lspare: i32,
    pub st_qspare: [isize; 2],
}

impl Clone for stat {
    fn clone(&self) -> Self {
        *self
    }
}

impl cdb {
    pub unsafe extern "C" fn cdb_free(c: *mut cdb) {
        if !(*c).map.is_null() {
            munmap(
                (*c).map as (*mut ::std::os::raw::c_void),
                (*c).size as (usize),
            );
            (*c).map = 0i32 as (*mut u8);
        }
    }

    pub unsafe extern "C" fn cdb_findstart(c: *mut cdb) {
        (*c).loopvar = 0u32;
    }

    pub unsafe extern "C" fn cdb_init(c: *mut cdb, fd: i32) {
        let mut st: stat = stat {
            st_dev: 0i32,
            st_mode: 0u16,
            st_nlink: 0u16,
            st_ino: 0usize,
            st_uid: 0u32,
            st_gid: 0u32,
            st_rdev: 0i32,
            st_atimespec: timespec {
                tv_sec: 0isize,
                tv_nsec: 0isize,
            },
            st_mtimespec: timespec {
                tv_sec: 0isize,
                tv_nsec: 0isize,
            },
            st_ctimespec: timespec {
                tv_sec: 0isize,
                tv_nsec: 0isize,
            },
            st_birthtimespec: timespec {
                tv_sec: 0isize,
                tv_nsec: 0isize,
            },
            st_size: 0isize,
            st_blocks: 0isize,
            st_blksize: 0i32,
            st_flags: 0u32,
            st_gen: 0u32,
            st_lspare: 0i32,
            st_qspare: [0isize, 0isize],
        };

        let x: *mut u8;
        Self::cdb_free(c);
        Self::cdb_findstart(c);
        (*c).fd = fd;
        if fstat(fd, &mut st as (*mut stat)) == 0i32 {
            if st.st_size as (usize) <= 0xffffffffusize {
                x = mmap(
                    0i32 as (*mut ::std::os::raw::c_void),
                    st.st_size as (usize),
                    0x1i32,
                    0x1i32,
                    fd,
                    0isize,
                ) as (*mut u8);
                if !x.offset(1isize).is_null() {
                    (*c).size = st.st_size as (u32);
                    (*c).map = x;
                }
            }
        }
    }

    pub unsafe extern "C" fn cdb_read(
        c: *mut cdb,
        mut buf: *mut u8,
        mut len: u32,
        pos: u32,
    ) -> i32 {
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
                    r = libc::read(
                        (*c).fd,
                        buf as *mut libc::c_void,
                        len as (usize),
                    ) as (i32);
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

    unsafe extern "C" fn match_(
        c: *mut cdb,
        mut key: *const u8,
        mut len: u32,
        mut pos: u32,
    ) -> i32 {
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
            if Self::cdb_read(c, buf.as_mut_ptr(), n as (u32), pos) == -1i32 {
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

    pub unsafe extern "C" fn cdb_findnext(c: *mut cdb, key: *const u8, len: u32) -> i32 {
        let current_block;
        let mut buf = [0u8; 8];
        let mut pos = 0u32;
        let mut u = 0u32;
        if (*c).loopvar == 0 {
            u = cdb_hash(key, len);
            if Self::cdb_read(c, buf.as_mut_ptr(), 8u32, u << 3i32 & 2047u32) == -1i32 {
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
            if Self::cdb_read(c, buf.as_mut_ptr(), 8u32, (*c).kpos) == -1i32 {
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
            if Self::cdb_read(c, buf.as_mut_ptr(), 8u32, pos) == -1i32 {
                current_block = 18;
                break;
            }
            uint32::unpack(buf.as_mut_ptr() as (*const u8), &mut u as (*mut u32));
            if !(u == len) {
                continue;
            }
            let switch1 = Self::match_(c, key, len, pos.wrapping_add(8u32));
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

    pub unsafe extern "C" fn cdb_find(c: *mut cdb, key: *const u8, len: u32) -> i32 {
        Self::cdb_findstart(c);
        Self::cdb_findnext(c, key, len)
    }
}
