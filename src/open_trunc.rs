extern "C" {
    fn open(arg1: *const u8, arg2: i32, ...) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn open_trunc(mut fn_: *const u8) -> i32 {
    open(fn_, 0x1i32 | 0x4i32 | 0x400i32 | 0x200i32, 0o644i32)
}
