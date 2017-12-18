enum _telldir {
}

#[derive(Copy)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: isize,
    pub __opaque: [u8; 56],
}

impl Clone for _opaque_pthread_mutex_t {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
    pub __dd_fd: i32,
    pub __dd_loc: isize,
    pub __dd_size: isize,
    pub __dd_buf: *mut u8,
    pub __dd_len: i32,
    pub __dd_seek: isize,
    pub __padding: isize,
    pub __dd_flags: i32,
    pub __dd_lock: _opaque_pthread_mutex_t,
    pub __dd_td: *mut _telldir,
}

impl Clone for Struct1 {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct dirent {
    pub d_ino: usize,
    pub d_seekoff: usize,
    pub d_reclen: u16,
    pub d_namlen: u16,
    pub d_type: u8,
    pub d_name: [u8; 1024],
}

impl Clone for dirent {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn foo() {
    let mut dir: *mut Struct1;
    let mut d: *mut dirent;
}
