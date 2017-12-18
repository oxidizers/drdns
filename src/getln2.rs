extern {
    fn buffer_feed(arg1 : *mut buffer) -> i32;
    fn buffer_get(
        arg1 : *mut buffer, arg2 : *mut u8, arg3 : u32
    ) -> i32;
    fn byte_chr(s : *mut u8, n : u32, c : i32) -> u32;
    fn stralloc_ready(arg1 : *mut stralloc, arg2 : u32) -> i32;
    fn stralloc_readyplus(arg1 : *mut stralloc, arg2 : u32) -> i32;
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

#[derive(Copy)]
#[repr(C)]
pub struct stralloc {
    pub s : *mut u8,
    pub len : u32,
    pub a : u32,
}

impl Clone for stralloc {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn getln2(
    mut ss : *mut buffer,
    mut sa : *mut stralloc,
    mut cont : *mut *mut u8,
    mut clen : *mut u32,
    mut sep : i32
) -> i32 {
    let mut _currentBlock;
    let mut x : *mut u8;
    let mut i : u32;
    let mut n : i32;
    if stralloc_ready(sa,0u32) == 0 {
        -1i32
    } else {
        (*sa).len = 0u32;
        'loop2: loop {
            n = buffer_feed(ss);
            if n < 0i32 {
                _currentBlock = 10;
                break;
            }
            if n == 0i32 {
                _currentBlock = 9;
                break;
            }
            x = (*ss).x.offset((*ss).n as (isize));
            i = byte_chr(x,n as (u32),sep);
            if i < n as (u32) {
                _currentBlock = 8;
                break;
            }
            if stralloc_readyplus(sa,n as (u32)) == 0 {
                _currentBlock = 7;
                break;
            }
            i = (*sa).len;
            (*sa).len = i.wrapping_add(
                            buffer_get(ss,(*sa).s.offset(i as (isize)),n as (u32)) as (u32)
                        );
        }
        (if _currentBlock == 7 {
             -1i32
         } else if _currentBlock == 8 {
             (*ss).p = (*ss).p.wrapping_sub(
                           {
                               *clen = i.wrapping_add(1u32);
                               *clen
                           }
                       );
             (*ss).n = (*ss).n.wrapping_add(
                           {
                               *clen = i.wrapping_add(1u32);
                               *clen
                           }
                       );
             *cont = x;
             0i32
         } else if _currentBlock == 9 {
             *clen = 0u32;
             0i32
         } else {
             -1i32
         })
    }
}