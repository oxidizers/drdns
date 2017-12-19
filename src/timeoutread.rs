use errno::{self, Errno};
use libc;
use tai::Tai;
use taia::TaiA;

extern "C" {
    fn iopause(arg1: *mut pollfd, arg2: u32, arg3: *mut TaiA, arg4: *mut TaiA);
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
    let mut now: TaiA;
    let mut deadline: TaiA;
    let mut x: pollfd;
    TaiA::now(&mut now as (*mut TaiA));
    TaiA::uint(&mut deadline as (*mut TaiA), t as (u32));
    TaiA::add(
        &mut deadline as (*mut TaiA),
        &mut now as (*mut TaiA) as (*const TaiA),
        &mut deadline as (*mut TaiA) as (*const TaiA),
    );
    x.fd = fd;
    x.events = 0x1i16;
    'loop1: loop {
        TaiA::now(&mut now as (*mut TaiA));
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
        if TaiA::less(
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
        libc::read(fd, buf as (*mut libc::c_void), len as (usize)) as (i32)
    }
}
