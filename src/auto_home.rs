#[no_mangle]
pub static mut auto_home: *const u8 = (*b"/usr/local\0").as_ptr();
