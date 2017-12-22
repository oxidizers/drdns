use byte;
use errno::{self, Errno};
use libc;
use stralloc::StrAlloc;
use string;

extern "C" {
    fn chdir(arg1: *const u8) -> i32;
    fn close(arg1: i32) -> i32;
    fn closedir(arg1: *mut Struct1) -> i32;
    fn dns_domain_equal(arg1: *const u8, arg2: *const u8) -> i32;
    fn dns_domain_fromdot(arg1: *mut *mut u8, arg2: *const u8, arg3: u32) -> i32;
    fn dns_domain_length(arg1: *const u8) -> u32;
    fn fchdir(arg1: i32) -> i32;
    fn ip4_scan(arg1: *const u8, arg2: *mut u8) -> u32;
    fn open_read(arg1: *const u8) -> i32;
    fn opendir(arg1: *const u8) -> *mut Struct1;
    fn openreadclose(arg1: *const u8, arg2: *mut StrAlloc, arg3: u32) -> i32;
    fn readdir(arg1: *mut Struct1) -> *mut dirent;
}

enum _telldir {
}

static mut data: StrAlloc = StrAlloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

unsafe extern "C" fn roots_find(mut q: *mut u8) -> i32 {
    let mut _currentBlock;
    let mut i: i32;
    let mut j: i32;
    i = 0i32;
    'loop1: loop {
        if !(i as (u32) < data.len) {
            _currentBlock = 2;
            break;
        }
        j = dns_domain_length(data.s.offset(i as (isize)) as (*const u8)) as (i32);
        if dns_domain_equal(data.s.offset(i as (isize)) as (*const u8), q as (*const u8)) != 0 {
            _currentBlock = 5;
            break;
        }
        i = i + j;
        i = i + 64i32;
    }
    if _currentBlock == 2 { -1i32 } else { i + j }
}

#[no_mangle]
pub unsafe extern "C" fn roots(mut servers: *mut u8, mut q: *mut u8) -> i32 {
    let mut r: i32;
    r = roots_find(q);
    if r == -1i32 {
        0i32
    } else {
        byte::copy(servers, 64u32, data.s.offset(r as (isize)));
        1i32
    }
}

unsafe extern "C" fn roots_search(mut q: *mut u8) -> i32 {
    let mut _currentBlock;
    let mut r: i32;
    'loop1: loop {
        r = roots_find(q);
        if r >= 0i32 {
            _currentBlock = 5;
            break;
        }
        if *q == 0 {
            _currentBlock = 4;
            break;
        }
        q = q.offset(*q as (isize));
        q = q.offset(1isize);
    }
    if _currentBlock == 4 { -1i32 } else { r }
}

#[no_mangle]
pub unsafe extern "C" fn roots_same(mut q: *mut u8, mut q2: *mut u8) -> i32 {
    (roots_search(q) == roots_search(q2)) as (i32)
}

#[derive(Copy)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: isize,
    pub __opaque: [u8; 56],
}

impl Clone for _opaque_pthread_mutex_t {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
    pub __dd_fd: i32,
    pub __dd_loc: isize,
    pub __dd_size: isize,
    pub __dd_buf: *mut u8,
    pub __dd_len: i32,
    pub __dd_seek: isize,
    pub __padding: isize,
    pub __dd_flags: i32,
    pub __dd_lock: _opaque_pthread_mutex_t,
    pub __dd_td: *mut _telldir,
}

