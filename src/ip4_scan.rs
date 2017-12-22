use ulong;

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
