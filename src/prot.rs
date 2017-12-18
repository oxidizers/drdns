extern "C" {
    fn setgid(arg1: u32) -> i32;
    fn setgroups(arg1: i32, arg2: *const u32) -> i32;
    fn setuid(arg1: u32) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn prot_gid(mut gid: i32) -> i32 {
    if setgroups(1i32, &mut gid as (*mut i32) as (*const u32)) == -1i32 {
        -1i32
    } else {
        setgid(gid as (u32))
    }
}

#[no_mangle]
pub unsafe extern "C" fn prot_uid(mut uid: i32) -> i32 {
    setuid(uid as (u32))
}
