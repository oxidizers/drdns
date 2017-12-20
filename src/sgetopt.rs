use buffer::Buffer;
use buffer_2::BUFFER_2;

extern "C" {
    fn subgetopt(arg1: i32, arg2: *mut *mut u8, arg3: *const u8) -> i32;
    static mut subgetoptind: i32;
    static mut subgetoptproblem: i32;
}

#[no_mangle]
pub static mut sgetopterr: i32 = 1i32;

#[no_mangle]
pub static mut sgetoptprogname: *const u8 = 0i32 as (*const u8);

#[no_mangle]
pub unsafe extern "C" fn sgetoptmine(
    mut argc: i32,
    mut argv: *mut *mut u8,
    mut opts: *const u8,
) -> i32 {
    let mut c: i32;
    let mut s: *const u8;
    if sgetoptprogname.is_null() {
        sgetoptprogname = *argv as (*const u8);
        if sgetoptprogname.is_null() {
            sgetoptprogname = (*b"\0").as_ptr();
        }
        s = sgetoptprogname;
        'loop4: loop {
            if *s == 0 {
                break;
            }
            if *s as (i32) == b'/' as (i32) {
                sgetoptprogname = s.offset(1isize);
            }
            s = s.offset(1isize);
        }
    }
    c = subgetopt(argc, argv, opts);
    if sgetopterr != 0 {
        if c == b'?' as (i32) {
            let mut chp: [u8; 2];
            chp[0usize] = subgetoptproblem as (u8);
            chp[1usize] = b'\n';
            Buffer::puts(BUFFER_2.as_mut_ptr(), sgetoptprogname);
            if !(*argv.offset(subgetoptind as (isize))).is_null() && (subgetoptind < argc) {
                Buffer::puts(BUFFER_2.as_mut_ptr(), (*b": illegal option -- \0").as_ptr());
            } else {
                Buffer::puts(BUFFER_2.as_mut_ptr(), (*b": option requires an argument -- \0").as_ptr());
            }
            Buffer::put(BUFFER_2.as_mut_ptr(), chp.as_mut_ptr() as (*const u8), 2u32);
            Buffer::flush(BUFFER_2.as_mut_ptr());
        }
    }
    c
}
