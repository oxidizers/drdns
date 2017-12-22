/* `ndelay.rs`: Enable or disable O_NDELAY (i.e. Nagle's algorithm) */

use libc;

pub unsafe fn off(fd: i32) -> i32 {
    libc::fcntl(fd, libc::F_SETFL, libc::fcntl(fd, libc::F_GETFL, 0) & !libc::O_NONBLOCK)
}

pub unsafe fn on(fd: i32) -> i32 {
    libc::fcntl(fd, libc::F_SETFL, libc::fcntl(fd, libc::F_GETFL, 0) | libc::O_NONBLOCK)
}
