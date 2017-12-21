use buffer::{Buffer, STDERR_BUFFER};
use byte;
use strerr::{StrErr, STRERR_SYS};

extern "C" {
    fn case_lowerb(arg1: *mut u8, arg2: u32);
    fn dns_domain_length(arg1: *const u8) -> u32;
    fn dns_packet_copy(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut u8, arg5: u32) -> u32;
    fn dns_packet_getname(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut *mut u8) -> u32;
    fn droproot(arg1: *const u8);
    fn env_get(arg1: *const u8) -> *mut u8;
    static mut fatal: *mut u8;
    fn initialize();
    fn ip4_scan(arg1: *const u8, arg2: *mut u8) -> u32;
    fn ndelay_off(arg1: i32) -> i32;
    fn qlog(
        arg1: *const u8,
        arg2: u16,
        arg3: *const u8,
        arg4: *const u8,
        arg5: *const u8,
        arg6: *const u8,
    );
    fn respond(arg1: *mut u8, arg2: *mut u8, arg3: *mut u8) -> i32;
    static mut response: *mut u8;
    fn response_id(arg1: *const u8);
    static mut response_len: u32;
    fn response_query(arg1: *const u8, arg2: *const u8, arg3: *const u8) -> i32;
    fn response_tc();
    fn socket_bind4_reuse(arg1: i32, arg2: *mut u8, arg3: u16) -> i32;
    fn socket_recv4(arg1: i32, arg2: *mut u8, arg3: i32, arg4: *mut u8, arg5: *mut u16) -> i32;
    fn socket_send4(arg1: i32, arg2: *const u8, arg3: i32, arg4: *const u8, arg5: u16) -> i32;
    fn socket_tryreservein(arg1: i32, arg2: i32);
    fn socket_udp() -> i32;
    static mut starting: *mut u8;
}

static mut ip: [u8; 4] = [0u8; 4];

static mut port: u16 = 0u16;

static mut buf: [u8; 513] = [0u8; 513];

static mut len: i32 = 0i32;

static mut q: *mut u8 = 0 as (*mut u8);

fn main() {
    let ret = unsafe { _c_main() };
    ::std::process::exit(ret);
}

