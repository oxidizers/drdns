#[no_mangle]
pub unsafe extern "C" fn fmt_ulong(mut s: *mut u8, mut u: usize) -> u32 {
    let mut len: u32;
    let mut q: usize;
    len = 1u32;
    q = u;
    'loop1: loop {
        if !(q > 9usize) {
            break;
        }
        len = len.wrapping_add(1u32);
        q = q.wrapping_div(10usize);
    }
    if !s.is_null() {
        s = s.offset(len as (isize));
        'loop4: loop {
            *{
                s = s.offset(-1isize);
                s
            } = (b'0' as (usize)).wrapping_add(u.wrapping_rem(10usize)) as (u8);
            u = u.wrapping_div(10usize);
            if u == 0 {
                break;
            }
        }
    }
    len
}
