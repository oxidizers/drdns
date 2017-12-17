extern {
    fn _exit(arg1 : i32);
    fn buffer_copy(arg1 : *mut buffer, arg2 : *mut buffer) -> i32;
    fn buffer_flush(arg1 : *mut buffer) -> i32;
    fn buffer_init(
        arg1 : *mut buffer,
        arg2 : unsafe extern fn() -> i32,
        arg3 : i32,
        arg4 : *mut u8,
        arg5 : u32
    );
    fn buffer_put(
        arg1 : *mut buffer, arg2 : *const u8, arg3 : u32
    ) -> i32;
    fn buffer_unixread(arg1 : i32, arg2 : *mut u8, arg3 : u32) -> i32;
    fn buffer_unixwrite(
        arg1 : i32, arg2 : *const u8, arg3 : u32
    ) -> i32;
    fn chdir(arg1 : *const u8) -> i32;
    fn chmod(arg1 : *const u8, arg2 : u16) -> i32;
    fn chown(arg1 : *const u8, arg2 : u32, arg3 : u32) -> i32;
    fn close(arg1 : i32) -> i32;
    static mut errno : i32;
    static mut error_exist : i32;
    fn fchdir(arg1 : i32) -> i32;
    fn fsync(arg1 : i32) -> i32;
    fn hier();
    fn mkdir(arg1 : *const u8, arg2 : u16) -> i32;
    fn open_read(arg1 : *const u8) -> i32;
    fn open_trunc(arg1 : *const u8) -> i32;
    fn strerr_die(
        arg1 : i32,
        arg2 : *const u8,
        arg3 : *const u8,
        arg4 : *const u8,
        arg5 : *const u8,
        arg6 : *const u8,
        arg7 : *const u8,
        arg8 : *const strerr
    );
    static mut strerr_sys : strerr;
    fn umask(arg1 : u16) -> u16;
}

#[no_mangle]
pub static mut fdsourcedir : i32 = -1i32;

#[derive(Copy)]
#[repr(C)]
pub struct strerr {
    pub who : *mut strerr,
    pub x : *const u8,
    pub y : *const u8,
    pub z : *const u8,
}

