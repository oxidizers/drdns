extern {
    fn read(
        arg1 : i32, arg2 : *mut ::std::os::raw::c_void, arg3 : usize
    ) -> isize;
}

#[no_mangle]
pub unsafe extern fn buffer_unixread(
    mut fd : i32, mut buf : *mut u8, mut len : u32
) -> i32 {
    read(
        fd,
        buf as (*mut ::std::os::raw::c_void),
        len as (usize)
    ) as (i32)
}
