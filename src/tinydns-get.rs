use byte;

extern "C" {
    fn _exit(arg1: i32);
    static mut buffer_1: *mut buffer;
    fn buffer_putflush(arg1: *mut buffer, arg2: *const u8, arg3: u32) -> i32;
    fn case_lowerb(arg1: *mut u8, arg2: u32);
    fn dns_domain_fromdot(arg1: *mut *mut u8, arg2: *const u8, arg3: u32) -> i32;
    fn dns_domain_length(arg1: *const u8) -> u32;
    fn dns_domain_todot_cat(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn ip4_scan(arg1: *const u8, arg2: *mut u8) -> u32;
    fn parsetype(arg1: *mut u8, arg2: *mut u8) -> i32;
    fn printpacket_cat(arg1: *mut stralloc, arg2: *mut u8, arg3: u32) -> u32;
    fn respond(arg1: *mut u8, arg2: *mut u8, arg3: *mut u8) -> i32;
    static mut response: *mut u8;
    static mut response_len: u32;
    fn response_query(arg1: *const u8, arg2: *const u8, arg3: *const u8) -> i32;
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
        (*b"tinydns-get: usage: tinydns-get type name [ip]\0").as_ptr(),
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
        (*b"tinydns-get: fatal: \0").as_ptr(),
        (*b"unable to parse: \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut strerr_sys as (*mut strerr) as (*const strerr),
    );
}

static mut ip: [u8; 4] = [0u8; 4];

static mut type_: [u8; 2] = [0u8; 2];

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
    let mut _currentBlock;
    let mut u16: u16;
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
        if ip4_scan(*argv as (*const u8), ip.as_mut_ptr()) == 0 {
            usage();
        }
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
    if response_query(
        q as (*const u8),
        type_.as_mut_ptr() as (*const u8),
        (*b"\0\x01\0").as_ptr(),
    ) == 0
    {
        oops();
    }
    let _rhs = !128i32;
    let _lhs = &mut *response.offset(3isize);
    *_lhs = (*_lhs as (i32) & _rhs) as (u8);
    let _rhs = !1i32;
    let _lhs = &mut *response.offset(2isize);
    *_lhs = (*_lhs as (i32) & _rhs) as (u8);
    let _rhs = 4i32;
    let _lhs = &mut *response.offset(2isize);
    *_lhs = (*_lhs as (i32) | _rhs) as (u8);
    case_lowerb(q, dns_domain_length(q as (*const u8)));
    if byte::diff(
        type_.as_mut_ptr(),
        2u32,
        (*b"\0\xFC\0").as_ptr() as (*mut u8),
    ) == 0
    {
        let _rhs = !15i32;
        let _lhs = &mut *response.offset(3isize);
        *_lhs = (*_lhs as (i32) & _rhs) as (u8);
        let _rhs = 4i32;
        let _lhs = &mut *response.offset(3isize);
        *_lhs = (*_lhs as (i32) | _rhs) as (u8);
        _currentBlock = 28;
    } else if respond(q, type_.as_mut_ptr(), ip.as_mut_ptr()) == 0 {
        _currentBlock = 30;
    } else {
        _currentBlock = 28;
    }
    if _currentBlock == 28 {
        if printpacket_cat(&mut out as (*mut stralloc), response, response_len) == 0 {
            oops();
        }
    }
    buffer_putflush(buffer_1, out.s as (*const u8), out.len);
    _exit(0i32);
    0
}
