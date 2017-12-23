use super::{rcip, DnsTransmit};
use iopause::iopause;
use libc;
use tai::Tai;
use taia::TaiA;

pub static mut TX: DnsTransmit = DnsTransmit {
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

pub unsafe fn resolve(q: *const u8, qtype: *const u8) -> i32 {
    let current_block;
    let mut stamp: TaiA = ::std::mem::zeroed();
    let mut deadline: TaiA = ::std::mem::zeroed();
    let mut servers: [u8; 64] = [0u8; 64];
    let mut x: [libc::pollfd; 1] = ::std::mem::zeroed();
    let mut r: i32;
    if rcip::resolvconfip(servers.as_mut_ptr()) == -1i32 {
        -1i32
    } else if DnsTransmit::start(
        &mut TX as (*mut DnsTransmit),
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
            DnsTransmit::io(
                &mut TX as (*mut DnsTransmit),
                x.as_mut_ptr(),
                &mut deadline as (*mut TaiA),
            );
            iopause(
                x.as_mut_ptr(),
                1u32,
                &mut deadline as (*mut TaiA),
                &mut stamp as (*mut TaiA),
            );
            r = DnsTransmit::get(
                &mut TX as (*mut DnsTransmit),
                x.as_mut_ptr() as (*const libc::pollfd),
                &mut stamp as (*mut TaiA) as (*const TaiA),
            );
            if r == -1i32 {
                current_block = 5;
                break;
            }
            if r == 1i32 {
                current_block = 4;
                break;
            }
        }
        (if current_block == 4 { 0i32 } else { -1i32 })
    }
}
