use byte;
use libc;
use ndelay;
use uint16;

extern "C" {
    fn accept(arg1: i32, arg2: *mut sockaddr, arg3: *mut u32) -> i32;
    fn bind(arg1: i32, arg2: *const sockaddr, arg3: u32) -> i32;
    fn close(arg1: i32) -> i32;
    fn connect(arg1: i32, arg2: *const sockaddr, arg3: u32) -> i32;
    fn getpeername(arg1: i32, arg2: *mut sockaddr, arg3: *mut u32) -> i32;
    fn listen(arg1: i32, arg2: i32) -> i32;
    fn recvfrom(
        arg1: i32,
        arg2: *mut ::std::os::raw::c_void,
        arg3: usize,
        arg4: i32,
        arg5: *mut sockaddr,
        arg6: *mut u32,
    ) -> isize;
    fn setsockopt(
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: *const ::std::os::raw::c_void,
        arg5: u32,
    ) -> i32;
    fn socket(arg1: i32, arg2: i32, arg3: i32) -> i32;
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
        byte::copy(ip, 4u32, &mut sa.sin_addr as (*mut in_addr) as (*mut u8));
        uint16::unpack_big(
            &mut sa.sin_port as (*mut u16) as (*mut u8) as (*const u8),
            port,
        );
        fd
    }
}

#[no_mangle]
pub unsafe extern "C" fn socket_bind4(mut s: i32, mut ip: *mut u8, mut port: u16) -> i32 {
    let mut sa: sockaddr_in;
    byte::zero(
        &mut sa as (*mut sockaddr_in) as (*mut u8),
        ::std::mem::size_of::<sockaddr_in>() as (u32),
    );
    sa.sin_family = 2u8;
    uint16::pack_big(&mut sa.sin_port as (*mut u16) as (*mut u8), port);
    byte::copy(&mut sa.sin_addr as (*mut in_addr) as (*mut u8), 4u32, ip);
    bind(
        s,
        &mut sa as (*mut sockaddr_in) as (*mut sockaddr) as (*const sockaddr),
        ::std::mem::size_of::<sockaddr_in>() as (u32),
    )
}

#[no_mangle]
pub unsafe extern "C" fn socket_bind4_reuse(mut s: i32, mut ip: *mut u8, mut port: u16) -> i32 {
    let mut opt: i32 = 1i32;
    setsockopt(
        s,
        0xffffi32,
        0x4i32,
        &mut opt as (*mut i32) as (*const ::std::os::raw::c_void),
        ::std::mem::size_of::<i32>() as (u32),
    );
    socket_bind4(s, ip, port)
}

#[no_mangle]
pub unsafe extern "C" fn socket_tryreservein(mut s: i32, mut size: i32) {
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
            ::std::mem::size_of::<i32>() as (u32),
        ) == 0i32
        {
            _currentBlock = 4;
            break;
        }
        size = size - (size >> 5i32);
    }
    if _currentBlock == 1 {}
}

#[no_mangle]
pub unsafe extern "C" fn socket_connect4(mut s: i32, mut ip: *const u8, mut port: u16) -> i32 {
    let mut sa: sockaddr_in;
    byte::zero(
        &mut sa as (*mut sockaddr_in) as (*mut u8),
        ::std::mem::size_of::<sockaddr_in>() as (u32),
    );
    sa.sin_family = 2u8;
    uint16::pack_big(&mut sa.sin_port as (*mut u16) as (*mut u8), port);
    byte::copy(
        &mut sa.sin_addr as (*mut in_addr) as (*mut u8),
        4u32,
        ip as (*mut u8),
    );
    connect(
        s,
        &mut sa as (*mut sockaddr_in) as (*mut sockaddr) as (*const sockaddr),
        ::std::mem::size_of::<sockaddr_in>() as (u32),
    )
}

#[no_mangle]
pub unsafe extern "C" fn socket_connected(mut s: i32) -> i32 {
    let mut sa: sockaddr_in;
    let mut dummy: i32;
    let mut ch: u8;
    dummy = ::std::mem::size_of::<sockaddr_in>() as (i32);
    if getpeername(
        s,
        &mut sa as (*mut sockaddr_in) as (*mut sockaddr),
        &mut dummy as (*mut i32) as (*mut u32),
    ) == -1i32
    {
        libc::read(
            s,
            &mut ch as (*mut u8) as (*mut libc::c_void),
            1usize,
        );
        0i32
    } else {
        1i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn socket_recv4(
    mut s: i32,
    mut buf: *mut u8,
    mut len: i32,
    mut ip: *mut u8,
    mut port: *mut u16,
) -> i32 {
    let mut sa: sockaddr_in;
    let mut dummy: i32 = ::std::mem::size_of::<sockaddr_in>() as (i32);
    let mut r: i32;
    r = recvfrom(
        s,
        buf as (*mut ::std::os::raw::c_void),
        len as (usize),
        0i32,
        &mut sa as (*mut sockaddr_in) as (*mut sockaddr),
        &mut dummy as (*mut i32) as (*mut u32),
    ) as (i32);
    if r == -1i32 {
        -1i32
    } else {
        byte::copy(ip, 4u32, &mut sa.sin_addr as (*mut in_addr) as (*mut u8));
        uint16::unpack_big(
            &mut sa.sin_port as (*mut u16) as (*mut u8) as (*const u8),
            port,
        );
        r
    }
}

#[no_mangle]
pub unsafe extern "C" fn socket_send4(
    mut s: i32,
    mut buf: *const u8,
    mut len: i32,
    mut ip: *const u8,
    mut port: u16,
) -> i32 {
    let mut sa: sockaddr_in;
    byte::zero(
        &mut sa as (*mut sockaddr_in) as (*mut u8),
        ::std::mem::size_of::<sockaddr_in>() as (u32),
    );
    sa.sin_family = 2u8;
    uint16::pack_big(&mut sa.sin_port as (*mut u16) as (*mut u8), port);
    byte::copy(
        &mut sa.sin_addr as (*mut in_addr) as (*mut u8),
        4u32,
        ip as (*mut u8),
    );
    sendto(
        s,
        buf as (*const ::std::os::raw::c_void),
        len as (usize),
        0i32,
        &mut sa as (*mut sockaddr_in) as (*mut sockaddr) as (*const sockaddr),
        ::std::mem::size_of::<sockaddr_in>() as (u32),
    ) as (i32)
}

#[no_mangle]
pub unsafe extern "C" fn socket_tcp() -> i32 {
    let mut s: i32;
    s = socket(2i32, 1i32, 0i32);
    if s == -1i32 {
        -1i32
    } else if ndelay::on(s) == -1i32 {
        close(s);
        -1i32
    } else {
        s
    }
}

#[no_mangle]
pub unsafe extern "C" fn socket_udp() -> i32 {
    let mut s: i32;
    s = socket(2i32, 2i32, 0i32);
    if s == -1i32 {
        -1i32
    } else if ndelay::on(s) == -1i32 {
        close(s);
        -1i32
    } else {
        s
    }
}