impl Clone for Struct1 {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct dirent {
    pub d_ino: usize,
    pub d_seekoff: usize,
    pub d_reclen: u16,
    pub d_namlen: u16,
    pub d_type: u8,
    pub d_name: [u8; 1024],
}

impl Clone for dirent {
    fn clone(&self) -> Self {
        *self
    }
}

unsafe extern "C" fn init2(mut dir: *mut Struct1) -> i32 {
    let mut _currentBlock;
    let mut d: *mut dirent;
    let mut fqdn: *const u8;
    static mut q: *mut u8 = 0 as (*mut u8);
    static mut text: StrAlloc = StrAlloc {
        s: 0 as (*mut u8),
        len: 0u32,
        a: 0u32,
    };
    let mut servers: [u8; 64];
    let mut serverslen: i32;
    let mut i: i32;
    let mut j: i32;
    'loop1: loop {
        errno::set_errno(Errno(0i32));
        d = readdir(dir);
        if d.is_null() {
            _currentBlock = 23;
            break;
        }
        if !((*d).d_name[0usize] as (i32) != b'.' as (i32)) {
            continue;
        }
        if openreadclose(
            (*d).d_name.as_mut_ptr() as (*const u8),
            &mut text as (*mut StrAlloc),
            32u32,
        ) != 1i32
        {
            _currentBlock = 22;
            break;
        }
        if StrAlloc::append(&mut text as (*mut StrAlloc), (*b"\n\0").as_ptr()) == 0 {
            _currentBlock = 21;
            break;
        }
        fqdn = (*d).d_name.as_mut_ptr() as (*const u8);
        if string::diff(fqdn, (*b"@\0").as_ptr()) == 0 {
            fqdn = (*b".\0").as_ptr();
        }
        if dns_domain_fromdot(&mut q as (*mut *mut u8), fqdn, libc::strlen(fqdn as *const i8) as u32) == 0 {
            _currentBlock = 20;
            break;
        }
        serverslen = 0i32;
        j = 0i32;
        i = 0i32;
        'loop9: loop {
            if !(i as (u32) < text.len) {
                break;
            }
            if *text.s.offset(i as (isize)) as (i32) == b'\n' as (i32) {
                if serverslen <= 60i32 {
                    if ip4_scan(
                        text.s.offset(j as (isize)) as (*const u8),
                        servers.as_mut_ptr().offset(serverslen as (isize)),
                    ) != 0
                    {
                        serverslen = serverslen + 4i32;
                    }
                }
                j = i + 1i32;
            }
            i = i + 1;
        }
        byte::zero(
            servers.as_mut_ptr().offset(serverslen as (isize)),
            (64i32 - serverslen) as (u32),
        );
        if StrAlloc::catb(
            &mut data as (*mut StrAlloc),
            q as (*const u8),
            dns_domain_length(q as (*const u8)),
        ) == 0
        {
            _currentBlock = 13;
            break;
        }
        if StrAlloc::catb(
            &mut data as (*mut StrAlloc),
            servers.as_mut_ptr() as (*const u8),
            64u32,
        ) == 0
        {
            _currentBlock = 12;
            break;
        }
    }
    if _currentBlock == 12 {
        0i32
    } else if _currentBlock == 13 {
        0i32
    } else if _currentBlock == 20 {
        0i32
    } else if _currentBlock == 21 {
        0i32
    } else if _currentBlock == 22 {
        0i32
    } else if errno::errno().0 != 0 {
        0i32
    } else {
        1i32
    }
}

unsafe extern "C" fn init1() -> i32 {
    let mut dir: *mut Struct1;
    let mut r: i32;
    if chdir((*b"servers\0").as_ptr()) == -1i32 {
        0i32
    } else {
        dir = opendir((*b".\0").as_ptr());
        (if dir.is_null() {
             0i32
         } else {
             r = init2(dir);
             closedir(dir);
             r
         })
    }
}

#[no_mangle]
pub unsafe extern "C" fn roots_init() -> i32 {
    let mut fddir: i32;
    let mut r: i32;
    if StrAlloc::copys(&mut data as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
        0i32
    } else {
        fddir = open_read((*b".\0").as_ptr());
        (if fddir == -1i32 {
             0i32
         } else {
             r = init1();
             if fchdir(fddir) == -1i32 {
                 r = 0i32;
             }
             close(fddir);
             r
         })
    }
}
