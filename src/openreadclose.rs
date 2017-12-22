use errno::{errno, Errno};
use libc;
use open;
use stralloc::StrAlloc;

extern "C" {
    fn readclose(arg1: i32, arg2: *mut StrAlloc, arg3: u32) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn openreadclose(
    mut filename: *const u8,
    mut sa: *mut StrAlloc,
    mut bufsize: u32,
) -> i32 {
    let mut fd: i32;
    fd = open::read(filename);
    if fd == -1i32 {
        (if errno() == Errno(libc::ENOENT) { 0i32 } else { -1i32 })
    } else if readclose(fd, sa, bufsize) == -1i32 {
        -1i32
    } else {
        1i32
    }
}
