extern {
    fn byte_diff(s : *mut u8, n : u32, t : *mut u8) -> i32;
    fn dd(arg1 : *const u8, arg2 : *const u8, arg3 : *mut u8) -> i32;
    static mut response : *mut u8;
    fn response_addbytes(arg1 : *const u8, arg2 : u32) -> i32;
    fn response_addname(arg1 : *const u8) -> i32;
    fn response_rfinish(arg1 : i32);
    fn response_rstart(
        arg1 : *const u8, arg2 : *const u8, arg3 : u32
    ) -> i32;
}

#[no_mangle]
pub static mut fatal : *const u8 = (*b"walldns: fatal: \0").as_ptr();

#[no_mangle]
pub static mut starting
    : *const u8
    = (*b"starting walldns\n\0").as_ptr();

#[no_mangle]
pub unsafe extern fn initialize() { }

#[no_mangle]
pub unsafe extern fn respond(
    mut q : *mut u8, mut qtype : *mut u8
) -> i32 {
    let mut flaga : i32;
    let mut flagptr : i32;
    let mut ip : [u8; 4];
    let mut j : i32;
    flaga = (byte_diff(
                 qtype,
                 2u32,
                 (*b"\0\x01\0").as_ptr() as (*mut u8)
             ) == 0) as (i32);
    flagptr = (byte_diff(
                   qtype,
                   2u32,
                   (*b"\0\x0C\0").as_ptr() as (*mut u8)
               ) == 0) as (i32);
    if byte_diff(
           qtype,
           2u32,
           (*b"\0\xFF\0").as_ptr() as (*mut u8)
       ) == 0 {
        flaga = {
                    flagptr = 1i32;
                    flagptr
                };
    }
    if flaga != 0 || flagptr != 0 {
        if dd(q as (*const u8),(*b"\0").as_ptr(),ip.as_mut_ptr()) == 4i32 {
            if flaga != 0 {
                if response_rstart(
                       q as (*const u8),
                       (*b"\0\x01\0").as_ptr(),
                       655360u32
                   ) == 0 {
                    return 0i32;
                } else if response_addbytes(
                              ip.as_mut_ptr() as (*const u8),
                              4u32
                          ) == 0 {
                    return 0i32;
                } else {
                    response_rfinish(6i32);
                }
            }
            return 1i32;
        } else {
            j = dd( q as (*const u8),
                    (*b"\x07in-addr\x04arpa\0").as_ptr(),
                    ip.as_mut_ptr()
                );
            if j >= 0i32 {
                if flaga != 0 && (j == 4i32) {
                    if response_rstart(
                           q as (*const u8),
                           (*b"\0\x01\0").as_ptr(),
                           655360u32
                       ) == 0 {
                        return 0i32;
                    } else if response_addbytes(
                                  ip.as_mut_ptr().offset(3isize) as (*const u8),
                                  1u32
                              ) == 0 {
                        return 0i32;
                    } else if response_addbytes(
                                  ip.as_mut_ptr().offset(2isize) as (*const u8),
                                  1u32
                              ) == 0 {
                        return 0i32;
                    } else if response_addbytes(
                                  ip.as_mut_ptr().offset(1isize) as (*const u8),
                                  1u32
                              ) == 0 {
                        return 0i32;
                    } else if response_addbytes(
                                  ip.as_mut_ptr().offset(0isize) as (*const u8),
                                  1u32
                              ) == 0 {
                        return 0i32;
                    } else {
                        response_rfinish(6i32);
                    }
                }
                if flagptr != 0 {
                    if response_rstart(
                           q as (*const u8),
                           (*b"\0\x0C\0").as_ptr(),
                           655360u32
                       ) == 0 {
                        return 0i32;
                    } else if response_addname(q as (*const u8)) == 0 {
                        return 0i32;
                    } else {
                        response_rfinish(6i32);
                    }
                }
                return 1i32;
            }
        }
    }
    let _rhs = !4i32;
    let _lhs = &mut *response.offset(2isize);
    *_lhs = (*_lhs as (i32) & _rhs) as (u8);
    let _rhs = !15i32;
    let _lhs = &mut *response.offset(3isize);
    *_lhs = (*_lhs as (i32) & _rhs) as (u8);
    let _rhs = 5i32;
    let _lhs = &mut *response.offset(3isize);
    *_lhs = (*_lhs as (i32) | _rhs) as (u8);
    1i32
}
