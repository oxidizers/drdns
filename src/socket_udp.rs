extern "C" {
    fn close(arg1: i32) -> i32;
    fn ndelay_on(arg1: i32) -> i32;
    fn socket(arg1: i32, arg2: i32, arg3: i32) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn socket_udp() -> i32 {
    let mut s: i32;
    s = socket(2i32, 2i32, 0i32);
    if s == -1i32 {
        -1i32
    } else if ndelay_on(s) == -1i32 {
        close(s);
        -1i32
    } else {
        s
    }
}
