extern "C" {
    fn buffer_unixwrite(arg1: i32, arg2: *const u8, arg3: u32) -> i32;
}

#[no_mangle]
pub static mut buffer_2_space: [u8; 256] = [0u8; 256];

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

static mut it: buffer = buffer {
    x: buffer_2_space.as_mut_ptr(),
    p: 0u32,
    n: ::std::mem::size_of::<[u8; 256]>() as (u32),
    fd: 2i32,
    op: buffer_unixwrite as (unsafe extern "C" fn() -> i32),
};

#[no_mangle]
pub static mut buffer_2: *mut buffer = &mut it as (*mut buffer);
