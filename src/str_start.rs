#[no_mangle]
pub unsafe extern "C" fn str_start(mut s: *const u8, mut t: *const u8) -> i32 {
    let mut _currentBlock;
    let mut x: u8;
    'loop1: loop {
        x = *{
            let _old = t;
            t = t.offset(1isize);
            _old
        };
        if x == 0 {
            _currentBlock = 16;
            break;
        }
        if x as (i32) !=
            *{
                let _old = s;
                s = s.offset(1isize);
                _old
            } as (i32)
        {
            _currentBlock = 15;
            break;
        }
        x = *{
            let _old = t;
            t = t.offset(1isize);
            _old
        };
        if x == 0 {
            _currentBlock = 14;
            break;
        }
        if x as (i32) !=
            *{
                let _old = s;
                s = s.offset(1isize);
                _old
            } as (i32)
        {
            _currentBlock = 13;
            break;
        }
        x = *{
            let _old = t;
            t = t.offset(1isize);
            _old
        };
        if x == 0 {
            _currentBlock = 12;
            break;
        }
        if x as (i32) !=
            *{
                let _old = s;
                s = s.offset(1isize);
                _old
            } as (i32)
        {
            _currentBlock = 11;
            break;
        }
        x = *{
            let _old = t;
            t = t.offset(1isize);
            _old
        };
        if x == 0 {
            _currentBlock = 10;
            break;
        }
        if x as (i32) !=
            *{
                let _old = s;
                s = s.offset(1isize);
                _old
            } as (i32)
        {
            _currentBlock = 9;
            break;
        }
    }
    if _currentBlock == 9 {
        0i32
    } else if _currentBlock == 10 {
        1i32
    } else if _currentBlock == 11 {
        0i32
    } else if _currentBlock == 12 {
        1i32
    } else if _currentBlock == 13 {
        0i32
    } else if _currentBlock == 14 {
        1i32
    } else if _currentBlock == 15 {
        0i32
    } else {
        1i32
    }
}
