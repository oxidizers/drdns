extern {
    fn dns_domain_copy(arg1 : *mut *mut u8, arg2 : *const u8) -> i32;
    static mut errno : i32;
    static mut error_proto : i32;
}

#[no_mangle]
pub unsafe extern fn dns_packet_copy(
    mut buf : *const u8,
    mut len : u32,
    mut pos : u32,
    mut out : *mut u8,
    mut outlen : u32
) -> u32 {
    let mut _currentBlock;
    'loop0: loop {
        if outlen == 0 {
            _currentBlock = 1;
            break;
        }
        if pos >= len {
            _currentBlock = 4;
            break;
        }
        *out = *buf.offset(
                    {
                        let _old = pos;
                        pos = pos.wrapping_add(1u32);
                        _old
                    } as (isize)
                );
        out = out.offset(1isize);
        outlen = outlen.wrapping_sub(1u32);
    }
    if _currentBlock == 1 {
        pos
    } else {
        errno = error_proto;
        0u32
    }
}

#[no_mangle]
pub unsafe extern fn dns_packet_skipname(
    mut buf : *const u8, mut len : u32, mut pos : u32
) -> u32 {
    let mut _currentBlock;
    let mut ch : u8;
    'loop1: loop {
        if pos >= len {
            _currentBlock = 8;
            break;
        }
        ch = *buf.offset(
                  {
                      let _old = pos;
                      pos = pos.wrapping_add(1u32);
                      _old
                  } as (isize)
              );
        if ch as (i32) >= 192i32 {
            _currentBlock = 7;
            break;
        }
        if ch as (i32) >= 64i32 {
            _currentBlock = 8;
            break;
        }
        if ch == 0 {
            _currentBlock = 6;
            break;
        }
        pos = pos.wrapping_add(ch as (u32));
    }
    if _currentBlock == 6 {
        pos
    } else if _currentBlock == 7 {
        pos.wrapping_add(1u32)
    } else {
        errno = error_proto;
        0u32
    }
}

#[no_mangle]
pub unsafe extern fn dns_packet_getname(
    mut buf : *const u8,
    mut len : u32,
    mut pos : u32,
    mut d : *mut *mut u8
) -> u32 {
    let mut _currentBlock;
    let mut loopvar : u32 = 0u32;
    let mut state : u32 = 0u32;
    let mut firstcompress : u32 = 0u32;
    let mut where_ : u32;
    let mut ch : u8;
    let mut name : [u8; 255];
    let mut namelen : u32 = 0u32;
    'loop1: loop {
        if pos >= len {
            _currentBlock = 21;
            break;
        }
        ch = *buf.offset(
                  {
                      let _old = pos;
                      pos = pos.wrapping_add(1u32);
                      _old
                  } as (isize)
              );
        if {
               loopvar = loopvar.wrapping_add(1u32);
               loopvar
           } >= 1000u32 {
            _currentBlock = 21;
            break;
        }
        if state != 0 {
            if namelen.wrapping_add(
                   1u32
               ) as (usize) > ::std::mem::size_of::<[u8; 255]>() {
                _currentBlock = 21;
                break;
            }
            name[
                {
                    let _old = namelen;
                    namelen = namelen.wrapping_add(1u32);
                    _old
                } as (usize)
            ] = ch;
            state = state.wrapping_sub(1u32);
        } else {
            'loop4: loop {
                if !(ch as (i32) >= 192i32) {
                    break;
                }
                where_ = ch as (u32);
                where_ = where_.wrapping_sub(192u32);
                where_ = where_ << 8i32;
                if pos >= len {
                    _currentBlock = 21;
                    break 'loop1;
                }
                ch = *buf.offset(
                          {
                              let _old = pos;
                              pos = pos.wrapping_add(1u32);
                              _old
                          } as (isize)
                      );
                if firstcompress == 0 {
                    firstcompress = pos;
                }
                pos = where_.wrapping_add(ch as (u32));
                if pos >= len {
                    _currentBlock = 21;
                    break 'loop1;
                }
                ch = *buf.offset(
                          {
                              let _old = pos;
                              pos = pos.wrapping_add(1u32);
                              _old
                          } as (isize)
                      );
                if {
                       loopvar = loopvar.wrapping_add(1u32);
                       loopvar
                   } >= 1000u32 {
                    _currentBlock = 21;
                    break 'loop1;
                }
            }
            if ch as (i32) >= 64i32 {
                _currentBlock = 21;
                break;
            }
            if namelen.wrapping_add(
                   1u32
               ) as (usize) > ::std::mem::size_of::<[u8; 255]>() {
                _currentBlock = 21;
                break;
            }
            name[
                {
                    let _old = namelen;
                    namelen = namelen.wrapping_add(1u32);
                    _old
                } as (usize)
            ] = ch;
            if ch == 0 {
                _currentBlock = 9;
                break;
            }
            state = ch as (u32);
        }
    }
    if _currentBlock == 9 {
        (if dns_domain_copy(d,name.as_mut_ptr() as (*const u8)) == 0 {
             0u32
         } else if firstcompress != 0 {
             firstcompress
         } else {
             pos
         })
    } else {
        errno = error_proto;
        0u32
    }
}
