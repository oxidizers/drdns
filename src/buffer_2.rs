//! `buffer_2.rs`: a 256-byte thread-unsafe static buffer used for STDERR(?)
//!
//! TODO: document those things and find a better way to handle them

use buffer::{self, Buffer};

const BUFFER_2_SIZE: usize = 256;

static mut BUFFER_2_SPACE: [u8; BUFFER_2_SIZE] = [0u8; BUFFER_2_SIZE];

pub static mut BUFFER_2: Buffer = Buffer {
    x: unsafe { &mut BUFFER_2_SPACE as *mut [u8] as *mut u8 },
    p: 0u32,
    n: BUFFER_2_SIZE as u32,
    fd: 2i32,
    op: Some(buffer::unixwrite as buffer::Op),
};
