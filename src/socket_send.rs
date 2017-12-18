extern {
    fn byte_copy(to : *mut u8, n : u32, from : *mut u8);
    fn byte_zero(s : *mut u8, n : u32);
    fn sendto(
        arg1 : i32,
        arg2 : *const ::std::os::raw::c_void,
        arg3 : usize,
        arg4 : i32,
        arg5 : *const sockaddr,
        arg6 : u32
    ) -> isize;
    fn uint16_pack_big(arg1 : *mut u8, arg2 : u16);
}

#[derive(Copy)]
#[repr(C)]
pub struct in_addr {
    pub s_addr : u32,
}

impl Clone for in_addr {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_len : u8,
    pub sin_family : u8,
    pub sin_port : u16,
    pub sin_addr : in_addr,
    pub sin_zero : [u8; 8],
}

impl Clone for sockaddr_in {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct sockaddr {
    pub sa_len : u8,
    pub sa_family : u8,
    pub sa_data : [u8; 14],
}

impl Clone for sockaddr {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn socket_send4(
    mut s : i32,
    mut buf : *const u8,
    mut len : i32,
    mut ip : *const u8,
    mut port : u16
) -> i32 {
    let mut sa : sockaddr_in;
    byte_zero(
        &mut sa as (*mut sockaddr_in) as (*mut u8),
        ::std::mem::size_of::<sockaddr_in>() as (u32)
    );
    sa.sin_family = 2u8;
    uint16_pack_big(&mut sa.sin_port as (*mut u16) as (*mut u8),port);
    byte_copy(
        &mut sa.sin_addr as (*mut in_addr) as (*mut u8),
        4u32,
        ip as (*mut u8)
    );
    sendto(
        s,
        buf as (*const ::std::os::raw::c_void),
        len as (usize),
        0i32,
        &mut sa as (*mut sockaddr_in) as (*mut sockaddr) as (*const sockaddr),
        ::std::mem::size_of::<sockaddr_in>() as (u32)
    ) as (i32)
}
