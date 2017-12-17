#[derive(Copy)]
#[repr(C)]
pub struct buffer {
    pub x : *mut u8,
    pub p : u32,
    pub n : u32,
    pub fd : i32,
    pub op : unsafe extern fn() -> i32,
}

impl Clone for buffer {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn buffer_init(
    mut s : *mut buffer,
    mut op : unsafe extern fn() -> i32,
    mut fd : i32,
    mut buf : *mut u8,
    mut len : u32
) {
    (*s).x = buf;
    (*s).fd = fd;
    (*s).op = op;
    (*s).p = 0u32;
    (*s).n = len;
}
