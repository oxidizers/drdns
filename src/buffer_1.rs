extern {
    fn buffer_unixwrite(
        arg1 : i32, arg2 : *const u8, arg3 : u32
    ) -> i32;
}

#[no_mangle]
pub static mut buffer_1_space : [u8; 8192] = [0u8; 8192];

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

static mut it
    : buffer
    = buffer {
          x: buffer_1_space.as_mut_ptr(),
          p: 0u32,
          n: ::std::mem::size_of::<[u8; 8192]>() as (u32),
          fd: 1i32,
          op: buffer_unixwrite as (unsafe extern fn() -> i32)
      };

#[no_mangle]
pub static mut buffer_1 : *mut buffer = &mut it as (*mut buffer);
