#[no_mangle]
pub unsafe extern "C" fn byte_copy(mut to: *mut u8, mut n: u32, mut from: *mut u8) {
    let mut _currentBlock;
    'loop0: loop {
        if n == 0 {
            _currentBlock = 8;
            break;
        }
        *{
            let _old = to;
            to = to.offset(1isize);
            _old
        } = *{
            let _old = from;
            from = from.offset(1isize);
            _old
        };
        n = n.wrapping_sub(1u32);
        if n == 0 {
            _currentBlock = 7;
            break;
        }
        *{
            let _old = to;
            to = to.offset(1isize);
            _old
        } = *{
            let _old = from;
            from = from.offset(1isize);
            _old
        };
        n = n.wrapping_sub(1u32);
        if n == 0 {
            _currentBlock = 6;
            break;
        }
        *{
            let _old = to;
            to = to.offset(1isize);
            _old
        } = *{
            let _old = from;
            from = from.offset(1isize);
            _old
        };
        n = n.wrapping_sub(1u32);
        if n == 0 {
            _currentBlock = 5;
            break;
        }
        *{
            let _old = to;
            to = to.offset(1isize);
            _old
        } = *{
            let _old = from;
            from = from.offset(1isize);
            _old
        };
        n = n.wrapping_sub(1u32);
    }
    if _currentBlock == 5 {
    } else if _currentBlock == 6 {
    } else if _currentBlock == 7 {
    }
}
