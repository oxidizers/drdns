#[no_mangle]
pub unsafe extern "C" fn str_len(mut s: *const u8) -> u32 {
    let mut _currentBlock;
    let mut t: *const u8;
    t = s;
    'loop1: loop {
        if *t == 0 {
            _currentBlock = 9;
            break;
        }
        t = t.offset(1isize);
        if *t == 0 {
            _currentBlock = 8;
            break;
        }
        t = t.offset(1isize);
        if *t == 0 {
            _currentBlock = 7;
            break;
        }
        t = t.offset(1isize);
        if *t == 0 {
            _currentBlock = 6;
            break;
        }
        t = t.offset(1isize);
    }
    if _currentBlock == 6 {
        ((t as (isize)).wrapping_sub(s as (isize)) / ::std::mem::size_of::<u8>() as (isize)) as
            (u32)
    } else if _currentBlock == 7 {
        ((t as (isize)).wrapping_sub(s as (isize)) / ::std::mem::size_of::<u8>() as (isize)) as
            (u32)
    } else if _currentBlock == 8 {
        ((t as (isize)).wrapping_sub(s as (isize)) / ::std::mem::size_of::<u8>() as (isize)) as
            (u32)
    } else {
        ((t as (isize)).wrapping_sub(s as (isize)) / ::std::mem::size_of::<u8>() as (isize)) as
            (u32)
    }
}
