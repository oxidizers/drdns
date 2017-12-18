#[no_mangle]
pub unsafe extern "C" fn case_diffb(mut s: *const u8, mut len: u32, mut t: *const u8) -> i32 {
    let mut _currentBlock;
    let mut x: u8;
    let mut y: u8;
    'loop1: loop {
        if !(len > 0u32) {
            _currentBlock = 2;
            break;
        }
        len = len.wrapping_sub(1u32);
        x = (*{
                 let _old = s;
                 s = s.offset(1isize);
                 _old
             } as (i32) - b'A' as (i32)) as (u8);
        if x as (i32) <= b'Z' as (i32) - b'A' as (i32) {
            x = (x as (i32) + b'a' as (i32)) as (u8);
        } else {
            x = (x as (i32) + b'A' as (i32)) as (u8);
        }
        y = (*{
                 let _old = t;
                 t = t.offset(1isize);
                 _old
             } as (i32) - b'A' as (i32)) as (u8);
        if y as (i32) <= b'Z' as (i32) - b'A' as (i32) {
            y = (y as (i32) + b'a' as (i32)) as (u8);
        } else {
            y = (y as (i32) + b'A' as (i32)) as (u8);
        }
        if x as (i32) != y as (i32) {
            _currentBlock = 10;
            break;
        }
    }
    if _currentBlock == 2 {
        0i32
    } else {
        x as (u32) as (i32) - y as (u32) as (i32)
    }
}
