use buffer::{self, Buffer};

#[no_mangle]
pub static mut buffer_1_space: [u8; 8192] = [0u8; 8192];

static mut it: Buffer = Buffer {
    x: buffer_1_space.as_mut_ptr(),
    p: 0u32,
    n: ::std::mem::size_of::<[u8; 8192]>() as (u32),
    fd: 1i32,
    op: Some(buffer::unixwrite as buffer::Op),
};

#[no_mangle]
pub static mut buffer_1: *mut Buffer = &mut it as (*mut Buffer);
