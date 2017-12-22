use ulong;

#[no_mangle]
pub unsafe extern "C" fn ip4_fmt(mut s: *mut u8, mut ip: *const u8) -> u32 {
    let mut len: u32;
    let mut i: u32;
    len = 0u32;
    i = ulong::fmt(s, *ip.offset(0isize) as (usize));
    len = len.wrapping_add(i);
    if !s.is_null() {
        s = s.offset(i as (isize));
    }
    if !s.is_null() {
        *{
            let _old = s;
            s = s.offset(1isize);
            _old
        } = b'.';
    }
    len = len.wrapping_add(1u32);
    i = ulong::fmt(s, *ip.offset(1isize) as (usize));
    len = len.wrapping_add(i);
    if !s.is_null() {
        s = s.offset(i as (isize));
    }
    if !s.is_null() {
        *{
            let _old = s;
            s = s.offset(1isize);
            _old
        } = b'.';
    }
    len = len.wrapping_add(1u32);
    i = ulong::fmt(s, *ip.offset(2isize) as (usize));
    len = len.wrapping_add(i);
    if !s.is_null() {
        s = s.offset(i as (isize));
    }
    if !s.is_null() {
        *{
            let _old = s;
            s = s.offset(1isize);
            _old
        } = b'.';
    }
    len = len.wrapping_add(1u32);
    i = ulong::fmt(s, *ip.offset(3isize) as (usize));
    len = len.wrapping_add(i);
    if !s.is_null() {
        s = s.offset(i as (isize));
    }
    len
}

#[no_mangle]
pub unsafe extern "C" fn ip4_scan(mut s: *const u8, mut ip: *mut u8) -> u32 {
    let mut i: u32;
    let mut len: u32;
    let mut u: usize;
    len = 0u32;
    i = ulong::scan(s, &mut u as (*mut usize));
    if i == 0 {
        0u32
    } else {
        *ip.offset(0isize) = u as (u8);
        s = s.offset(i as (isize));
        len = len.wrapping_add(i);
        (if *s as (i32) != b'.' as (i32) {
             0u32
         } else {
             s = s.offset(1isize);
             len = len.wrapping_add(1u32);
             i = ulong::scan(s, &mut u as (*mut usize));
             (if i == 0 {
                  0u32
              } else {
                  *ip.offset(1isize) = u as (u8);
                  s = s.offset(i as (isize));
                  len = len.wrapping_add(i);
                  (if *s as (i32) != b'.' as (i32) {
                       0u32
                   } else {
                       s = s.offset(1isize);
                       len = len.wrapping_add(1u32);
                       i = ulong::scan(s, &mut u as (*mut usize));
                       (if i == 0 {
                            0u32
                        } else {
                            *ip.offset(2isize) = u as (u8);
                            s = s.offset(i as (isize));
                            len = len.wrapping_add(i);
                            (if *s as (i32) != b'.' as (i32) {
                                 0u32
                             } else {
                                 s = s.offset(1isize);
                                 len = len.wrapping_add(1u32);
                                 i = ulong::scan(s, &mut u as (*mut usize));
                                 (if i == 0 {
                                      0u32
                                  } else {
                                      *ip.offset(3isize) = u as (u8);
                                      s = s.offset(i as (isize));
                                      len = len.wrapping_add(i);
                                      len
                                  })
                             })
                        })
                   })
              })
         })
    }
}
