extern "C" {
    fn listen(arg1: i32, arg2: i32) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn socket_listen(mut s: i32, mut backlog: i32) -> i32 {
    listen(s, backlog)
}
