//! `buffer_1.rs`: an 8kB thread-unsafe static buffer used for STDOUT(?)
//!
//! TODO: document those things and find a better way to handle them

use buffer::{self, Buffer};

const BUFFER_1_SIZE: usize = 8192;

static mut BUFFER_1_SPACE: [u8; BUFFER_1_SIZE] = [0u8; BUFFER_1_SIZE];

pub static mut BUFFER_1: Buffer = Buffer {
    x: unsafe { &mut BUFFER_1_SPACE as *mut [u8] as *mut u8 },
    p: 0u32,
    n: BUFFER_1_SIZE as u32,
    fd: 1i32, // STDOUT?
    op: Some(buffer::unixwrite as buffer::Op),
};
