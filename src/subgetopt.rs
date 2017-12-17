#[no_mangle]
pub static mut subgetoptind: i32 = 1i32;

#[no_mangle]
pub static mut subgetoptpos: i32 = 0i32;

#[no_mangle]
pub static mut subgetoptarg: *mut u8 = 0i32 as (*mut u8);

#[no_mangle]
pub static mut subgetoptproblem: i32 = 0i32;

#[no_mangle]
pub static mut subgetoptdone: i32 = -1i32;

#[no_mangle]
pub unsafe extern "C" fn subgetopt(
    mut argc: i32,
    mut argv: *mut *mut u8,
    mut opts: *const u8,
) -> i32 {
    let mut _currentBlock;
    let mut c: i32;
    let mut s: *const u8;
    subgetoptarg = 0i32 as (*mut u8);
    if argv.is_null() || subgetoptind >= argc || (*argv.offset(subgetoptind as (isize))).is_null() {
        subgetoptdone
    } else {
        if subgetoptpos != 0 &&
            (*(*argv.offset(subgetoptind as (isize))).offset(subgetoptpos as (isize)) == 0)
        {
            subgetoptind = subgetoptind + 1;
            subgetoptpos = 0i32;
            if subgetoptind >= argc || (*argv.offset(subgetoptind as (isize))).is_null() {
                return subgetoptdone;
            }
        }
        if subgetoptpos == 0 {
            if *(*argv.offset(subgetoptind as (isize))).offset(0isize) as (i32) != b'-' as (i32) {
                return subgetoptdone;
            } else {
                subgetoptpos = subgetoptpos + 1;
                c = *(*argv.offset(subgetoptind as (isize))).offset(1isize) as (i32);
                if c == b'-' as (i32) || c == 0i32 {
                    if c != 0 {
                        subgetoptind = subgetoptind + 1;
                    }
                    subgetoptpos = 0i32;
                    return subgetoptdone;
                }
            }
        }
        c = *(*argv.offset(subgetoptind as (isize))).offset(subgetoptpos as (isize)) as (i32);
        subgetoptpos = subgetoptpos + 1;
        s = opts;
        'loop7: loop {
            if *s == 0 {
                _currentBlock = 8;
                break;
            }
            if c == *s as (i32) {
                _currentBlock = 12;
                break;
            }
            s = s.offset(1isize);
            if !(*s as (i32) == b':' as (i32)) {
                continue;
            }
            s = s.offset(1isize);
        }
        (if _currentBlock == 8 {
             subgetoptproblem = c;
             b'?' as (i32)
         } else {
             if *s.offset(1isize) as (i32) == b':' as (i32) {
                 subgetoptarg =
                     (*argv.offset(subgetoptind as (isize))).offset(subgetoptpos as (isize));
                 subgetoptind = subgetoptind + 1;
                 subgetoptpos = 0i32;
                 if *subgetoptarg == 0 {
                     subgetoptarg = *argv.offset(subgetoptind as (isize));
                     if subgetoptind >= argc || subgetoptarg.is_null() {
                         subgetoptproblem = c;
                         return b'?' as (i32);
                     } else {
                         subgetoptind = subgetoptind + 1;
                     }
                 }
             }
             c
         })
    }
}
