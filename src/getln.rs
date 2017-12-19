use buffer::Buffer;

extern "C" {
    fn getln2(
        arg1: *mut Buffer,
        arg2: *mut stralloc,
        arg3: *mut *mut u8,
        arg4: *mut u32,
        arg5: i32,
    ) -> i32;
    fn stralloc_catb(arg1: *mut stralloc, arg2: *const u8, arg3: u32) -> i32;
}

#[derive(Copy)]
#[repr(C)]
pub struct stralloc {
    pub s: *mut u8,
    pub len: u32,
    pub a: u32,
}

impl Clone for stralloc {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn getln(
    mut ss: *mut Buffer,
    mut sa: *mut stralloc,
    mut match_: *mut i32,
    mut sep: i32,
) -> i32 {
    let mut cont: *mut u8;
    let mut clen: u32;
    if getln2(
        ss,
        sa,
        &mut cont as (*mut *mut u8),
        &mut clen as (*mut u32),
        sep,
    ) == -1i32
    {
        -1i32
    } else if clen == 0 {
        *match_ = 0i32;
        0i32
    } else if stralloc_catb(sa, cont as (*const u8), clen) == 0 {
        -1i32
    } else {
        *match_ = 1i32;
        0i32
    }
}
