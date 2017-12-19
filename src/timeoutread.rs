use errno::{self, Errno};

extern "C" {
    fn iopause(arg1: *mut pollfd, arg2: u32, arg3: *mut taia, arg4: *mut taia);
    fn read(arg1: i32, arg2: *mut ::std::os::raw::c_void, arg3: usize) -> isize;
    fn taia_add(arg1: *mut taia, arg2: *const taia, arg3: *const taia);
    fn taia_less(arg1: *const taia, arg2: *const taia) -> i32;
    fn taia_now(arg1: *mut taia);
    fn taia_uint(arg1: *mut taia, arg2: u32);
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
    pub sec: tai,
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
pub unsafe extern "C" fn timeoutread(
    mut t: i32,
    mut fd: i32,
    mut buf: *mut u8,
    mut len: i32,
) -> i32 {
    let mut _currentBlock;
    let mut now: taia;
    let mut deadline: taia;
    let mut x: pollfd;
    taia_now(&mut now as (*mut taia));
    taia_uint(&mut deadline as (*mut taia), t as (u32));
    taia_add(
        &mut deadline as (*mut taia),
        &mut now as (*mut taia) as (*const taia),
        &mut deadline as (*mut taia) as (*const taia),
    );
    x.fd = fd;
    x.events = 0x1i16;
    'loop1: loop {
        taia_now(&mut now as (*mut taia));
        iopause(
            &mut x as (*mut pollfd),
            1u32,
            &mut deadline as (*mut taia),
            &mut now as (*mut taia),
        );
        if x.revents != 0 {
            _currentBlock = 4;
            break;
        }
        if taia_less(
            &mut deadline as (*mut taia) as (*const taia),
            &mut now as (*mut taia) as (*const taia),
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
        read(fd, buf as (*mut ::std::os::raw::c_void), len as (usize)) as (i32)
    }
}
