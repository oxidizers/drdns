use errno::{self, Errno};
use libc;

extern "C" {
    fn iopause(arg1: *mut pollfd, arg2: u32, arg3: *mut TaiA, arg4: *mut TaiA);
    fn taia_add(arg1: *mut TaiA, arg2: *const TaiA, arg3: *const TaiA);
    fn taia_less(arg1: *const TaiA, arg2: *const TaiA) -> i32;
    fn taia_now(arg1: *mut TaiA);
    fn taia_uint(arg1: *mut TaiA, arg2: u32);
}

#[derive(Copy)]
#[repr(C)]
pub struct tai {
    pub x: usize,
}

impl Clone for tai {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct taia {
    pub sec: Tai,
    pub nano: usize,
    pub atto: usize,
}

impl Clone for taia {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct pollfd {
    pub fd: i32,
    pub events: i16,
    pub revents: i16,
}

impl Clone for pollfd {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn timeoutwrite(
    mut t: i32,
    mut fd: i32,
    mut buf: *mut u8,
    mut len: i32,
) -> i32 {
    let mut _currentBlock;
    let mut now: TaiA;
    let mut deadline: TaiA;
    let mut x: pollfd;
    taia_now(&mut now as (*mut TaiA));
    taia_uint(&mut deadline as (*mut TaiA), t as (u32));
    taia_add(
        &mut deadline as (*mut TaiA),
        &mut now as (*mut TaiA) as (*const TaiA),
        &mut deadline as (*mut TaiA) as (*const TaiA),
    );
    x.fd = fd;
    x.events = 0x4i16;
    'loop1: loop {
        taia_now(&mut now as (*mut TaiA));
        iopause(
            &mut x as (*mut pollfd),
            1u32,
            &mut deadline as (*mut TaiA),
            &mut now as (*mut TaiA),
        );
        if x.revents != 0 {
            _currentBlock = 4;
            break;
        }
        if taia_less(
            &mut deadline as (*mut TaiA) as (*const TaiA),
            &mut now as (*mut TaiA) as (*const TaiA),
        ) != 0
        {
            _currentBlock = 3;
            break;
        }
    }
    if _currentBlock == 3 {
        errno::set_errno(Errno(libc::ETIMEDOUT));
        -1i32
    } else {
        libc::write(fd, buf as (*const libc::c_void), len as (usize)) as (i32)
    }
}
