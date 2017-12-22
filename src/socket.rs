//! `socket.rs`: Socket-related functionality
//!
//! This should probably be replaced by `std::io`

use byte;
use libc;
use ndelay;
use uint16;

pub unsafe fn accept4(s: i32, ip: *mut u8, port: *mut u16) -> i32 {
    let mut sa: libc::sockaddr_in = ::std::mem::zeroed();
    let mut dummy: i32 = ::std::mem::size_of::<libc::sockaddr_in>() as (i32);
    let fd = libc::accept(
        s,
        &mut sa as (*mut libc::sockaddr_in) as (*mut libc::sockaddr),
        &mut dummy as (*mut i32) as (*mut u32),
    );
    if fd == -1i32 {
        -1i32
    } else {
        byte::copy(ip, 4u32, &mut sa.sin_addr as (*mut libc::in_addr) as (*mut u8));
        uint16::unpack_big(
            &mut sa.sin_port as (*mut u16) as (*mut u8) as (*const u8),
            port,
        );
        fd
    }
}

pub unsafe fn bind4(s: i32, ip: *mut u8, port: u16) -> i32 {
    let mut sa: libc::sockaddr_in = ::std::mem::zeroed();
    sa.sin_family = 2;
    uint16::pack_big(&mut sa.sin_port as (*mut u16) as (*mut u8), port);
    byte::copy(&mut sa.sin_addr as (*mut libc::in_addr) as (*mut u8), 4u32, ip);
    libc::bind(
        s,
        &mut sa as (*mut libc::sockaddr_in) as (*mut libc::sockaddr) as (*const libc::sockaddr),
        ::std::mem::size_of::<libc::sockaddr_in>() as (u32),
    )
}

pub unsafe fn bind4_reuse(s: i32, ip: *mut u8, port: u16) -> i32 {
    let mut opt: i32 = 1i32;
    libc::setsockopt(
        s,
        0xffffi32,
        0x4i32,
        &mut opt as (*mut i32) as (*const ::libc::c_void),
        ::std::mem::size_of::<i32>() as (u32),
    );
    bind4(s, ip, port)
}

pub unsafe fn tryreservein(s: i32, mut size: i32) {
    let current_block;
    'loop0: loop {
        if !(size >= 1024i32) {
            current_block = 1;
            break;
        }
        if libc::setsockopt(
            s,
            0xffffi32,
            0x1002i32,
            &mut size as (*mut i32) as (*const ::libc::c_void),
            ::std::mem::size_of::<i32>() as (u32),
        ) == 0i32
        {
            current_block = 4;
            break;
        }
        size = size - (size >> 5i32);
    }
    if current_block == 1 {}
}

pub unsafe fn connect4(s: i32, ip: *const u8, port: u16) -> i32 {
    let mut sa: libc::sockaddr_in = ::std::mem::zeroed();
    sa.sin_family = 2;
    uint16::pack_big(&mut sa.sin_port as (*mut u16) as (*mut u8), port);
    byte::copy(
        &mut sa.sin_addr as (*mut libc::in_addr) as (*mut u8),
        4u32,
        ip as (*mut u8),
    );
    libc::connect(
        s,
        &mut sa as (*mut libc::sockaddr_in) as (*mut libc::sockaddr) as (*const libc::sockaddr),
        ::std::mem::size_of::<libc::sockaddr_in>() as (u32),
    )
}

pub unsafe fn connected(s: i32) -> i32 {
    let mut sa: libc::sockaddr_in = ::std::mem::zeroed();
    let mut dummy: i32;
    let mut ch: u8 = 0;
    dummy = ::std::mem::size_of::<libc::sockaddr_in>() as (i32);
    if libc::getpeername(
        s,
        &mut sa as (*mut libc::sockaddr_in) as (*mut libc::sockaddr),
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

pub unsafe fn recv4(
    s: i32,
    buf: *mut u8,
    len: i32,
    ip: *mut u8,
    port: *mut u16,
) -> i32 {
    let mut sa: libc::sockaddr_in = ::std::mem::zeroed();
    let mut dummy: i32 = ::std::mem::size_of::<libc::sockaddr_in>() as (i32);
    let r = libc::recvfrom(
        s,
        buf as (*mut ::libc::c_void),
        len as (usize),
        0i32,
        &mut sa as (*mut libc::sockaddr_in) as (*mut libc::sockaddr),
        &mut dummy as (*mut i32) as (*mut u32),
    ) as (i32);
    if r == -1i32 {
        -1i32
    } else {
        byte::copy(ip, 4u32, &mut sa.sin_addr as (*mut libc::in_addr) as (*mut u8));
        uint16::unpack_big(
            &mut sa.sin_port as (*mut u16) as (*mut u8) as (*const u8),
            port,
        );
        r
    }
}

pub unsafe fn send4(
    s: i32,
    buf: *const u8,
    len: i32,
    ip: *const u8,
    port: u16,
) -> i32 {
    let mut sa: libc::sockaddr_in = ::std::mem::zeroed();
    sa.sin_family = 2;
    uint16::pack_big(&mut sa.sin_port as (*mut u16) as (*mut u8), port);
    byte::copy(
        &mut sa.sin_addr as (*mut libc::in_addr) as (*mut u8),
        4u32,
        ip as (*mut u8),
    );
    libc::sendto(
        s,
        buf as (*const ::libc::c_void),
        len as (usize),
        0i32,
        &mut sa as (*mut libc::sockaddr_in) as (*mut libc::sockaddr) as (*const libc::sockaddr),
        ::std::mem::size_of::<libc::sockaddr_in>() as (u32),
    ) as (i32)
}

pub unsafe fn tcp() -> i32 {
    let s = libc::socket(2i32, 1i32, 0i32);
    if s == -1i32 {
        -1i32
    } else if ndelay::on(s) == -1i32 {
        libc::close(s);
        -1i32
    } else {
        s
    }
}

pub unsafe fn udp() -> i32 {
    let s = libc::socket(2i32, 2i32, 0i32);
    if s == -1i32 {
        -1i32
    } else if ndelay::on(s) == -1i32 {
        libc::close(s);
        -1i32
    } else {
        s
    }
}
