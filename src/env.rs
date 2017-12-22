use libc;
use string;

extern "C" {
    static mut environ: *mut *mut u8;
}

#[no_mangle]
pub unsafe extern "C" fn env_get(mut s: *const u8) -> *mut u8 {
    let mut _currentBlock;
    let mut i: i32;
    let mut len: u32;
    if s.is_null() {
        0i32 as (*mut u8)
    } else {
        len = libc::strlen(s as *const i8) as u32;
        i = 0i32;
        'loop2: loop {
            if (*environ.offset(i as (isize))).is_null() {
                _currentBlock = 3;
                break;
            }
            if string::start(*environ.offset(i as (isize)) as (*const u8), s) != 0 &&
                (*(*environ.offset(i as (isize))).offset(len as (isize)) as (i32) ==
                    b'=' as (i32))
            {
                _currentBlock = 6;
                break;
            }
            i = i + 1;
        }
        (if _currentBlock == 3 {
             0i32 as (*mut u8)
         } else {
             (*environ.offset(i as (isize)))
                 .offset(len as (isize))
                 .offset(1isize)
         })
    }
}