impl Clone for strerr {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn h(
    mut home : *mut u8, mut uid : i32, mut gid : i32, mut mode : i32
) {
    if mkdir(home as (*const u8),0o700u16) == -1i32 {
        if errno != error_exist {
            strerr_die(
                111i32,
                (*b"install: fatal: \0").as_ptr(),
                (*b"unable to mkdir \0").as_ptr(),
                home as (*const u8),
                (*b": \0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                &mut strerr_sys as (*mut strerr) as (*const strerr)
            );
        }
    }
    if chown(home as (*const u8),uid as (u32),gid as (u32)) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to chown \0").as_ptr(),
            home as (*const u8),
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if chmod(home as (*const u8),mode as (u16)) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to chmod \0").as_ptr(),
            home as (*const u8),
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
}

#[no_mangle]
pub unsafe extern fn d(
    mut home : *mut u8,
    mut subdir : *mut u8,
    mut uid : i32,
    mut gid : i32,
    mut mode : i32
) {
    if chdir(home as (*const u8)) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to switch to \0").as_ptr(),
            home as (*const u8),
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if mkdir(subdir as (*const u8),0o700u16) == -1i32 {
        if errno != error_exist {
            strerr_die(
                111i32,
                (*b"install: fatal: \0").as_ptr(),
                (*b"unable to mkdir \0").as_ptr(),
                home as (*const u8),
                (*b"/\0").as_ptr(),
                subdir as (*const u8),
                (*b": \0").as_ptr(),
                &mut strerr_sys as (*mut strerr) as (*const strerr)
            );
        }
    }
    if chown(
           subdir as (*const u8),
           uid as (u32),
           gid as (u32)
       ) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to chown \0").as_ptr(),
            home as (*const u8),
            (*b"/\0").as_ptr(),
            subdir as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if chmod(subdir as (*const u8),mode as (u16)) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to chmod \0").as_ptr(),
            home as (*const u8),
            (*b"/\0").as_ptr(),
            subdir as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
}

#[no_mangle]
pub static mut inbuf : [u8; 8192] = [0u8; 8192];

#[no_mangle]
pub static mut outbuf : [u8; 8192] = [0u8; 8192];

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
pub static mut ssin
    : buffer
    = buffer {
          x: 0 as (*mut u8),
          p: 0u32,
          n: 0u32,
          fd: 0i32,
          op: 0 as (unsafe extern fn() -> i32)
      };

#[no_mangle]
pub static mut ssout
    : buffer
    = buffer {
          x: 0 as (*mut u8),
          p: 0u32,
          n: 0u32,
          fd: 0i32,
          op: 0 as (unsafe extern fn() -> i32)
      };

#[no_mangle]
pub unsafe extern fn c(
    mut home : *mut u8,
    mut subdir : *mut u8,
    mut file : *mut u8,
    mut uid : i32,
    mut gid : i32,
    mut mode : i32
) {
    let mut _currentBlock;
    let mut fdin : i32;
    let mut fdout : i32;
    if fchdir(fdsourcedir) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to switch back to source directory: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    fdin = open_read(file as (*const u8));
    if fdin == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to read \0").as_ptr(),
            file as (*const u8),
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    buffer_init(
        &mut ssin as (*mut buffer),
        buffer_unixread as (unsafe extern fn() -> i32),
        fdin,
        inbuf.as_mut_ptr(),
        ::std::mem::size_of::<[u8; 8192]>() as (u32)
    );
    if chdir(home as (*const u8)) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to switch to \0").as_ptr(),
            home as (*const u8),
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if chdir(subdir as (*const u8)) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to switch to \0").as_ptr(),
            home as (*const u8),
            (*b"/\0").as_ptr(),
            subdir as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    fdout = open_trunc(file as (*const u8));
    if fdout == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to write .../\0").as_ptr(),
            subdir as (*const u8),
            (*b"/\0").as_ptr(),
            file as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    buffer_init(
        &mut ssout as (*mut buffer),
        buffer_unixwrite as (unsafe extern fn() -> i32),
        fdout,
        outbuf.as_mut_ptr(),
        ::std::mem::size_of::<[u8; 8192]>() as (u32)
    );
    let switch1
        = buffer_copy(
              &mut ssout as (*mut buffer),
              &mut ssin as (*mut buffer)
          );
    if switch1 == -3i32 {
        _currentBlock = 14;
    } else if switch1 == -2i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to read \0").as_ptr(),
            file as (*const u8),
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
        _currentBlock = 14;
    } else {
        _currentBlock = 15;
    }
    if _currentBlock == 14 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to write .../\0").as_ptr(),
            subdir as (*const u8),
            (*b"/\0").as_ptr(),
            file as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    close(fdin);
    if buffer_flush(&mut ssout as (*mut buffer)) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to write .../\0").as_ptr(),
            subdir as (*const u8),
            (*b"/\0").as_ptr(),
            file as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if fsync(fdout) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to write .../\0").as_ptr(),
            subdir as (*const u8),
            (*b"/\0").as_ptr(),
            file as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if close(fdout) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to write .../\0").as_ptr(),
            subdir as (*const u8),
            (*b"/\0").as_ptr(),
            file as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if chown(file as (*const u8),uid as (u32),gid as (u32)) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to chown .../\0").as_ptr(),
            subdir as (*const u8),
            (*b"/\0").as_ptr(),
            file as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if chmod(file as (*const u8),mode as (u16)) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to chmod .../\0").as_ptr(),
            subdir as (*const u8),
            (*b"/\0").as_ptr(),
            file as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
}

#[no_mangle]
pub unsafe extern fn z(
    mut home : *mut u8,
    mut subdir : *mut u8,
    mut file : *mut u8,
    mut len : i32,
    mut uid : i32,
    mut gid : i32,
    mut mode : i32
) {
    let mut fdout : i32;
    if chdir(home as (*const u8)) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to switch to \0").as_ptr(),
            home as (*const u8),
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if chdir(subdir as (*const u8)) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to switch to \0").as_ptr(),
            home as (*const u8),
            (*b"/\0").as_ptr(),
            subdir as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    fdout = open_trunc(file as (*const u8));
    if fdout == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to write .../\0").as_ptr(),
            subdir as (*const u8),
            (*b"/\0").as_ptr(),
            file as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    buffer_init(
        &mut ssout as (*mut buffer),
        buffer_unixwrite as (unsafe extern fn() -> i32),
        fdout,
        outbuf.as_mut_ptr(),
        ::std::mem::size_of::<[u8; 8192]>() as (u32)
    );
    'loop7: loop {
        if !({
                 let _old = len;
                 len = len - 1;
                 _old
             } > 0i32) {
            break;
        }
        if !(buffer_put(
                 &mut ssout as (*mut buffer),
                 (*b"\0").as_ptr(),
                 1u32
             ) == -1i32) {
            continue;
        }
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to write .../\0").as_ptr(),
            subdir as (*const u8),
            (*b"/\0").as_ptr(),
            file as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if buffer_flush(&mut ssout as (*mut buffer)) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to write .../\0").as_ptr(),
            subdir as (*const u8),
            (*b"/\0").as_ptr(),
            file as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if fsync(fdout) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to write .../\0").as_ptr(),
            subdir as (*const u8),
            (*b"/\0").as_ptr(),
            file as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if close(fdout) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to write .../\0").as_ptr(),
            subdir as (*const u8),
            (*b"/\0").as_ptr(),
            file as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if chown(file as (*const u8),uid as (u32),gid as (u32)) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to chown .../\0").as_ptr(),
            subdir as (*const u8),
            (*b"/\0").as_ptr(),
            file as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if chmod(file as (*const u8),mode as (u16)) == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to chmod .../\0").as_ptr(),
            subdir as (*const u8),
            (*b"/\0").as_ptr(),
            file as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
}

fn main() {
    let ret = unsafe { _c_main() };
    ::std::process::exit(ret);
}

#[no_mangle]
pub unsafe extern fn _c_main() -> i32 {
    fdsourcedir = open_read((*b".\0").as_ptr());
    if fdsourcedir == -1i32 {
        strerr_die(
            111i32,
            (*b"install: fatal: \0").as_ptr(),
            (*b"unable to open current directory: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    umask(0o77u16);
    hier();
    _exit(0i32);
    0
}
