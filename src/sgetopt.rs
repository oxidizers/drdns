extern {
    static mut buffer_2 : *mut buffer;
    fn buffer_flush(arg1 : *mut buffer) -> i32;
    fn buffer_put(
        arg1 : *mut buffer, arg2 : *const u8, arg3 : u32
    ) -> i32;
    fn buffer_puts(arg1 : *mut buffer, arg2 : *const u8) -> i32;
    fn subgetopt(
        arg1 : i32, arg2 : *mut *mut u8, arg3 : *const u8
    ) -> i32;
    static mut subgetoptind : i32;
    static mut subgetoptproblem : i32;
}

#[no_mangle]
pub static mut sgetopterr : i32 = 1i32;

#[no_mangle]
pub static mut sgetoptprogname : *const u8 = 0i32 as (*const u8);

#[derive(Copy)]
#[repr(C)]
pub struct buffer {
    pub x : *mut u8,
    pub p : u32,
    pub n : u32,
    pub fd : i32,
    pub op : unsafe extern fn() -> i32,
}

impl Clone for buffer {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn sgetoptmine(
    mut argc : i32, mut argv : *mut *mut u8, mut opts : *const u8
) -> i32 {
    let mut c : i32;
    let mut s : *const u8;
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
    c = subgetopt(argc,argv,opts);
    if sgetopterr != 0 {
        if c == b'?' as (i32) {
            let mut chp : [u8; 2];
            chp[0usize] = subgetoptproblem as (u8);
            chp[1usize] = b'\n';
            buffer_puts(buffer_2,sgetoptprogname);
            if !(*argv.offset(subgetoptind as (isize))).is_null(
                ) && (subgetoptind < argc) {
                buffer_puts(buffer_2,(*b": illegal option -- \0").as_ptr());
            } else {
                buffer_puts(
                    buffer_2,
                    (*b": option requires an argument -- \0").as_ptr()
                );
            }
            buffer_put(buffer_2,chp.as_mut_ptr() as (*const u8),2u32);
            buffer_flush(buffer_2);
        }
    }
    c
}
