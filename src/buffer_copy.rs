extern {
    fn buffer_feed(arg1 : *mut buffer) -> i32;
    fn buffer_put(
        arg1 : *mut buffer, arg2 : *const u8, arg3 : u32
    ) -> i32;
}

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
pub unsafe extern fn buffer_copy(
    mut bout : *mut buffer, mut bin : *mut buffer
) -> i32 {
    let mut _currentBlock;
    let mut n : i32;
    let mut x : *mut u8;
    'loop1: loop {
        n = buffer_feed(bin);
        if n < 0i32 {
            _currentBlock = 7;
            break;
        }
        if n == 0 {
            _currentBlock = 6;
            break;
        }
        x = (*bin).x.offset((*bin).n as (isize));
        if buffer_put(bout,x as (*const u8),n as (u32)) == -1i32 {
            _currentBlock = 5;
            break;
        }
        (*bin).p = (*bin).p.wrapping_sub(n as (u32));
        (*bin).n = (*bin).n.wrapping_add(n as (u32));
    }
    if _currentBlock == 5 {
        -3i32
    } else if _currentBlock == 6 {
        0i32
    } else {
        -2i32
    }
}
