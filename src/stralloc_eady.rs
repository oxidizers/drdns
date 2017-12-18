extern "C" {
    fn alloc(n: u32) -> *mut u8;
    fn alloc_re(x: *mut *mut u8, m: u32, n: u32) -> i32;
}

#[derive(Copy)]
#[repr(C)]
pub struct stralloc {
    pub s: *mut u8,
    pub len: u32,
    pub a: u32,
}

impl Clone for stralloc {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn stralloc_ready(mut x: *mut stralloc, mut n: u32) -> i32 {
    let mut i: u32;
    if !(*x).s.is_null() {
        i = (*x).a;
        (if n > i {
             (*x).a = 30u32.wrapping_add(n).wrapping_add(n >> 3i32);
             (if alloc_re(
                &mut (*x).s as (*mut *mut u8),
                (i as (usize)).wrapping_mul(::std::mem::size_of::<u8>()) as (u32),
                ((*x).a as (usize)).wrapping_mul(::std::mem::size_of::<u8>()) as (u32),
            ) != 0
            {
                  1i32
              } else {
                  (*x).a = i;
                  0i32
              })
         } else {
             1i32
         })
    } else {
        (*x).len = 0u32;
        !{
            (*x).s = alloc(({
                 (*x).a = n;
                 (*x).a
             } as (usize))
                .wrapping_mul(::std::mem::size_of::<u8>()) as
                (u32));
            (*x).s
        }.is_null() as (i32)
    }
}

#[no_mangle]
pub unsafe extern "C" fn stralloc_readyplus(mut x: *mut stralloc, mut n: u32) -> i32 {
    let mut i: u32;
    if !(*x).s.is_null() {
        i = (*x).a;
        n = n.wrapping_add((*x).len);
        (if n > i {
             (*x).a = 30u32.wrapping_add(n).wrapping_add(n >> 3i32);
             (if alloc_re(
                &mut (*x).s as (*mut *mut u8),
                (i as (usize)).wrapping_mul(::std::mem::size_of::<u8>()) as (u32),
                ((*x).a as (usize)).wrapping_mul(::std::mem::size_of::<u8>()) as (u32),
            ) != 0
            {
                  1i32
              } else {
                  (*x).a = i;
                  0i32
              })
         } else {
             1i32
         })
    } else {
        (*x).len = 0u32;
        !{
            (*x).s = alloc(({
                 (*x).a = n;
                 (*x).a
             } as (usize))
                .wrapping_mul(::std::mem::size_of::<u8>()) as
                (u32));
            (*x).s
        }.is_null() as (i32)
    }
}
