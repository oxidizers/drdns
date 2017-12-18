extern "C" {
    fn open(arg1: *const u8, arg2: i32, ...) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn open_read(mut fn_: *const u8) -> i32 {
    open(fn_, 0x0i32 | 0x4i32)
}
