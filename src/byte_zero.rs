#[no_mangle]
pub unsafe extern "C" fn byte_zero(mut s: *mut u8, mut n: u32) {
    'loop0: loop {
        if n == 0 {
            break;
        }
        *{
            let _old = s;
            s = s.offset(1isize);
            _old
        } = 0u8;
        n = n.wrapping_sub(1u32);
        if n == 0 {
            break;
        }
        *{
            let _old = s;
            s = s.offset(1isize);
            _old
        } = 0u8;
        n = n.wrapping_sub(1u32);
        if n == 0 {
            break;
        }
        *{
            let _old = s;
            s = s.offset(1isize);
            _old
        } = 0u8;
        n = n.wrapping_sub(1u32);
        if n == 0 {
            break;
        }
        *{
            let _old = s;
            s = s.offset(1isize);
            _old
        } = 0u8;
        n = n.wrapping_sub(1u32);
    }
}
