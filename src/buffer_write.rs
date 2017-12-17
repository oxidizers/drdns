extern {
    fn write(
        __fd : i32, __buf : *const ::std::os::raw::c_void, __nbyte : usize
    ) -> isize;
}

#[no_mangle]
pub unsafe extern fn buffer_unixwrite(
    mut fd : i32, mut buf : *const u8, mut len : u32
) -> i32 {
    write(
        fd,
        buf as (*const ::std::os::raw::c_void),
        len as (usize)
    ) as (i32)
}
