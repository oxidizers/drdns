#[no_mangle]
pub unsafe extern fn byte_copyr(
    mut to : *mut u8, mut n : u32, mut from : *mut u8
) {
    let mut _currentBlock;
    to = to.offset(n as (isize));
    from = from.offset(n as (isize));
    'loop1: loop {
        if n == 0 {
            _currentBlock = 9;
            break;
        }
        *{
             to = to.offset(-1isize);
             to
         } = *{
                  from = from.offset(-1isize);
                  from
              };
        n = n.wrapping_sub(1u32);
        if n == 0 {
            _currentBlock = 8;
            break;
        }
        *{
             to = to.offset(-1isize);
             to
         } = *{
                  from = from.offset(-1isize);
                  from
              };
        n = n.wrapping_sub(1u32);
        if n == 0 {
            _currentBlock = 7;
            break;
        }
        *{
             to = to.offset(-1isize);
             to
         } = *{
                  from = from.offset(-1isize);
                  from
              };
        n = n.wrapping_sub(1u32);
        if n == 0 {
            _currentBlock = 6;
            break;
        }
        *{
             to = to.offset(-1isize);
             to
         } = *{
                  from = from.offset(-1isize);
                  from
              };
        n = n.wrapping_sub(1u32);
    }
    if _currentBlock == 6 {
    } else if _currentBlock == 7 {
    } else if _currentBlock == 8 {
    }
}
