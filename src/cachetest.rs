use libc;

extern "C" {
    static mut buffer_1: *mut buffer;
    fn buffer_flush(arg1: *mut buffer) -> i32;
    fn buffer_put(arg1: *mut buffer, arg2: *const u8, arg3: u32) -> i32;
    fn buffer_puts(arg1: *mut buffer, arg2: *const u8) -> i32;
    fn cache_get(arg1: *const u8, arg2: u32, arg3: *mut u32, arg4: *mut u32) -> *mut u8;
    fn cache_init(arg1: u32) -> i32;
    fn cache_set(arg1: *const u8, arg2: u32, arg3: *const u8, arg4: u32, arg5: u32);
    fn str_chr(arg1: *const u8, arg2: i32) -> u32;
}

fn main() {
    use std::os::unix::ffi::OsStringExt;
    let mut argv_storage = ::std::env::args_os()
        .map(|str| {
            let mut vec = str.into_vec();
            vec.push(b'\0');
            vec
        })
        .collect::<Vec<_>>();
    let mut argv = argv_storage
        .iter_mut()
        .map(|vec| vec.as_mut_ptr())
        .chain(Some(::std::ptr::null_mut()))
        .collect::<Vec<_>>();
    let ret = unsafe { _c_main(argv_storage.len() as (i32), argv.as_mut_ptr()) };
    ::std::process::exit(ret);
}

#[derive(Copy)]
#[repr(C)]
pub struct buffer {
    pub x: *mut u8,
    pub p: u32,
    pub n: u32,
    pub fd: i32,
    pub op: unsafe extern "C" fn() -> i32,
}

impl Clone for buffer {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn _c_main(mut argc: i32, mut argv: *mut *mut u8) -> i32 {
    let mut i: i32;
    let mut x: *mut u8;
    let mut y: *mut u8;
    let mut u: u32;
    let mut ttl: u32;
    if cache_init(200u32) == 0 {
        libc::_exit(111i32);
    }
    if !(*argv).is_null() {
        argv = argv.offset(1isize);
    }
    'loop4: loop {
        if {
            x = *{
                let _old = argv;
                argv = argv.offset(1isize);
                _old
            };
            x
        }.is_null()
        {
            break;
        }
        i = str_chr(x as (*const u8), b':' as (i32)) as (i32);
        if *x.offset(i as (isize)) != 0 {
            cache_set(
                x as (*const u8),
                i as (u32),
                x.offset(i as (isize)).offset(1isize) as (*const u8),
                libc::strlen(x as (*const u8))
                    .wrapping_sub(i as (u32))
                    .wrapping_sub(1u32),
                86400u32,
            );
        } else {
            y = cache_get(
                x as (*const u8),
                i as (u32),
                &mut u as (*mut u32),
                &mut ttl as (*mut u32),
            );
            if !y.is_null() {
                buffer_put(buffer_1, y as (*const u8), u);
            }
            buffer_puts(buffer_1, (*b"\n\0").as_ptr());
        }
    }
    buffer_flush(buffer_1);
    libc::_exit(0i32);
}
