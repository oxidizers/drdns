use dns;

#[no_mangle]
pub static mut fatal: *const u8 = (*b"tinydns: fatal: \0").as_ptr();

#[no_mangle]
pub static mut starting: *const u8 = (*b"starting tinydns\n\0").as_ptr();

static mut seed: [u8; 128] = [0u8; 128];

#[no_mangle]
pub unsafe extern "C" fn initialize() {
    dns::random::init(seed.as_mut_ptr() as (*const u8));
}
