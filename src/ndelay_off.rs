extern "C" {
    fn fcntl(arg1: i32, arg2: i32, ...) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn ndelay_off(mut fd: i32) -> i32 {
    fcntl(fd, 4i32, fcntl(fd, 3i32, 0i32) & !0x4i32)
}
