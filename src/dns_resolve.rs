use tai::Tai;
use taia::TaiA;

extern "C" {
    fn dns_resolvconfip(arg1: *mut u8) -> i32;
    fn dns_transmit_get(arg1: *mut dns_transmit, arg2: *const pollfd, arg3: *const TaiA) -> i32;
    fn dns_transmit_io(arg1: *mut dns_transmit, arg2: *mut pollfd, arg3: *mut TaiA);
    fn dns_transmit_start(
        arg1: *mut dns_transmit,
        arg2: *const u8,
        arg3: i32,
        arg4: *const u8,
        arg5: *const u8,
        arg6: *const u8,
    ) -> i32;
    fn iopause(arg1: *mut pollfd, arg2: u32, arg3: *mut TaiA, arg4: *mut TaiA);
}

#[derive(Copy)]
#[repr(C)]
pub struct dns_transmit {
    pub query: *mut u8,
    pub querylen: u32,
    pub packet: *mut u8,
    pub packetlen: u32,
    pub s1: i32,
    pub tcpstate: i32,
    pub udploop: u32,
    pub curserver: u32,
    pub deadline: TaiA,
    pub pos: u32,
    pub servers: *const u8,
    pub localip: [u8; 4],
    pub qtype: [u8; 2],
}

impl Clone for dns_transmit {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub static mut dns_resolve_tx: dns_transmit = dns_transmit {
    query: 0i32 as (*mut u8),
    querylen: 0u32,
    packet: 0 as (*mut u8),
    packetlen: 0u32,
    s1: 0i32,
    tcpstate: 0i32,
    udploop: 0u32,
    curserver: 0u32,
    deadline: TaiA {
        sec: Tai { x: 0usize },
        nano: 0usize,
        atto: 0usize,
    },
    pos: 0u32,
    servers: 0 as (*const u8),
    localip: [0u8; 4],
    qtype: [0u8; 2],
};

#[derive(Copy)]
#[repr(C)]
pub struct pollfd {
    pub fd: i32,
    pub events: i16,
    pub revents: i16,
}

impl Clone for pollfd {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn dns_resolve(mut q: *const u8, mut qtype: *const u8) -> i32 {
    let mut _currentBlock;
    let mut stamp: TaiA;
    let mut deadline: TaiA;
    let mut servers: [u8; 64];
    let mut x: [pollfd; 1];
    let mut r: i32;
    if dns_resolvconfip(servers.as_mut_ptr()) == -1i32 {
        -1i32
    } else if dns_transmit_start(
        &mut dns_resolve_tx as (*mut dns_transmit),
        servers.as_mut_ptr() as (*const u8),
        1i32,
        q,
        qtype,
        (*b"\0\0\0\0\0").as_ptr(),
    ) == -1i32
    {
        -1i32
    } else {
        'loop2: loop {
            TaiA::now(&mut stamp as (*mut TaiA));
            TaiA::uint(&mut deadline as (*mut TaiA), 120u32);
            TaiA::add(
                &mut deadline as (*mut TaiA),
                &mut deadline as (*mut TaiA) as (*const TaiA),
                &mut stamp as (*mut TaiA) as (*const TaiA),
            );
            dns_transmit_io(
                &mut dns_resolve_tx as (*mut dns_transmit),
                x.as_mut_ptr(),
                &mut deadline as (*mut TaiA),
            );
            iopause(
                x.as_mut_ptr(),
                1u32,
                &mut deadline as (*mut TaiA),
                &mut stamp as (*mut TaiA),
            );
            r = dns_transmit_get(
                &mut dns_resolve_tx as (*mut dns_transmit),
                x.as_mut_ptr() as (*const pollfd),
                &mut stamp as (*mut TaiA) as (*const TaiA),
            );
            if r == -1i32 {
                _currentBlock = 5;
                break;
            }
            if r == 1i32 {
                _currentBlock = 4;
                break;
            }
        }
        (if _currentBlock == 4 { 0i32 } else { -1i32 })
    }
}
