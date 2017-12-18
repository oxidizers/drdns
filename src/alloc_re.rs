extern "C" {
    fn alloc(n: u32) -> *mut u8;
    fn alloc_free(x: *mut u8);
    fn byte_copy(to: *mut u8, n: u32, from: *mut u8);
}

#[no_mangle]
pub unsafe extern "C" fn alloc_re(mut x: *mut *mut u8, mut m: u32, mut n: u32) -> i32 {
    let mut y: *mut u8;
    y = alloc(n);
    if y.is_null() {
        0i32
    } else {
        byte_copy(y, m, *x);
        alloc_free(*x);
        *x = y;
        1i32
    }
}
