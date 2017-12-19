use byte;
use errno::{self, Errno};
use libc;

extern "C" {
    fn cdb_hash(arg1: *const u8, arg2: u32) -> u32;
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
    fn read(arg1: i32, arg2: *mut ::std::os::raw::c_void, arg3: usize) -> isize;
    fn seek_set(arg1: i32, arg2: usize) -> i32;
    fn uint32_unpack(arg1: *const u8, arg2: *mut u32);
}

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

#[no_mangle]
pub unsafe extern "C" fn cdb_free(mut c: *mut cdb) {
    if !(*c).map.is_null() {
        munmap(
            (*c).map as (*mut ::std::os::raw::c_void),
            (*c).size as (usize),
        );
        (*c).map = 0i32 as (*mut u8);
    }
}

#[no_mangle]
pub unsafe extern "C" fn cdb_findstart(mut c: *mut cdb) {
    (*c).loopvar = 0u32;
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

#[no_mangle]
pub unsafe extern "C" fn cdb_init(mut c: *mut cdb, mut fd: i32) {
    let mut st: stat;
    let mut x: *mut u8;
    cdb_free(c);
    cdb_findstart(c);
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

#[no_mangle]
pub unsafe extern "C" fn cdb_read(
    mut c: *mut cdb,
    mut buf: *mut u8,
    mut len: u32,
    mut pos: u32,
) -> i32 {
    let mut _currentBlock;
    if !(*c).map.is_null() {
        if pos > (*c).size || (*c).size.wrapping_sub(pos) < len {
            _currentBlock = 14;
        } else {
            byte::copy(buf, len, (*c).map.offset(pos as (isize)));
            _currentBlock = 13;
        }
    } else if seek_set((*c).fd, pos as (usize)) == -1i32 {
        return -1i32;
    } else {
        'loop2: loop {
            if !(len > 0u32) {
                _currentBlock = 13;
                break;
            }
            let mut r: i32;
            'loop4: loop {
                r = read(
                    (*c).fd,
                    buf as (*mut ::std::os::raw::c_void),
                    len as (usize),
                ) as (i32);
                if !(r == -1i32 && (errno::errno() == Errno(libc::EINTR))) {
                    break;
                }
            }
            if r == -1i32 {
                _currentBlock = 9;
                break;
            }
            if r == 0i32 {
                _currentBlock = 14;
                break;
            }
            buf = buf.offset(r as (isize));
            len = len.wrapping_sub(r as (u32));
        }
        if _currentBlock == 13 {
        } else if _currentBlock == 14 {
        } else {
            return -1i32;
        }
    }
    if _currentBlock == 13 {
        0i32
    } else {
        errno::set_errno(Errno(libc::EPROTO));
        -1i32
    }
}

unsafe extern "C" fn match_(
    mut c: *mut cdb,
    mut key: *const u8,
    mut len: u32,
    mut pos: u32,
) -> i32 {
    let mut _currentBlock;
    let mut buf: [u8; 32];
    let mut n: i32;
    'loop1: loop {
        if !(len > 0u32) {
            _currentBlock = 2;
            break;
        }
        n = ::std::mem::size_of::<[u8; 32]>() as (i32);
        if n as (u32) > len {
            n = len as (i32);
        }
        if cdb_read(c, buf.as_mut_ptr(), n as (u32), pos) == -1i32 {
            _currentBlock = 9;
            break;
        }
        if byte::diff(buf.as_mut_ptr(), n as (u32), key as (*mut u8)) != 0 {
            _currentBlock = 8;
            break;
        }
        pos = pos.wrapping_add(n as (u32));
        key = key.offset(n as (isize));
        len = len.wrapping_sub(n as (u32));
    }
    if _currentBlock == 2 {
        1i32
    } else if _currentBlock == 8 {
        0i32
    } else {
        -1i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn cdb_findnext(mut c: *mut cdb, mut key: *const u8, mut len: u32) -> i32 {
    let mut _currentBlock;
    let mut buf: [u8; 8];
    let mut pos: u32;
    let mut u: u32;
    if (*c).loopvar == 0 {
        u = cdb_hash(key, len);
        if cdb_read(c, buf.as_mut_ptr(), 8u32, u << 3i32 & 2047u32) == -1i32 {
            return -1i32;
        } else {
            uint32_unpack(
                buf.as_mut_ptr().offset(4isize) as (*const u8),
                &mut (*c).hslots as (*mut u32),
            );
            if (*c).hslots == 0 {
                return 0i32;
            } else {
                uint32_unpack(
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
            _currentBlock = 5;
            break;
        }
        if cdb_read(c, buf.as_mut_ptr(), 8u32, (*c).kpos) == -1i32 {
            _currentBlock = 20;
            break;
        }
        uint32_unpack(
            buf.as_mut_ptr().offset(4isize) as (*const u8),
            &mut pos as (*mut u32),
        );
        if pos == 0 {
            _currentBlock = 19;
            break;
        }
        (*c).loopvar = (*c).loopvar.wrapping_add(1u32);
        (*c).kpos = (*c).kpos.wrapping_add(8u32);
        if (*c).kpos == (*c).hpos.wrapping_add((*c).hslots << 3i32) {
            (*c).kpos = (*c).hpos;
        }
        uint32_unpack(buf.as_mut_ptr() as (*const u8), &mut u as (*mut u32));
        if !(u == (*c).khash) {
            continue;
        }
        if cdb_read(c, buf.as_mut_ptr(), 8u32, pos) == -1i32 {
            _currentBlock = 18;
            break;
        }
        uint32_unpack(buf.as_mut_ptr() as (*const u8), &mut u as (*mut u32));
        if !(u == len) {
            continue;
        }
        let switch1 = match_(c, key, len, pos.wrapping_add(8u32));
        if switch1 == 1i32 {
            _currentBlock = 17;
            break;
        }
        if switch1 == -1i32 {
            _currentBlock = 16;
            break;
        }
    }
    if _currentBlock == 5 {
        0i32
    } else if _currentBlock == 16 {
        -1i32
    } else if _currentBlock == 17 {
        uint32_unpack(
            buf.as_mut_ptr().offset(4isize) as (*const u8),
            &mut (*c).dlen as (*mut u32),
        );
        (*c).dpos = pos.wrapping_add(8u32).wrapping_add(len);
        1i32
    } else if _currentBlock == 18 {
        -1i32
    } else if _currentBlock == 19 {
        0i32
    } else {
        -1i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn cdb_find(mut c: *mut cdb, mut key: *const u8, mut len: u32) -> i32 {
    cdb_findstart(c);
    cdb_findnext(c, key, len)
}
