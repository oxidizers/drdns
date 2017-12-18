#[no_mangle]
pub static mut error_intr: i32 = 4i32;

#[no_mangle]
pub static mut error_nomem: i32 = 12i32;

#[no_mangle]
pub static mut error_noent: i32 = 2i32;

#[no_mangle]
pub static mut error_txtbsy: i32 = 26i32;

#[no_mangle]
pub static mut error_io: i32 = 5i32;

#[no_mangle]
pub static mut error_exist: i32 = 17i32;

#[no_mangle]
pub static mut error_timeout: i32 = 60i32;

#[no_mangle]
pub static mut error_inprogress: i32 = 36i32;

#[no_mangle]
pub static mut error_wouldblock: i32 = 35i32;

#[no_mangle]
pub static mut error_again: i32 = 35i32;

#[no_mangle]
pub static mut error_pipe: i32 = 32i32;

#[no_mangle]
pub static mut error_perm: i32 = 1i32;

#[no_mangle]
pub static mut error_acces: i32 = 13i32;

#[no_mangle]
pub static mut error_nodevice: i32 = 6i32;

#[no_mangle]
pub static mut error_proto: i32 = 100i32;

#[no_mangle]
pub static mut error_isdir: i32 = 21i32;

#[no_mangle]
pub static mut error_connrefused: i32 = 61i32;