unsafe extern "C" fn doit() -> i32 {
    let mut pos: u32;
    let mut header: [u8; 12];
    let mut qtype: [u8; 2];
    let mut qclass: [u8; 2];
    if !(len as (usize) >= ::std::mem::size_of::<[u8; 513]>()) {
        pos = dns_packet_copy(
            buf.as_mut_ptr() as (*const u8),
            len as (u32),
            0u32,
            header.as_mut_ptr(),
            12u32,
        );
        if !(pos == 0) {
            if header[2usize] as (i32) & 128i32 == 0 {
                if header[4usize] == 0 {
                    if !(header[5usize] as (i32) != 1i32) {
                        pos = dns_packet_getname(
                            buf.as_mut_ptr() as (*const u8),
                            len as (u32),
                            pos,
                            &mut q as (*mut *mut u8),
                        );
                        if !(pos == 0) {
                            pos = dns_packet_copy(
                                buf.as_mut_ptr() as (*const u8),
                                len as (u32),
                                pos,
                                qtype.as_mut_ptr(),
                                2u32,
                            );
                            if !(pos == 0) {
                                pos = dns_packet_copy(
                                    buf.as_mut_ptr() as (*const u8),
                                    len as (u32),
                                    pos,
                                    qclass.as_mut_ptr(),
                                    2u32,
                                );
                                if !(pos == 0) {
                                    if !(response_query(
                                        q as (*const u8),
                                        qtype.as_mut_ptr() as (*const u8),
                                        qclass.as_mut_ptr() as (*const u8),
                                    ) == 0)
                                    {
                                        response_id(header.as_mut_ptr() as (*const u8));
                                        if byte::diff(
                                            qclass.as_mut_ptr(),
                                            2u32,
                                            (*b"\0\x01\0").as_ptr() as (*mut u8),
                                        ) == 0
                                        {
                                            let _rhs = 4i32;
                                            let _lhs = &mut *response.offset(2isize);
                                            *_lhs = (*_lhs as (i32) | _rhs) as (u8);
                                        } else if byte::diff(
                                            qclass.as_mut_ptr(),
                                            2u32,
                                            (*b"\0\xFF\0").as_ptr() as (*mut u8),
                                        ) != 0
                                        {
                                            let _rhs = !15i32;
                                            let _lhs = &mut *response.offset(3isize);
                                            *_lhs = (*_lhs as (i32) & _rhs) as (u8);
                                            let _rhs = 1i32;
                                            let _lhs = &mut *response.offset(3isize);
                                            *_lhs = (*_lhs as (i32) | _rhs) as (u8);
                                            qlog(
                                                ip.as_mut_ptr() as (*const u8),
                                                port,
                                                header.as_mut_ptr() as (*const u8),
                                                q as (*const u8),
                                                qtype.as_mut_ptr() as (*const u8),
                                                (*b" C \0").as_ptr(),
                                            );
                                            return 1i32;
                                        }
                                        let _rhs = !128i32;
                                        let _lhs = &mut *response.offset(3isize);
                                        *_lhs = (*_lhs as (i32) & _rhs) as (u8);
                                        if header[2usize] as (i32) & 1i32 == 0 {
                                            let _rhs = !1i32;
                                            let _lhs = &mut *response.offset(2isize);
                                            *_lhs = (*_lhs as (i32) & _rhs) as (u8);
                                        }
                                        if header[2usize] as (i32) & 126i32 == 0 {
                                            if !(byte::diff(
                                                qtype.as_mut_ptr(),
                                                2u32,
                                                (*b"\0\xFC\0").as_ptr() as (*mut u8),
                                            ) ==
                                                     0)
                                            {
                                                case_lowerb(q, dns_domain_length(q as (*const u8)));
                                                if respond(
                                                    q,
                                                    qtype.as_mut_ptr(),
                                                    ip.as_mut_ptr(),
                                                ) ==
                                                    0
                                                {
                                                    qlog(
                                                        ip.as_mut_ptr() as (*const u8),
                                                        port,
                                                        header.as_mut_ptr() as (*const u8),
                                                        q as (*const u8),
                                                        qtype.as_mut_ptr() as (*const u8),
                                                        (*b" - \0").as_ptr(),
                                                    );
                                                    return 0i32;
                                                } else {
                                                    qlog(
                                                        ip.as_mut_ptr() as (*const u8),
                                                        port,
                                                        header.as_mut_ptr() as (*const u8),
                                                        q as (*const u8),
                                                        qtype.as_mut_ptr() as (*const u8),
                                                        (*b" + \0").as_ptr(),
                                                    );
                                                    return 1i32;
                                                }
                                            }
                                        }
                                        let _rhs = !15i32;
                                        let _lhs = &mut *response.offset(3isize);
                                        *_lhs = (*_lhs as (i32) & _rhs) as (u8);
                                        let _rhs = 4i32;
                                        let _lhs = &mut *response.offset(3isize);
                                        *_lhs = (*_lhs as (i32) | _rhs) as (u8);
                                        qlog(
                                            ip.as_mut_ptr() as (*const u8),
                                            port,
                                            header.as_mut_ptr() as (*const u8),
                                            q as (*const u8),
                                            qtype.as_mut_ptr() as (*const u8),
                                            (*b" I \0").as_ptr(),
                                        );
                                        return 1i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    qlog(
        ip.as_mut_ptr() as (*const u8),
        port,
        (*b"\0\0\0").as_ptr(),
        (*b"\0").as_ptr(),
        (*b"\0\0\0").as_ptr(),
        (*b" / \0").as_ptr(),
    );
    0i32
}

#[no_mangle]
pub unsafe extern "C" fn _c_main() -> i32 {
    let mut x: *mut u8;
    let mut udp53: i32;
    x = env_get((*b"IP\0").as_ptr());
    if x.is_null() {
        StrErr::die(
            111i32,
            fatal as (*const u8),
            (*b"$IP not set\0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const StrErr),
        );
    }
    if ip4_scan(x as (*const u8), ip.as_mut_ptr()) == 0 {
        StrErr::die(
            111i32,
            fatal as (*const u8),
            (*b"unable to parse IP address \0").as_ptr(),
            x as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const StrErr),
        );
    }
    udp53 = socket_udp();
    if udp53 == -1i32 {
        StrErr::die(
            111i32,
            fatal as (*const u8),
            (*b"unable to create UDP socket: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
        );
    }
    if socket_bind4_reuse(udp53, ip.as_mut_ptr(), 53u16) == -1i32 {
        StrErr::die(
            111i32,
            fatal as (*const u8),
            (*b"unable to bind UDP socket: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
        );
    }
    droproot(fatal as (*const u8));
    initialize();
    ndelay_off(udp53);
    socket_tryreservein(udp53, 65536i32);
    Buffer::putsflush(STDERR_BUFFER.as_mut_ptr(), starting as (*const u8));
    'loop9: loop {
        len = socket_recv4(
            udp53,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[u8; 513]>() as (i32),
            ip.as_mut_ptr(),
            &mut port as (*mut u16),
        );
        if len < 0i32 {
            continue;
        }
        if doit() == 0 {
            continue;
        }
        if response_len > 512u32 {
            response_tc();
        }
        socket_send4(
            udp53,
            response as (*const u8),
            response_len as (i32),
            ip.as_mut_ptr() as (*const u8),
            port,
        );
    }
}
