use errno::{errno, Errno};
use libc;
use stralloc::StrAlloc;

extern "C" {
    fn close(arg1: i32) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn readclose_append(
    mut fd: i32,
    mut sa: *mut StrAlloc,
    mut bufsize: u32,
) -> i32 {
    let mut _currentBlock;
    let mut r: i32;
    'loop1: loop {
        if StrAlloc::readyplus(sa, bufsize) == 0 {
            _currentBlock = 7;
            break;
        }
        r = libc::read(
            fd,
            (*sa).s.offset((*sa).len as (isize)) as (*mut libc::c_void),
            bufsize as (usize),
        ) as (i32);
        if r == -1i32 {
            if errno() == Errno(libc::EINTR) {
                continue;
            }
        }
        if r <= 0i32 {
            _currentBlock = 6;
            break;
        }
        (*sa).len = (*sa).len.wrapping_add(r as (u32));
    }
    if _currentBlock == 6 {
        close(fd);
        r
    } else {
        close(fd);
        -1i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn readclose(mut fd: i32, mut sa: *mut StrAlloc, mut bufsize: u32) -> i32 {
    if StrAlloc::copys(sa, (*b"\0").as_ptr()) == 0 {
        close(fd);
        -1i32
    } else {
        readclose_append(fd, sa, bufsize)
    }
}
