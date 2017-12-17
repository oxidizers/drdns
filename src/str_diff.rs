#[no_mangle]
pub unsafe extern fn str_diff(
    mut s : *const u8, mut t : *const u8
) -> i32 {
    let mut x : u8;
    'loop1: loop {
        x = *s;
        if x as (i32) != *t as (i32) {
            break;
        }
        if x == 0 {
            break;
        }
        s = s.offset(1isize);
        t = t.offset(1isize);
        x = *s;
        if x as (i32) != *t as (i32) {
            break;
        }
        if x == 0 {
            break;
        }
        s = s.offset(1isize);
        t = t.offset(1isize);
        x = *s;
        if x as (i32) != *t as (i32) {
            break;
        }
        if x == 0 {
            break;
        }
        s = s.offset(1isize);
        t = t.offset(1isize);
        x = *s;
        if x as (i32) != *t as (i32) {
            break;
        }
        if x == 0 {
            break;
        }
        s = s.offset(1isize);
        t = t.offset(1isize);
    }
    x as (u32) as (i32) - *t as (u32) as (i32)
}
