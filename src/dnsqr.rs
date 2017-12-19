use errno::errno;
use libc;

extern "C" {
    static mut buffer_1: *mut buffer;
    fn buffer_putflush(arg1: *mut buffer, arg2: *const u8, arg3: u32) -> i32;
    fn dns_domain_fromdot(arg1: *mut *mut u8, arg2: *const u8, arg3: u32) -> i32;
    fn dns_domain_todot_cat(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn dns_random_init(arg1: *const u8);
    fn dns_resolve(arg1: *const u8, arg2: *const u8) -> i32;
    static mut dns_resolve_tx: dns_transmit;
    fn parsetype(arg1: *mut u8, arg2: *mut u8) -> i32;
    fn printpacket_cat(arg1: *mut stralloc, arg2: *mut u8, arg3: u32) -> u32;
    fn str_len(arg1: *const u8) -> u32;
    fn stralloc_cats(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn stralloc_catulong0(arg1: *mut stralloc, arg2: usize, arg3: u32) -> i32;
    fn stralloc_copys(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn strerr_die(
        arg1: i32,
        arg2: *const u8,
        arg3: *const u8,
        arg4: *const u8,
        arg5: *const u8,
        arg6: *const u8,
        arg7: *const u8,
        arg8: *const strerr,
    );
    static mut strerr_sys: strerr;
    fn uint16_unpack_big(arg1: *const u8, arg2: *mut u16);
}

#[derive(Copy)]
#[repr(C)]
pub struct strerr {
    pub who: *mut strerr,
    pub x: *const u8,
    pub y: *const u8,
    pub z: *const u8,
}

impl Clone for strerr {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn usage() {
    strerr_die(
        100i32,
        (*b"dnsqr: usage: dnsqr type name\0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const strerr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn oops() {
    strerr_die(
        111i32,
        (*b"dnsqr: fatal: \0").as_ptr(),
        (*b"unable to parse: \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut strerr_sys as (*mut strerr) as (*const strerr),
    );
}

#[no_mangle]
pub static mut type_: [u8; 2] = [0u8; 2];

static mut q: *mut u8 = 0 as (*mut u8);

#[derive(Copy)]
#[repr(C)]
pub struct stralloc {
    pub s: *mut u8,
    pub len: u32,
    pub a: u32,
}

impl Clone for stralloc {
    fn clone(&self) -> Self {
        *self
    }
}

static mut out: stralloc = stralloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

static mut seed: [u8; 128] = [0u8; 128];

fn main() {
    use std::os::unix::ffi::OsStringExt;
    let mut argv_storage = ::std::env::args_os()
        .map(|str| {
            let mut vec = str.into_vec();
            vec.push(b'\0');
            vec
        })
        .collect::<Vec<_>>();
    let mut argv = argv_storage
        .iter_mut()
        .map(|vec| vec.as_mut_ptr())
        .chain(Some(::std::ptr::null_mut()))
        .collect::<Vec<_>>();
    let ret = unsafe { _c_main(argv_storage.len() as (i32), argv.as_mut_ptr()) };
    ::std::process::exit(ret);
}

#[derive(Copy)]
#[repr(C)]
pub struct tai {
    pub x: usize,
}

impl Clone for tai {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct taia {
    pub sec: tai,
    pub nano: usize,
    pub atto: usize,
}

impl Clone for taia {
    fn clone(&self) -> Self {
        *self
    }
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
    pub deadline: taia,
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

#[derive(Copy)]
#[repr(C)]
pub struct buffer {
    pub x: *mut u8,
    pub p: u32,
    pub n: u32,
    pub fd: i32,
    pub op: unsafe extern "C" fn() -> i32,
}

impl Clone for buffer {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn _c_main(mut argc: i32, mut argv: *mut *mut u8) -> i32 {
    let mut u16: u16;
    dns_random_init(seed.as_mut_ptr() as (*const u8));
    if (*argv).is_null() {
        usage();
    }
    if (*{
            argv = argv.offset(1isize);
            argv
        }).is_null()
    {
        usage();
    }
    if parsetype(*argv, type_.as_mut_ptr()) == 0 {
        usage();
    }
    if (*{
            argv = argv.offset(1isize);
            argv
        }).is_null()
    {
        usage();
    }
    if dns_domain_fromdot(
        &mut q as (*mut *mut u8),
        *argv as (*const u8),
        str_len(*argv as (*const u8)),
    ) == 0
    {
        oops();
    }
    if !(*{
             argv = argv.offset(1isize);
             argv
         }).is_null()
    {
        usage();
    }
    if stralloc_copys(&mut out as (*mut stralloc), (*b"\0").as_ptr()) == 0 {
        oops();
    }
    uint16_unpack_big(type_.as_mut_ptr() as (*const u8), &mut u16 as (*mut u16));
    if stralloc_catulong0(&mut out as (*mut stralloc), u16 as (usize), 0u32) == 0 {
        oops();
    }
    if stralloc_cats(&mut out as (*mut stralloc), (*b" \0").as_ptr()) == 0 {
        oops();
    }
    if dns_domain_todot_cat(&mut out as (*mut stralloc), q as (*const u8)) == 0 {
        oops();
    }
    if stralloc_cats(&mut out as (*mut stralloc), (*b":\n\0").as_ptr()) == 0 {
        oops();
    }
    if dns_resolve(q as (*const u8), type_.as_mut_ptr() as (*const u8)) == -1i32 {
        if stralloc_cats(&mut out as (*mut stralloc), libc::strerror(errno().0)) == 0 {
            oops();
        }
        if stralloc_cats(&mut out as (*mut stralloc), (*b"\n\0").as_ptr()) == 0 {
            oops();
        }
    } else {
        if dns_resolve_tx.packetlen < 4u32 {
            oops();
        }
        let _rhs = !1i32;
        let _lhs = &mut *dns_resolve_tx.packet.offset(2isize);
        *_lhs = (*_lhs as (i32) & _rhs) as (u8);
        let _rhs = !128i32;
        let _lhs = &mut *dns_resolve_tx.packet.offset(3isize);
        *_lhs = (*_lhs as (i32) & _rhs) as (u8);
        if printpacket_cat(
            &mut out as (*mut stralloc),
            dns_resolve_tx.packet,
            dns_resolve_tx.packetlen,
        ) == 0
        {
            oops();
        }
    }
    buffer_putflush(buffer_1, out.s as (*const u8), out.len);
    libc::_exit(0i32);
}
