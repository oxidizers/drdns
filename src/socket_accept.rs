extern "C" {
    fn accept(arg1: i32, arg2: *mut sockaddr, arg3: *mut u32) -> i32;
    fn byte_copy(to: *mut u8, n: u32, from: *mut u8);
    fn uint16_unpack_big(arg1: *const u8, arg2: *mut u16);
}

#[derive(Copy)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

impl Clone for in_addr {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_len: u8,
    pub sin_family: u8,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

impl Clone for sockaddr_in {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct sockaddr {
    pub sa_len: u8,
    pub sa_family: u8,
    pub sa_data: [u8; 14],
}

impl Clone for sockaddr {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn socket_accept4(mut s: i32, mut ip: *mut u8, mut port: *mut u16) -> i32 {
    let mut sa: sockaddr_in;
    let mut dummy: i32 = ::std::mem::size_of::<sockaddr_in>() as (i32);
    let mut fd: i32;
    fd = accept(
        s,
        &mut sa as (*mut sockaddr_in) as (*mut sockaddr),
        &mut dummy as (*mut i32) as (*mut u32),
    );
    if fd == -1i32 {
        -1i32
    } else {
        byte_copy(ip, 4u32, &mut sa.sin_addr as (*mut in_addr) as (*mut u8));
        uint16_unpack_big(
            &mut sa.sin_port as (*mut u16) as (*mut u8) as (*const u8),
            port,
        );
        fd
    }
}
