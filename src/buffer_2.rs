use buffer::{self, Buffer};

#[no_mangle]
pub static mut buffer_2_space: [u8; 256] = [0u8; 256];

static mut it: Buffer = Buffer {
    x: buffer_2_space.as_mut_ptr(),
    p: 0u32,
    n: ::std::mem::size_of::<[u8; 256]>() as (u32),
    fd: 2i32,
    op: buffer_unixwrite as buffer::Op,
};

#[no_mangle]
pub static mut buffer_2: *mut Buffer = &mut it as (*mut Buffer);
