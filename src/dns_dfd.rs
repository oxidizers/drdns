use byte;
use libc;

extern "C" {
    static mut errno: i32;
}

#[no_mangle]
pub unsafe extern "C" fn dns_domain_fromdot(
    mut out: *mut *mut u8,
    mut buf: *const u8,
    mut n: u32,
) -> i32 {
    let mut _currentBlock;
    let mut label: [u8; 63];
    let mut labellen: u32 = 0u32;
    let mut name: [u8; 255];
    let mut namelen: u32 = 0u32;
    let mut ch: u8;
    let mut x: *mut u8;
    errno = libc::EPROTO;
    'loop1: loop {
        if n == 0 {
            _currentBlock = 16;
            break;
        }
        ch = *{
            let _old = buf;
            buf = buf.offset(1isize);
            _old
        };
        n = n.wrapping_sub(1u32);
        if ch as (i32) == b'.' as (i32) {
            if labellen == 0 {
                continue;
            }
            if namelen.wrapping_add(labellen).wrapping_add(1u32) as (usize) >
                ::std::mem::size_of::<[u8; 255]>()
            {
                _currentBlock = 15;
                break;
            }
            name[{
                     let _old = namelen;
                     namelen = namelen.wrapping_add(1u32);
                     _old
                 } as (usize)] = labellen as (u8);
            byte::copy(
                name.as_mut_ptr().offset(namelen as (isize)),
                labellen,
                label.as_mut_ptr(),
            );
            namelen = namelen.wrapping_add(labellen);
            labellen = 0u32;
        } else {
            if ch as (i32) == b'\\' as (i32) {
                if n == 0 {
                    _currentBlock = 16;
                    break;
                }
                ch = *{
                    let _old = buf;
                    buf = buf.offset(1isize);
                    _old
                };
                n = n.wrapping_sub(1u32);
                if ch as (i32) >= b'0' as (i32) && (ch as (i32) <= b'7' as (i32)) {
                    ch = (ch as (i32) - b'0' as (i32)) as (u8);
                    if n != 0 && (*buf as (i32) >= b'0' as (i32)) &&
                        (*buf as (i32) <= b'7' as (i32))
                    {
                        ch = (ch as (i32) << 3i32) as (u8);
                        ch = (ch as (i32) + (*buf as (i32) - b'0' as (i32))) as (u8);
                        buf = buf.offset(1isize);
                        n = n.wrapping_sub(1u32);
                        if n != 0 && (*buf as (i32) >= b'0' as (i32)) &&
                            (*buf as (i32) <= b'7' as (i32))
                        {
                            ch = (ch as (i32) << 3i32) as (u8);
                            ch = (ch as (i32) + (*buf as (i32) - b'0' as (i32))) as (u8);
                            buf = buf.offset(1isize);
                            n = n.wrapping_sub(1u32);
                        }
                    }
                }
            }
            if labellen as (usize) >= ::std::mem::size_of::<[u8; 63]>() {
                _currentBlock = 11;
                break;
            }
            label[{
                      let _old = labellen;
                      labellen = labellen.wrapping_add(1u32);
                      _old
                  } as (usize)] = ch;
        }
    }
    if _currentBlock == 11 {
        0i32
    } else if _currentBlock == 15 {
        0i32
    } else {
        if labellen != 0 {
            if namelen.wrapping_add(labellen).wrapping_add(1u32) as (usize) >
                ::std::mem::size_of::<[u8; 255]>()
            {
                return 0i32;
            } else {
                name[{
                         let _old = namelen;
                         namelen = namelen.wrapping_add(1u32);
                         _old
                     } as (usize)] = labellen as (u8);
                byte::copy(
                    name.as_mut_ptr().offset(namelen as (isize)),
                    labellen,
                    label.as_mut_ptr(),
                );
                namelen = namelen.wrapping_add(labellen);
                labellen = 0u32;
            }
        }
        (if namelen.wrapping_add(1u32) as (usize) > ::std::mem::size_of::<[u8; 255]>() {
             0i32
         } else {
             name[{
                      let _old = namelen;
                      namelen = namelen.wrapping_add(1u32);
                      _old
                  } as (usize)] = 0u8;
             x = alloc::alloc(namelen);
             (if x.is_null() {
                  0i32
              } else {
                  byte::copy(x, namelen, name.as_mut_ptr());
                  if !(*out).is_null() {
                      alloc::alloc_free(*out);
                  }
                  *out = x;
                  1i32
              })
         })
    }
}
