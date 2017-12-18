extern {
    fn bind(arg1 : i32, arg2 : *const sockaddr, arg3 : u32) -> i32;
    fn byte_copy(to : *mut u8, n : u32, from : *mut u8);
    fn byte_zero(s : *mut u8, n : u32);
    fn setsockopt(
        arg1 : i32,
        arg2 : i32,
        arg3 : i32,
        arg4 : *const ::std::os::raw::c_void,
        arg5 : u32
    ) -> i32;
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
pub unsafe extern fn socket_bind4(
    mut s : i32, mut ip : *mut u8, mut port : u16
) -> i32 {
    let mut sa : sockaddr_in;
    byte_zero(
        &mut sa as (*mut sockaddr_in) as (*mut u8),
        ::std::mem::size_of::<sockaddr_in>() as (u32)
    );
    sa.sin_family = 2u8;
    uint16_pack_big(&mut sa.sin_port as (*mut u16) as (*mut u8),port);
    byte_copy(&mut sa.sin_addr as (*mut in_addr) as (*mut u8),4u32,ip);
    bind(
        s,
        &mut sa as (*mut sockaddr_in) as (*mut sockaddr) as (*const sockaddr),
        ::std::mem::size_of::<sockaddr_in>() as (u32)
    )
}

#[no_mangle]
pub unsafe extern fn socket_bind4_reuse(
    mut s : i32, mut ip : *mut u8, mut port : u16
) -> i32 {
    let mut opt : i32 = 1i32;
    setsockopt(
        s,
        0xffffi32,
        0x4i32,
        &mut opt as (*mut i32) as (*const ::std::os::raw::c_void),
        ::std::mem::size_of::<i32>() as (u32)
    );
    socket_bind4(s,ip,port)
}

#[no_mangle]
pub unsafe extern fn socket_tryreservein(mut s : i32, mut size : i32) {
    let mut _currentBlock;
    'loop0: loop {
        if !(size >= 1024i32) {
            _currentBlock = 1;
            break;
        }
        if setsockopt(
               s,
               0xffffi32,
               0x1002i32,
               &mut size as (*mut i32) as (*const ::std::os::raw::c_void),
               ::std::mem::size_of::<i32>() as (u32)
           ) == 0i32 {
            _currentBlock = 4;
            break;
        }
        size = size - (size >> 5i32);
    }
    if _currentBlock == 1 { }
}
