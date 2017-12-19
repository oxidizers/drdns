use buffer::{self, Buffer};
use libc;

#[no_mangle]
pub static mut bspace: [u8; 256] = [0u8; 256];

#[no_mangle]
pub static mut b: Buffer = Buffer {
    x: bspace.as_mut_ptr(),
    p: 0u32,
    n: ::std::mem::size_of::<[u8; 256]>() as (u32),
    fd: 1i32,
    op: Some(buffer::unixwrite as buffer::Op),
};

#[no_mangle]
pub unsafe extern "C" fn puts(mut s: *const u8) {
    if Buffer::puts(&mut b as (*mut Buffer), s) == -1i32 {
        libc::_exit(111i32);
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
        libc::_exit(100i32);
    }
    value = *argv.offset(2isize);
    if value.is_null() {
        libc::_exit(100i32);
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
    if Buffer::flush(&mut b as (*mut Buffer)) == -1i32 {
        libc::_exit(111i32);
    }
    libc::_exit(0i32);
}
