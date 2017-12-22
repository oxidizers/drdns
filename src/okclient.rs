use ip4;
use string;

extern "C" {
    fn stat(arg1: *const u8, arg2: *mut stat) -> i32;
}

static mut fn_: [u8; 23] = [0u8; 23];

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
pub unsafe extern "C" fn okclient(mut ip: *mut u8) -> i32 {
    let mut _currentBlock;
    let mut st: stat;
    let mut i: i32;
    fn_[0usize] = b'i';
    fn_[1usize] = b'p';
    fn_[2usize] = b'/';
    fn_[3u32.wrapping_add(ip4::fmt(fn_.as_mut_ptr().offset(3isize), ip as (*const u8))) as
            (usize)] = 0u8;
    'loop1: loop {
        if stat(fn_.as_mut_ptr() as (*const u8), &mut st as (*mut stat)) == 0i32 {
            _currentBlock = 5;
            break;
        }
        i = string::rchr(fn_.as_mut_ptr() as (*const u8), b'.' as (i32)) as (i32);
        if fn_[i as (usize)] == 0 {
            _currentBlock = 4;
            break;
        }
        fn_[i as (usize)] = 0u8;
    }
    if _currentBlock == 4 { 0i32 } else { 1i32 }
}
