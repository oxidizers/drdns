use byte;

extern "C" {
    fn alloc(n: u32) -> *mut u8;
    fn alloc_free(x: *mut u8);
    fn case_diffb(arg1: *const u8, arg2: u32, arg3: *const u8) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn dns_domain_length(mut dn: *const u8) -> u32 {
    let mut x: *const u8;
    let mut c: u8;
    x = dn;
    'loop1: loop {
        if {
            c = *{
                let _old = x;
                x = x.offset(1isize);
                _old
            };
            c
        } == 0
        {
            break;
        }
        x = x.offset(c as (u32) as (isize));
    }
    ((x as (isize)).wrapping_sub(dn as (isize)) / ::std::mem::size_of::<u8>() as (isize)) as (u32)
}

#[no_mangle]
pub unsafe extern "C" fn dns_domain_free(mut out: *mut *mut u8) {
    if !(*out).is_null() {
        alloc_free(*out);
        *out = 0i32 as (*mut u8);
    }
}

#[no_mangle]
pub unsafe extern "C" fn dns_domain_copy(mut out: *mut *mut u8, mut in_: *const u8) -> i32 {
    let mut len: u32;
    let mut x: *mut u8;
    len = dns_domain_length(in_);
    x = alloc(len);
    if x.is_null() {
        0i32
    } else {
        byte::copy(x, len, in_ as (*mut u8));
        if !(*out).is_null() {
            alloc_free(*out);
        }
        *out = x;
        1i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn dns_domain_equal(mut dn1: *const u8, mut dn2: *const u8) -> i32 {
    let mut len: u32;
    len = dns_domain_length(dn1);
    if len != dns_domain_length(dn2) {
        0i32
    } else if case_diffb(dn1, len, dn2) != 0 {
        0i32
    } else {
        1i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn dns_domain_suffix(mut big: *const u8, mut little: *const u8) -> i32 {
    let mut _currentBlock;
    let mut c: u8;
    'loop1: loop {
        if dns_domain_equal(big, little) != 0 {
            _currentBlock = 5;
            break;
        }
        c = *{
            let _old = big;
            big = big.offset(1isize);
            _old
        };
        if c == 0 {
            _currentBlock = 4;
            break;
        }
        big = big.offset(c as (isize));
    }
    if _currentBlock == 4 { 0i32 } else { 1i32 }
}

#[no_mangle]
pub unsafe extern "C" fn dns_domain_suffixpos(mut big: *const u8, mut little: *const u8) -> u32 {
    let mut _currentBlock;
    let mut orig: *const u8 = big;
    let mut c: u8;
    'loop1: loop {
        if dns_domain_equal(big, little) != 0 {
            _currentBlock = 5;
            break;
        }
        c = *{
            let _old = big;
            big = big.offset(1isize);
            _old
        };
        if c == 0 {
            _currentBlock = 4;
            break;
        }
        big = big.offset(c as (isize));
    }
    if _currentBlock == 4 {
        0u32
    } else {
        ((big as (isize)).wrapping_sub(orig as (isize)) /
            ::std::mem::size_of::<u8>() as (isize)) as (u32)
    }
}
