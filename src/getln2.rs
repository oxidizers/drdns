use buffer::Buffer;
use byte;
use stralloc::StrAlloc;

#[no_mangle]
pub unsafe extern "C" fn getln2(
    mut ss: *mut Buffer,
    mut sa: *mut StrAlloc,
    mut cont: *mut *mut u8,
    mut clen: *mut u32,
    mut sep: i32,
) -> i32 {
    let mut _currentBlock;
    let mut x: *mut u8;
    let mut i: u32;
    let mut n: i32;
    if StrAlloc::ready(sa, 0u32) == 0 {
        -1i32
    } else {
        (*sa).len = 0u32;
        'loop2: loop {
            n = Buffer::feed(ss);
            if n < 0i32 {
                _currentBlock = 10;
                break;
            }
            if n == 0i32 {
                _currentBlock = 9;
                break;
            }
            x = (*ss).x.offset((*ss).n as (isize));
            i = byte::chr(x, n as (u32), sep);
            if i < n as (u32) {
                _currentBlock = 8;
                break;
            }
            if StrAlloc::readyplus(sa, n as (u32)) == 0 {
                _currentBlock = 7;
                break;
            }
            i = (*sa).len;
            (*sa).len = i.wrapping_add(Buffer::get(ss, (*sa).s.offset(i as (isize)), n as (u32)) as
                (u32));
        }
        (if _currentBlock == 7 {
             -1i32
         } else if _currentBlock == 8 {
             (*ss).p = (*ss).p.wrapping_sub({
                *clen = i.wrapping_add(1u32);
                *clen
            });
             (*ss).n = (*ss).n.wrapping_add({
                *clen = i.wrapping_add(1u32);
                *clen
            });
             *cont = x;
             0i32
         } else if _currentBlock == 9 {
             *clen = 0u32;
             0i32
         } else {
             -1i32
         })
    }
}
