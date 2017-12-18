#[no_mangle]
pub unsafe extern "C" fn scan_ulong(mut s: *const u8, mut u: *mut usize) -> u32 {
    let mut pos: u32 = 0u32;
    let mut result: usize = 0usize;
    let mut c: usize;
    'loop1: loop {
        if !({
                 c = (*s.offset(pos as (isize)) as (i32) - b'0' as (i32)) as (u8) as (usize);
                 c
             } < 10usize)
        {
            break;
        }
        result = result.wrapping_mul(10usize).wrapping_add(c);
        pos = pos.wrapping_add(1u32);
    }
    *u = result;
    pos
}
