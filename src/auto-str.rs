extern "C" {
    fn _exit(arg1: i32);
    fn buffer_flush(arg1: *mut buffer) -> i32;
    fn buffer_puts(arg1: *mut buffer, arg2: *const u8) -> i32;
    fn buffer_unixwrite(arg1: i32, arg2: *const u8, arg3: u32) -> i32;
}

#[no_mangle]
pub static mut bspace: [u8; 256] = [0u8; 256];

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
pub static mut b: buffer = buffer {
    x: bspace.as_mut_ptr(),
    p: 0u32,
    n: ::std::mem::size_of::<[u8; 256]>() as (u32),
    fd: 1i32,
    op: buffer_unixwrite as (unsafe extern "C" fn() -> i32),
};

#[no_mangle]
pub unsafe extern "C" fn puts(mut s: *const u8) {
    if buffer_puts(&mut b as (*mut buffer), s) == -1i32 {
        _exit(111i32);
    }
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

#[no_mangle]
pub unsafe extern "C" fn _c_main(mut argc: i32, mut argv: *mut *mut u8) -> i32 {
    let mut name: *mut u8;
    let mut value: *mut u8;
    let mut ch: u8;
    let mut octal: [u8; 4];
    name = *argv.offset(1isize);
    if name.is_null() {
        _exit(100i32);
    }
    value = *argv.offset(2isize);
    if value.is_null() {
        _exit(100i32);
    }
    puts((*b"const char \0").as_ptr());
    puts(name as (*const u8));
    puts((*b"[] = \"\\\n\0").as_ptr());
    'loop5: loop {
        if {
            ch = *{
                let _old = value;
                value = value.offset(1isize);
                _old
            };
            ch
        } == 0
        {
            break;
        }
        puts((*b"\\\0").as_ptr());
        octal[3usize] = 0u8;
        octal[2usize] = (b'0' as (i32) + (ch as (i32) & 7i32)) as (u8);
        ch = (ch as (i32) >> 3i32) as (u8);
        octal[1usize] = (b'0' as (i32) + (ch as (i32) & 7i32)) as (u8);
        ch = (ch as (i32) >> 3i32) as (u8);
        octal[0usize] = (b'0' as (i32) + (ch as (i32) & 7i32)) as (u8);
        puts(octal.as_mut_ptr() as (*const u8));
    }
    puts((*b"\\\n\";\n\0").as_ptr());
    if buffer_flush(&mut b as (*mut buffer)) == -1i32 {
        _exit(111i32);
    }
    _exit(0i32);
    0
}
