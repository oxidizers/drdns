use byte;

extern "C" {
    static mut errno: i32;
    static mut error_nomem: i32;
    fn free(arg1: *mut ::std::os::raw::c_void);
    fn malloc(__size: usize) -> *mut ::std::os::raw::c_void;
}

#[no_mangle]
pub unsafe extern "C" fn alloc(mut n: u32) -> *mut u8 {
    let mut x: *mut u8;
    x = malloc(n as (usize)) as (*mut u8);
    if x.is_null() {
        errno = error_nomem;
    }
    x
}

#[no_mangle]
pub unsafe extern "C" fn alloc_re(mut x: *mut *mut u8, mut m: u32, mut n: u32) -> i32 {
    let mut y: *mut u8;
    y = alloc(n);
    if y.is_null() {
        0i32
    } else {
        byte::copy(y, m, *x);
        alloc_free(*x);
        *x = y;
        1i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn alloc_free(mut x: *mut u8) {
    free(x as (*mut ::std::os::raw::c_void));
}
