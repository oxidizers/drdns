extern {
    fn _exit(arg1 : i32);
    fn alloc(n : u32) -> *mut u8;
    fn alloc_re(x : *mut *mut u8, m : u32, n : u32) -> i32;
    static mut buffer_1 : *mut buffer;
    fn buffer_flush(arg1 : *mut buffer) -> i32;
    fn buffer_put(
        arg1 : *mut buffer, arg2 : *const u8, arg3 : u32
    ) -> i32;
    fn buffer_puts(arg1 : *mut buffer, arg2 : *const u8) -> i32;
    fn byte_copy(to : *mut u8, n : u32, from : *mut u8);
    fn byte_diff(s : *mut u8, n : u32, t : *mut u8) -> i32;
    fn byte_zero(s : *mut u8, n : u32);
    fn dd(arg1 : *const u8, arg2 : *const u8, arg3 : *mut u8) -> i32;
    fn dns_domain_copy(arg1 : *mut *mut u8, arg2 : *const u8) -> i32;
    fn dns_domain_equal(arg1 : *const u8, arg2 : *const u8) -> i32;
    fn dns_domain_fromdot(
        arg1 : *mut *mut u8, arg2 : *const u8, arg3 : u32
    ) -> i32;
    fn dns_domain_suffix(arg1 : *const u8, arg2 : *const u8) -> i32;
    fn dns_domain_todot_cat(
        arg1 : *mut stralloc, arg2 : *const u8
    ) -> i32;
    fn dns_ip4_qualify(
        arg1 : *mut stralloc, arg2 : *mut stralloc, arg3 : *const stralloc
    ) -> i32;
    fn dns_packet_copy(
        arg1 : *const u8,
        arg2 : u32,
        arg3 : u32,
        arg4 : *mut u8,
        arg5 : u32
    ) -> u32;
    fn dns_packet_getname(
        arg1 : *const u8, arg2 : u32, arg3 : u32, arg4 : *mut *mut u8
    ) -> u32;
    fn dns_packet_skipname(
        arg1 : *const u8, arg2 : u32, arg3 : u32
    ) -> u32;
    fn dns_random_init(arg1 : *const u8);
    fn dns_transmit_get(
        arg1 : *mut dns_transmit, arg2 : *const pollfd, arg3 : *const taia
    ) -> i32;
    fn dns_transmit_io(
        arg1 : *mut dns_transmit, arg2 : *mut pollfd, arg3 : *mut taia
    );
    fn dns_transmit_start(
        arg1 : *mut dns_transmit,
        arg2 : *const u8,
        arg3 : i32,
        arg4 : *const u8,
        arg5 : *const u8,
        arg6 : *const u8
    ) -> i32;
    static mut errno : i32;
    static mut error_proto : i32;
    fn error_str(arg1 : i32) -> *const u8;
    fn iopause(
        arg1 : *mut pollfd, arg2 : u32, arg3 : *mut taia, arg4 : *mut taia
    );
    fn ip4_fmt(arg1 : *mut u8, arg2 : *const u8) -> u32;
    fn parsetype(arg1 : *mut u8, arg2 : *mut u8) -> i32;
    fn printrecord(
        arg1 : *mut stralloc,
        arg2 : *const u8,
        arg3 : u32,
        arg4 : u32,
        arg5 : *const u8,
        arg6 : *const u8
    ) -> u32;
    fn str_len(arg1 : *const u8) -> u32;
    fn stralloc_catb(
        arg1 : *mut stralloc, arg2 : *const u8, arg3 : u32
    ) -> i32;
    fn stralloc_cats(arg1 : *mut stralloc, arg2 : *const u8) -> i32;
    fn stralloc_catulong0(
        arg1 : *mut stralloc, arg2 : usize, arg3 : u32
    ) -> i32;
    fn stralloc_copys(arg1 : *mut stralloc, arg2 : *const u8) -> i32;
    fn strerr_die(
        arg1 : i32,
        arg2 : *const u8,
        arg3 : *const u8,
        arg4 : *const u8,
        arg5 : *const u8,
        arg6 : *const u8,
        arg7 : *const u8,
        arg8 : *const strerr
    );
    fn taia_add(
        arg1 : *mut taia, arg2 : *const taia, arg3 : *const taia
    );
    fn taia_less(arg1 : *const taia, arg2 : *const taia) -> i32;
    fn taia_now(arg1 : *mut taia);
    fn taia_sub(
        arg1 : *mut taia, arg2 : *const taia, arg3 : *const taia
    );
    fn taia_uint(arg1 : *mut taia, arg2 : u32);
    fn uint16_unpack_big(arg1 : *const u8, arg2 : *mut u16);
}

#[derive(Copy)]
#[repr(C)]
pub struct strerr {
    pub who : *mut strerr,
    pub x : *const u8,
    pub y : *const u8,
    pub z : *const u8,
}

impl Clone for strerr {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn nomem() {
    strerr_die(
        111i32,
        (*b"dnstrace: fatal: \0").as_ptr(),
        (*b"out of memory\0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const strerr)
    );
}

#[no_mangle]
pub unsafe extern fn usage() {
    strerr_die(
        100i32,
        (*b"dnstrace: usage: dnstrace type name rootip ...\0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const strerr)
    );
}

#[derive(Copy)]
#[repr(C)]
pub struct stralloc {
    pub s : *mut u8,
    pub len : u32,
    pub a : u32,
}

impl Clone for stralloc {
    fn clone(&self) -> Self { *self }
}

static mut querystr
    : stralloc
    = stralloc { s: 0 as (*mut u8), len: 0u32, a: 0u32 };

#[no_mangle]
pub static mut ipstr : [u8; 20] = [0u8; 20];

static mut tmp
    : stralloc
    = stralloc { s: 0 as (*mut u8), len: 0u32, a: 0u32 };

#[derive(Copy)]
#[repr(C)]
pub struct buffer {
    pub x : *mut u8,
    pub p : u32,
    pub n : u32,
    pub fd : i32,
    pub op : unsafe extern fn() -> i32,
}

impl Clone for buffer {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn printdomain(mut d : *const u8) {
    if stralloc_copys(
           &mut tmp as (*mut stralloc),
           (*b"\0").as_ptr()
       ) == 0 {
        nomem();
    }
    if dns_domain_todot_cat(&mut tmp as (*mut stralloc),d) == 0 {
        nomem();
    }
    buffer_put(buffer_1,tmp.s as (*const u8),tmp.len);
}

#[derive(Copy)]
#[repr(C)]
pub struct tai {
    pub x : usize,
}

impl Clone for tai {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct taia {
    pub sec : tai,
    pub nano : usize,
    pub atto : usize,
}

impl Clone for taia {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct dns_transmit {
    pub query : *mut u8,
    pub querylen : u32,
    pub packet : *mut u8,
    pub packetlen : u32,
    pub s1 : i32,
    pub tcpstate : i32,
    pub udploop : u32,
    pub curserver : u32,
    pub deadline : taia,
    pub pos : u32,
    pub servers : *const u8,
    pub localip : [u8; 4],
    pub qtype : [u8; 2],
}

impl Clone for dns_transmit {
    fn clone(&self) -> Self { *self }
}

static mut tx
    : dns_transmit
    = dns_transmit {
          query: 0 as (*mut u8),
          querylen: 0u32,
          packet: 0 as (*mut u8),
          packetlen: 0u32,
          s1: 0i32,
          tcpstate: 0i32,
          udploop: 0u32,
          curserver: 0u32,
          deadline: taia {
                        sec: tai { x: 0usize },
                        nano: 0usize,
                        atto: 0usize
                    },
          pos: 0u32,
          servers: 0 as (*const u8),
          localip: [0u8; 4],
          qtype: [0u8; 2]
      };

#[derive(Copy)]
#[repr(C)]
pub struct pollfd {
    pub fd : i32,
    pub events : i16,
    pub revents : i16,
}

impl Clone for pollfd {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn resolve(
    mut q : *mut u8, mut qtype : *mut u8, mut ip : *mut u8
) -> i32 {
    let mut _currentBlock;
    let mut start : taia;
    let mut stamp : taia;
    let mut deadline : taia;
    let mut servers : [u8; 64];
    let mut x : [pollfd; 1];
    let mut r : i32;
    taia_now(&mut start as (*mut taia));
    byte_zero(servers.as_mut_ptr(),64u32);
    byte_copy(servers.as_mut_ptr(),4u32,ip);
    if dns_transmit_start(
           &mut tx as (*mut dns_transmit),
           servers.as_mut_ptr() as (*const u8),
           0i32,
           q as (*const u8),
           qtype as (*const u8),
           (*b"\0\0\0\0\0").as_ptr()
       ) == -1i32 {
        -1i32
    } else {
        'loop1: loop {
            taia_now(&mut stamp as (*mut taia));
            taia_uint(&mut deadline as (*mut taia),120u32);
            taia_add(
                &mut deadline as (*mut taia),
                &mut deadline as (*mut taia) as (*const taia),
                &mut stamp as (*mut taia) as (*const taia)
            );
            dns_transmit_io(
                &mut tx as (*mut dns_transmit),
                x.as_mut_ptr(),
                &mut deadline as (*mut taia)
            );
            iopause(
                x.as_mut_ptr(),
                1u32,
                &mut deadline as (*mut taia),
                &mut stamp as (*mut taia)
            );
            r = dns_transmit_get(
                    &mut tx as (*mut dns_transmit),
                    x.as_mut_ptr() as (*const pollfd),
                    &mut stamp as (*mut taia) as (*const taia)
                );
            if r == -1i32 {
                _currentBlock = 6;
                break;
            }
            if r == 1i32 {
                _currentBlock = 3;
                break;
            }
        }
        (if _currentBlock == 3 {
             taia_now(&mut stamp as (*mut taia));
             taia_sub(
                 &mut stamp as (*mut taia),
                 &mut stamp as (*mut taia) as (*const taia),
                 &mut start as (*mut taia) as (*const taia)
             );
             taia_uint(&mut deadline as (*mut taia),1u32);
             if taia_less(
                    &mut deadline as (*mut taia) as (*const taia),
                    &mut stamp as (*mut taia) as (*const taia)
                ) != 0 {
                 buffer_put(buffer_1,querystr.s as (*const u8),querystr.len);
                 buffer_puts(
                     buffer_1,
                     (*b"ALERT:took more than 1 second\n\0").as_ptr()
                 );
             }
             0i32
         } else {
             -1i32
         })
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct address {
    pub owner : *mut u8,
    pub ip : [u8; 4],
}

impl Clone for address {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct address_alloc {
    pub s : *mut address,
    pub len : u32,
    pub a : u32,
}

impl Clone for address_alloc {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn address_alloc_readyplus(
    mut x : *mut address_alloc, mut n : u32
) -> i32 {
    let mut i : u32;
    if !(*x).s.is_null() {
        i = (*x).a;
        n = n.wrapping_add((*x).len);
        (if n > i {
             (*x).a = 30u32.wrapping_add(n).wrapping_add(n >> 3i32);
             (if alloc_re(
                     &mut (*x).s as (*mut *mut address) as (*mut *mut u8),
                     (i as (usize)).wrapping_mul(
                         ::std::mem::size_of::<address>()
                     ) as (u32),
                     ((*x).a as (usize)).wrapping_mul(
                         ::std::mem::size_of::<address>()
                     ) as (u32)
                 ) != 0 {
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
             (*x).s = alloc(
                          ({
                               (*x).a = n;
                               (*x).a
                           } as (usize)).wrapping_mul(
                              ::std::mem::size_of::<address>()
                          ) as (u32)
                      ) as (*mut address);
             (*x).s
         }.is_null(
         ) as (i32)
    }
}

#[no_mangle]
pub unsafe extern fn address_alloc_append(
    mut x : *mut address_alloc, mut i : *const address
) -> i32 {
    if address_alloc_readyplus(x,1u32) == 0 {
        0i32
    } else {
        *(*x).s.offset(
             {
                 let _old = (*x).len;
                 (*x).len = (*x).len.wrapping_add(1u32);
                 _old
             } as (isize)
         ) = *i;
        1i32
    }
}

static mut address
    : address_alloc
    = address_alloc { s: 0 as (*mut address), len: 0u32, a: 0u32 };

#[derive(Copy)]
#[repr(C)]
pub struct ns {
    pub owner : *mut u8,
    pub ns : *mut u8,
}

impl Clone for ns {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct ns_alloc {
    pub s : *mut ns,
    pub len : u32,
    pub a : u32,
}

impl Clone for ns_alloc {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn ns_alloc_readyplus(
    mut x : *mut ns_alloc, mut n : u32
) -> i32 {
    let mut i : u32;
    if !(*x).s.is_null() {
        i = (*x).a;
        n = n.wrapping_add((*x).len);
        (if n > i {
             (*x).a = 30u32.wrapping_add(n).wrapping_add(n >> 3i32);
             (if alloc_re(
                     &mut (*x).s as (*mut *mut ns) as (*mut *mut u8),
                     (i as (usize)).wrapping_mul(::std::mem::size_of::<ns>()) as (u32),
                     ((*x).a as (usize)).wrapping_mul(
                         ::std::mem::size_of::<ns>()
                     ) as (u32)
                 ) != 0 {
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
             (*x).s = alloc(
                          ({
                               (*x).a = n;
                               (*x).a
                           } as (usize)).wrapping_mul(
                              ::std::mem::size_of::<ns>()
                          ) as (u32)
                      ) as (*mut ns);
             (*x).s
         }.is_null(
         ) as (i32)
    }
}

#[no_mangle]
pub unsafe extern fn ns_alloc_append(
    mut x : *mut ns_alloc, mut i : *const ns
) -> i32 {
    if ns_alloc_readyplus(x,1u32) == 0 {
        0i32
    } else {
        *(*x).s.offset(
             {
                 let _old = (*x).len;
                 (*x).len = (*x).len.wrapping_add(1u32);
                 _old
             } as (isize)
         ) = *i;
        1i32
    }
}

static mut ns
    : ns_alloc
    = ns_alloc { s: 0 as (*mut ns), len: 0u32, a: 0u32 };

#[derive(Copy)]
#[repr(C)]
pub struct query {
    pub owner : *mut u8,
    pub type_ : [u8; 2],
}

impl Clone for query {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct query_alloc {
    pub s : *mut query,
    pub len : u32,
    pub a : u32,
}

impl Clone for query_alloc {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn query_alloc_readyplus(
    mut x : *mut query_alloc, mut n : u32
) -> i32 {
    let mut i : u32;
    if !(*x).s.is_null() {
        i = (*x).a;
        n = n.wrapping_add((*x).len);
        (if n > i {
             (*x).a = 30u32.wrapping_add(n).wrapping_add(n >> 3i32);
             (if alloc_re(
                     &mut (*x).s as (*mut *mut query) as (*mut *mut u8),
                     (i as (usize)).wrapping_mul(
                         ::std::mem::size_of::<query>()
                     ) as (u32),
                     ((*x).a as (usize)).wrapping_mul(
                         ::std::mem::size_of::<query>()
                     ) as (u32)
                 ) != 0 {
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
             (*x).s = alloc(
                          ({
                               (*x).a = n;
                               (*x).a
                           } as (usize)).wrapping_mul(
                              ::std::mem::size_of::<query>()
                          ) as (u32)
                      ) as (*mut query);
             (*x).s
         }.is_null(
         ) as (i32)
    }
}

#[no_mangle]
pub unsafe extern fn query_alloc_append(
    mut x : *mut query_alloc, mut i : *const query
) -> i32 {
    if query_alloc_readyplus(x,1u32) == 0 {
        0i32
    } else {
        *(*x).s.offset(
             {
                 let _old = (*x).len;
                 (*x).len = (*x).len.wrapping_add(1u32);
                 _old
             } as (isize)
         ) = *i;
        1i32
    }
}

static mut query
    : query_alloc
    = query_alloc { s: 0 as (*mut query), len: 0u32, a: 0u32 };

#[derive(Copy)]
#[repr(C)]
pub struct qt {
    pub owner : *mut u8,
    pub type_ : [u8; 2],
    pub control : *mut u8,
    pub ip : [u8; 4],
}

impl Clone for qt {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct qt_alloc {
    pub s : *mut qt,
    pub len : u32,
    pub a : u32,
}

impl Clone for qt_alloc {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn qt_alloc_readyplus(
    mut x : *mut qt_alloc, mut n : u32
) -> i32 {
    let mut i : u32;
    if !(*x).s.is_null() {
        i = (*x).a;
        n = n.wrapping_add((*x).len);
        (if n > i {
             (*x).a = 30u32.wrapping_add(n).wrapping_add(n >> 3i32);
             (if alloc_re(
                     &mut (*x).s as (*mut *mut qt) as (*mut *mut u8),
                     (i as (usize)).wrapping_mul(::std::mem::size_of::<qt>()) as (u32),
                     ((*x).a as (usize)).wrapping_mul(
                         ::std::mem::size_of::<qt>()
                     ) as (u32)
                 ) != 0 {
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
             (*x).s = alloc(
                          ({
                               (*x).a = n;
                               (*x).a
                           } as (usize)).wrapping_mul(
                              ::std::mem::size_of::<qt>()
                          ) as (u32)
                      ) as (*mut qt);
             (*x).s
         }.is_null(
         ) as (i32)
    }
}

#[no_mangle]
pub unsafe extern fn qt_alloc_append(
    mut x : *mut qt_alloc, mut i : *const qt
) -> i32 {
    if qt_alloc_readyplus(x,1u32) == 0 {
        0i32
    } else {
        *(*x).s.offset(
             {
                 let _old = (*x).len;
                 (*x).len = (*x).len.wrapping_add(1u32);
                 _old
             } as (isize)
         ) = *i;
        1i32
    }
}

static mut qt
    : qt_alloc
    = qt_alloc { s: 0 as (*mut qt), len: 0u32, a: 0u32 };

#[no_mangle]
pub unsafe extern fn qt_add(
    mut q : *const u8,
    mut type_ : *const u8,
    mut control : *const u8,
    mut ip : *const u8
) {
    let mut _currentBlock;
    let mut x : qt;
    let mut i : i32;
    if *q == 0 {
    } else {
        i = 0i32;
        'loop2: loop {
            if !(i as (u32) < qt.len) {
                _currentBlock = 3;
                break;
            }
            if dns_domain_equal(
                   (*qt.s.offset(i as (isize))).owner as (*const u8),
                   q
               ) != 0 {
                if dns_domain_equal(
                       (*qt.s.offset(i as (isize))).control as (*const u8),
                       control
                   ) != 0 {
                    if byte_diff(
                           (*qt.s.offset(i as (isize))).type_.as_mut_ptr(),
                           2u32,
                           type_ as (*mut u8)
                       ) == 0 {
                        if byte_diff(
                               (*qt.s.offset(i as (isize))).ip.as_mut_ptr(),
                               4u32,
                               ip as (*mut u8)
                           ) == 0 {
                            _currentBlock = 15;
                            break;
                        }
                    }
                }
            }
            i = i + 1;
        }
        (if _currentBlock == 3 {
             byte_zero(
                 &mut x as (*mut qt) as (*mut u8),
                 ::std::mem::size_of::<qt>() as (u32)
             );
             if dns_domain_copy(&mut x.owner as (*mut *mut u8),q) == 0 {
                 nomem();
             }
             if dns_domain_copy(&mut x.control as (*mut *mut u8),control) == 0 {
                 nomem();
             }
             byte_copy(x.type_.as_mut_ptr(),2u32,type_ as (*mut u8));
             byte_copy(x.ip.as_mut_ptr(),4u32,ip as (*mut u8));
             if qt_alloc_append(
                    &mut qt as (*mut qt_alloc),
                    &mut x as (*mut qt) as (*const qt)
                ) == 0 {
                 nomem();
             }
         })
    }
}

#[no_mangle]
pub unsafe extern fn query_add(
    mut owner : *const u8, mut type_ : *const u8
) {
    let mut _currentBlock;
    let mut x : query;
    let mut i : i32;
    let mut j : i32;
    i = 0i32;
    'loop1: loop {
        if !(i as (u32) < query.len) {
            _currentBlock = 2;
            break;
        }
        if dns_domain_equal(
               (*query.s.offset(i as (isize))).owner as (*const u8),
               owner
           ) != 0 {
            if byte_diff(
                   (*query.s.offset(i as (isize))).type_.as_mut_ptr(),
                   2u32,
                   type_ as (*mut u8)
               ) == 0 {
                _currentBlock = 19;
                break;
            }
        }
        i = i + 1;
    }
    if _currentBlock == 2 {
        byte_zero(
            &mut x as (*mut query) as (*mut u8),
            ::std::mem::size_of::<query>() as (u32)
        );
        if dns_domain_copy(&mut x.owner as (*mut *mut u8),owner) == 0 {
            nomem();
        }
        byte_copy(x.type_.as_mut_ptr(),2u32,type_ as (*mut u8));
        if query_alloc_append(
               &mut query as (*mut query_alloc),
               &mut x as (*mut query) as (*const query)
           ) == 0 {
            nomem();
        }
        i = 0i32;
        'loop7: loop {
            if !(i as (u32) < ns.len) {
                break;
            }
            if dns_domain_suffix(
                   owner,
                   (*ns.s.offset(i as (isize))).owner as (*const u8)
               ) != 0 {
                j = 0i32;
                'loop11: loop {
                    if !(j as (u32) < address.len) {
                        break;
                    }
                    if dns_domain_equal(
                           (*ns.s.offset(i as (isize))).ns as (*const u8),
                           (*address.s.offset(j as (isize))).owner as (*const u8)
                       ) != 0 {
                        qt_add(
                            owner,
                            type_,
                            (*ns.s.offset(i as (isize))).owner as (*const u8),
                            (*address.s.offset(j as (isize))).ip.as_mut_ptr() as (*const u8)
                        );
                    }
                    j = j + 1;
                }
            }
            i = i + 1;
        }
    }
}

#[no_mangle]
pub unsafe extern fn ns_add(
    mut owner : *const u8, mut server : *const u8
) {
    let mut _currentBlock;
    let mut x : ns;
    let mut i : i32;
    let mut j : i32;
    buffer_put(buffer_1,querystr.s as (*const u8),querystr.len);
    buffer_puts(buffer_1,(*b"NS:\0").as_ptr());
    printdomain(owner);
    buffer_puts(buffer_1,(*b":\0").as_ptr());
    printdomain(server);
    buffer_puts(buffer_1,(*b"\n\0").as_ptr());
    i = 0i32;
    'loop1: loop {
        if !(i as (u32) < ns.len) {
            _currentBlock = 2;
            break;
        }
        if dns_domain_equal(
               (*ns.s.offset(i as (isize))).owner as (*const u8),
               owner
           ) != 0 {
            if dns_domain_equal(
                   (*ns.s.offset(i as (isize))).ns as (*const u8),
                   server
               ) != 0 {
                _currentBlock = 21;
                break;
            }
        }
        i = i + 1;
    }
    if _currentBlock == 2 {
        query_add(server,(*b"\0\x01\0").as_ptr());
        byte_zero(
            &mut x as (*mut ns) as (*mut u8),
            ::std::mem::size_of::<ns>() as (u32)
        );
        if dns_domain_copy(&mut x.owner as (*mut *mut u8),owner) == 0 {
            nomem();
        }
        if dns_domain_copy(&mut x.ns as (*mut *mut u8),server) == 0 {
            nomem();
        }
        if ns_alloc_append(
               &mut ns as (*mut ns_alloc),
               &mut x as (*mut ns) as (*const ns)
           ) == 0 {
            nomem();
        }
        i = 0i32;
        'loop9: loop {
            if !(i as (u32) < query.len) {
                break;
            }
            if dns_domain_suffix(
                   (*query.s.offset(i as (isize))).owner as (*const u8),
                   owner
               ) != 0 {
                j = 0i32;
                'loop13: loop {
                    if !(j as (u32) < address.len) {
                        break;
                    }
                    if dns_domain_equal(
                           server,
                           (*address.s.offset(j as (isize))).owner as (*const u8)
                       ) != 0 {
                        qt_add(
                            (*query.s.offset(i as (isize))).owner as (*const u8),
                            (*query.s.offset(i as (isize))).type_.as_mut_ptr() as (*const u8),
                            owner,
                            (*address.s.offset(j as (isize))).ip.as_mut_ptr() as (*const u8)
                        );
                    }
                    j = j + 1;
                }
            }
            i = i + 1;
        }
    }
}

#[no_mangle]
pub unsafe extern fn address_add(
    mut owner : *const u8, mut ip : *const u8
) {
    let mut _currentBlock;
    let mut x : address;
    let mut i : i32;
    let mut j : i32;
    buffer_put(buffer_1,querystr.s as (*const u8),querystr.len);
    buffer_puts(buffer_1,(*b"A:\0").as_ptr());
    printdomain(owner);
    buffer_puts(buffer_1,(*b":\0").as_ptr());
    buffer_put(
        buffer_1,
        ipstr.as_mut_ptr() as (*const u8),
        ip4_fmt(ipstr.as_mut_ptr(),ip)
    );
    buffer_puts(buffer_1,(*b"\n\0").as_ptr());
    i = 0i32;
    'loop1: loop {
        if !(i as (u32) < address.len) {
            _currentBlock = 2;
            break;
        }
        if dns_domain_equal(
               (*address.s.offset(i as (isize))).owner as (*const u8),
               owner
           ) != 0 {
            if byte_diff(
                   (*address.s.offset(i as (isize))).ip.as_mut_ptr(),
                   4u32,
                   ip as (*mut u8)
               ) == 0 {
                _currentBlock = 19;
                break;
            }
        }
        i = i + 1;
    }
    if _currentBlock == 2 {
        byte_zero(
            &mut x as (*mut address) as (*mut u8),
            ::std::mem::size_of::<address>() as (u32)
        );
        if dns_domain_copy(&mut x.owner as (*mut *mut u8),owner) == 0 {
            nomem();
        }
        byte_copy(x.ip.as_mut_ptr(),4u32,ip as (*mut u8));
        if address_alloc_append(
               &mut address as (*mut address_alloc),
               &mut x as (*mut address) as (*const address)
           ) == 0 {
            nomem();
        }
        i = 0i32;
        'loop7: loop {
            if !(i as (u32) < ns.len) {
                break;
            }
            if dns_domain_equal(
                   (*ns.s.offset(i as (isize))).ns as (*const u8),
                   owner
               ) != 0 {
                j = 0i32;
                'loop11: loop {
                    if !(j as (u32) < query.len) {
                        break;
                    }
                    if dns_domain_suffix(
                           (*query.s.offset(j as (isize))).owner as (*const u8),
                           (*ns.s.offset(i as (isize))).owner as (*const u8)
                       ) != 0 {
                        qt_add(
                            (*query.s.offset(j as (isize))).owner as (*const u8),
                            (*query.s.offset(j as (isize))).type_.as_mut_ptr() as (*const u8),
                            (*ns.s.offset(i as (isize))).owner as (*const u8),
                            ip
                        );
                    }
                    j = j + 1;
                }
            }
            i = i + 1;
        }
    }
}

#[no_mangle]
pub static mut seed : [u8; 128] = [0u8; 128];

static mut t1 : *mut u8 = 0 as (*mut u8);

static mut t2 : *mut u8 = 0 as (*mut u8);

static mut referral : *mut u8 = 0 as (*mut u8);

static mut cname : *mut u8 = 0 as (*mut u8);

unsafe extern fn typematch(
    mut rtype : *const u8, mut qtype : *const u8
) -> i32 {
    (byte_diff(
         qtype as (*mut u8),
         2u32,
         rtype as (*mut u8)
     ) == 0 || byte_diff(
                   qtype as (*mut u8),
                   2u32,
                   (*b"\0\xFF\0").as_ptr() as (*mut u8)
               ) == 0) as (i32)
}

#[no_mangle]
pub unsafe extern fn parsepacket(
    mut buf : *const u8,
    mut len : u32,
    mut d : *const u8,
    mut dtype : *const u8,
    mut control : *const u8
) {
    let mut _currentBlock;
    let mut misc : [u8; 20];
    let mut header : [u8; 12];
    let mut pos : u32;
    let mut numanswers : u16;
    let mut posanswers : u32;
    let mut numauthority : u16;
    let mut posauthority : u32;
    let mut numglue : u16;
    let mut posglue : u32;
    let mut datalen : u16;
    let mut rcode : u32;
    let mut flagout : i32;
    let mut flagcname : i32;
    let mut flagreferral : i32;
    let mut flagsoa : i32;
    let mut j : i32;
    let mut x : *const u8;
    pos = dns_packet_copy(buf,len,0u32,header.as_mut_ptr(),12u32);
    if !(pos == 0) {
        pos = dns_packet_skipname(buf,len,pos);
        if !(pos == 0) {
            pos = pos.wrapping_add(4u32);
            uint16_unpack_big(
                header.as_mut_ptr().offset(6isize) as (*const u8),
                &mut numanswers as (*mut u16)
            );
            uint16_unpack_big(
                header.as_mut_ptr().offset(8isize) as (*const u8),
                &mut numauthority as (*mut u16)
            );
            uint16_unpack_big(
                header.as_mut_ptr().offset(10isize) as (*const u8),
                &mut numglue as (*mut u16)
            );
            rcode = (header[3usize] as (i32) & 15i32) as (u32);
            if rcode != 0 && (rcode != 3u32) {
                errno = error_proto;
            } else {
                flagout = 0i32;
                flagcname = 0i32;
                flagreferral = 0i32;
                flagsoa = 0i32;
                posanswers = pos;
                j = 0i32;
                'loop4: loop {
                    if !(j < numanswers as (i32)) {
                        _currentBlock = 5;
                        break;
                    }
                    pos = dns_packet_getname(buf,len,pos,&mut t1 as (*mut *mut u8));
                    if pos == 0 {
                        _currentBlock = 60;
                        break;
                    }
                    pos = dns_packet_copy(buf,len,pos,header.as_mut_ptr(),10u32);
                    if pos == 0 {
                        _currentBlock = 60;
                        break;
                    }
                    if dns_domain_equal(t1 as (*const u8),d) != 0 {
                        if byte_diff(
                               header.as_mut_ptr().offset(2isize),
                               2u32,
                               (*b"\0\x01\0").as_ptr() as (*mut u8)
                           ) == 0 {
                            if typematch(header.as_mut_ptr() as (*const u8),dtype) != 0 {
                                flagout = 1i32;
                            } else if typematch(
                                          header.as_mut_ptr() as (*const u8),
                                          (*b"\0\x05\0").as_ptr()
                                      ) != 0 {
                                if dns_packet_getname(
                                       buf,
                                       len,
                                       pos,
                                       &mut cname as (*mut *mut u8)
                                   ) == 0 {
                                    _currentBlock = 60;
                                    break;
                                }
                                flagcname = 1i32;
                            }
                        }
                    }
                    uint16_unpack_big(
                        header.as_mut_ptr().offset(8isize) as (*const u8),
                        &mut datalen as (*mut u16)
                    );
                    pos = pos.wrapping_add(datalen as (u32));
                    j = j + 1;
                }
                if _currentBlock == 60 {
                } else {
                    posauthority = pos;
                    j = 0i32;
                    'loop6: loop {
                        if !(j < numauthority as (i32)) {
                            _currentBlock = 7;
                            break;
                        }
                        pos = dns_packet_getname(buf,len,pos,&mut t1 as (*mut *mut u8));
                        if pos == 0 {
                            _currentBlock = 60;
                            break;
                        }
                        pos = dns_packet_copy(buf,len,pos,header.as_mut_ptr(),10u32);
                        if pos == 0 {
                            _currentBlock = 60;
                            break;
                        }
                        if typematch(
                               header.as_mut_ptr() as (*const u8),
                               (*b"\0\x06\0").as_ptr()
                           ) != 0 {
                            flagsoa = 1i32;
                        } else if typematch(
                                      header.as_mut_ptr() as (*const u8),
                                      (*b"\0\x02\0").as_ptr()
                                  ) != 0 {
                            flagreferral = 1i32;
                            if dns_domain_copy(
                                   &mut referral as (*mut *mut u8),
                                   t1 as (*const u8)
                               ) == 0 {
                                _currentBlock = 60;
                                break;
                            }
                        }
                        uint16_unpack_big(
                            header.as_mut_ptr().offset(8isize) as (*const u8),
                            &mut datalen as (*mut u16)
                        );
                        pos = pos.wrapping_add(datalen as (u32));
                        j = j + 1;
                    }
                    if _currentBlock == 60 {
                    } else {
                        posglue = pos;
                        if flagcname == 0 && (rcode == 0) && (flagout == 0) && (flagreferral != 0) && (flagsoa == 0) {
                            if dns_domain_equal(
                                   referral as (*const u8),
                                   control
                               ) != 0 || dns_domain_suffix(referral as (*const u8),control) == 0 {
                                buffer_put(buffer_1,querystr.s as (*const u8),querystr.len);
                                buffer_puts(
                                    buffer_1,
                                    (*b"ALERT:lame server; refers to \0").as_ptr()
                                );
                                printdomain(referral as (*const u8));
                                buffer_puts(buffer_1,(*b"\n\0").as_ptr());
                                return;
                            }
                        }
                        pos = posanswers;
                        j = 0i32;
                        'loop10: loop {
                            if !(j < numanswers as (i32) + numauthority as (i32) + numglue as (i32)) {
                                _currentBlock = 11;
                                break;
                            }
                            pos = dns_packet_getname(buf,len,pos,&mut t1 as (*mut *mut u8));
                            if pos == 0 {
                                _currentBlock = 60;
                                break;
                            }
                            pos = dns_packet_copy(buf,len,pos,header.as_mut_ptr(),10u32);
                            if pos == 0 {
                                _currentBlock = 60;
                                break;
                            }
                            uint16_unpack_big(
                                header.as_mut_ptr().offset(8isize) as (*const u8),
                                &mut datalen as (*mut u16)
                            );
                            if dns_domain_suffix(t1 as (*const u8),control) != 0 {
                                if byte_diff(
                                       header.as_mut_ptr().offset(2isize),
                                       2u32,
                                       (*b"\0\x01\0").as_ptr() as (*mut u8)
                                   ) == 0 {
                                    if typematch(
                                           header.as_mut_ptr() as (*const u8),
                                           (*b"\0\x02\0").as_ptr()
                                       ) != 0 {
                                        if dns_packet_getname(
                                               buf,
                                               len,
                                               pos,
                                               &mut t2 as (*mut *mut u8)
                                           ) == 0 {
                                            _currentBlock = 60;
                                            break;
                                        }
                                        ns_add(t1 as (*const u8),t2 as (*const u8));
                                    } else if typematch(
                                                  header.as_mut_ptr() as (*const u8),
                                                  (*b"\0\x01\0").as_ptr()
                                              ) != 0 && (datalen as (i32) == 4i32) {
                                        if dns_packet_copy(
                                               buf,
                                               len,
                                               pos,
                                               misc.as_mut_ptr(),
                                               4u32
                                           ) == 0 {
                                            _currentBlock = 60;
                                            break;
                                        }
                                        address_add(
                                            t1 as (*const u8),
                                            misc.as_mut_ptr() as (*const u8)
                                        );
                                    }
                                }
                            }
                            pos = pos.wrapping_add(datalen as (u32));
                            j = j + 1;
                        }
                        if _currentBlock == 60 {
                        } else if flagcname != 0 {
                            query_add(cname as (*const u8),dtype);
                            buffer_put(buffer_1,querystr.s as (*const u8),querystr.len);
                            buffer_puts(buffer_1,(*b"CNAME:\0").as_ptr());
                            printdomain(cname as (*const u8));
                            buffer_puts(buffer_1,(*b"\n\0").as_ptr());
                            return;
                        } else if rcode == 3u32 {
                            buffer_put(buffer_1,querystr.s as (*const u8),querystr.len);
                            buffer_puts(buffer_1,(*b"NXDOMAIN\n\0").as_ptr());
                            return;
                        } else if flagout != 0 || flagsoa != 0 || flagreferral == 0 {
                            if flagout == 0 {
                                buffer_put(buffer_1,querystr.s as (*const u8),querystr.len);
                                buffer_puts(buffer_1,(*b"NODATA\n\0").as_ptr());
                                return;
                            } else {
                                pos = posanswers;
                                j = 0i32;
                                'loop18: loop {
                                    if !(j < numanswers as (i32) + numauthority as (i32) + numglue as (i32)) {
                                        _currentBlock = 19;
                                        break;
                                    }
                                    pos = printrecord(
                                              &mut tmp as (*mut stralloc),
                                              buf,
                                              len,
                                              pos,
                                              d,
                                              dtype
                                          );
                                    if pos == 0 {
                                        _currentBlock = 60;
                                        break;
                                    }
                                    if tmp.len != 0 {
                                        buffer_put(buffer_1,querystr.s as (*const u8),querystr.len);
                                        buffer_puts(buffer_1,(*b"answer:\0").as_ptr());
                                        buffer_put(buffer_1,tmp.s as (*const u8),tmp.len);
                                    }
                                    j = j + 1;
                                }
                                if _currentBlock == 60 {
                                } else {
                                    return;
                                }
                            }
                        } else if !(dns_domain_suffix(d,referral as (*const u8)) == 0) {
                            buffer_put(buffer_1,querystr.s as (*const u8),querystr.len);
                            buffer_puts(buffer_1,(*b"see:\0").as_ptr());
                            printdomain(referral as (*const u8));
                            buffer_puts(buffer_1,(*b"\n\0").as_ptr());
                            return;
                        }
                    }
                }
            }
        }
    }
    x = error_str(errno);
    buffer_put(buffer_1,querystr.s as (*const u8),querystr.len);
    buffer_puts(
        buffer_1,
        (*b"ALERT:unable to parse response packet; \0").as_ptr()
    );
    buffer_puts(buffer_1,x);
    buffer_puts(buffer_1,(*b"\n\0").as_ptr());
}

fn main() {
    use ::std::os::unix::ffi::OsStringExt;
    let mut argv_storage
        = ::std::env::args_os().map(
              |str| {
                        let mut vec = str.into_vec();
                        vec.push(b'\0');
                        vec
                    }
          ).collect::<Vec<_>>(
          );
    let mut argv
        = argv_storage.iter_mut().map(|vec| vec.as_mut_ptr()).chain(
              Some(::std::ptr::null_mut())
          ).collect::<Vec<_>>(
          );
    let ret
        = unsafe {
              _c_main(argv_storage.len() as (i32),argv.as_mut_ptr())
          };
    ::std::process::exit(ret);
}

#[no_mangle]
pub unsafe extern fn _c_main(
    mut argc : i32, mut argv : *mut *mut u8
) -> i32 {
    static mut out
        : stralloc
        = stralloc { s: 0 as (*mut u8), len: 0u32, a: 0u32 };
    static mut fqdn
        : stralloc
        = stralloc { s: 0 as (*mut u8), len: 0u32, a: 0u32 };
    static mut udn
        : stralloc
        = stralloc { s: 0 as (*mut u8), len: 0u32, a: 0u32 };
    static mut q : *mut u8 = 0 as (*mut u8);
    let mut control : *mut u8;
    let mut type_ : [u8; 2];
    let mut ip : [u8; 64];
    let mut i : i32;
    let mut u16 : u16;
    dns_random_init(seed.as_mut_ptr() as (*const u8));
    if stralloc_copys(
           &mut querystr as (*mut stralloc),
           (*b"0:.:.:start:\0").as_ptr()
       ) == 0 {
        nomem();
    }
    if address_alloc_readyplus(
           &mut address as (*mut address_alloc),
           1u32
       ) == 0 {
        nomem();
    }
    if query_alloc_readyplus(
           &mut query as (*mut query_alloc),
           1u32
       ) == 0 {
        nomem();
    }
    if ns_alloc_readyplus(&mut ns as (*mut ns_alloc),1u32) == 0 {
        nomem();
    }
    if qt_alloc_readyplus(&mut qt as (*mut qt_alloc),1u32) == 0 {
        nomem();
    }
    if (*argv).is_null() {
        usage();
    }
    if (*{
             argv = argv.offset(1isize);
             argv
         }).is_null(
       ) {
        usage();
    }
    if parsetype(*argv,type_.as_mut_ptr()) == 0 {
        usage();
    }
    if (*{
             argv = argv.offset(1isize);
             argv
         }).is_null(
       ) {
        usage();
    }
    if dns_domain_fromdot(
           &mut q as (*mut *mut u8),
           *argv as (*const u8),
           str_len(*argv as (*const u8))
       ) == 0 {
        nomem();
    }
    query_add(q as (*const u8),type_.as_mut_ptr() as (*const u8));
    ns_add((*b"\0").as_ptr(),(*b"\0").as_ptr());
    'loop21: loop {
        if (*{
                 argv = argv.offset(1isize);
                 argv
             }).is_null(
           ) {
            break;
        }
        if stralloc_copys(
               &mut udn as (*mut stralloc),
               *argv as (*const u8)
           ) == 0 {
            nomem();
        }
        if dns_ip4_qualify(
               &mut out as (*mut stralloc),
               &mut fqdn as (*mut stralloc),
               &mut udn as (*mut stralloc) as (*const stralloc)
           ) == -1i32 {
            nomem();
        }
        i = 0i32;
        'loop60: loop {
            if !((i + 4i32) as (u32) <= out.len) {
                break;
            }
            address_add(
                (*b"\0").as_ptr(),
                out.s.offset(i as (isize)) as (*const u8)
            );
            i = i + 4i32;
        }
    }
    i = 0i32;
    'loop23: loop {
        if !(i as (u32) < qt.len) {
            break;
        }
        if dns_domain_copy(
               &mut q as (*mut *mut u8),
               (*qt.s.offset(i as (isize))).owner as (*const u8)
           ) == 0 {
            nomem();
        }
        control = (*qt.s.offset(i as (isize))).control;
        if !(dns_domain_suffix(
                 q as (*const u8),
                 control as (*const u8)
             ) == 0) {
            byte_copy(
                type_.as_mut_ptr(),
                2u32,
                (*qt.s.offset(i as (isize))).type_.as_mut_ptr()
            );
            byte_copy(
                ip.as_mut_ptr(),
                4u32,
                (*qt.s.offset(i as (isize))).ip.as_mut_ptr()
            );
            if stralloc_copys(
                   &mut querystr as (*mut stralloc),
                   (*b"\0").as_ptr()
               ) == 0 {
                nomem();
            }
            uint16_unpack_big(
                type_.as_mut_ptr() as (*const u8),
                &mut u16 as (*mut u16)
            );
            if stralloc_catulong0(
                   &mut querystr as (*mut stralloc),
                   u16 as (usize),
                   0u32
               ) == 0 {
                nomem();
            }
            if stralloc_cats(
                   &mut querystr as (*mut stralloc),
                   (*b":\0").as_ptr()
               ) == 0 {
                nomem();
            }
            if dns_domain_todot_cat(
                   &mut querystr as (*mut stralloc),
                   q as (*const u8)
               ) == 0 {
                nomem();
            }
            if stralloc_cats(
                   &mut querystr as (*mut stralloc),
                   (*b":\0").as_ptr()
               ) == 0 {
                nomem();
            }
            if dns_domain_todot_cat(
                   &mut querystr as (*mut stralloc),
                   control as (*const u8)
               ) == 0 {
                nomem();
            }
            if stralloc_cats(
                   &mut querystr as (*mut stralloc),
                   (*b":\0").as_ptr()
               ) == 0 {
                nomem();
            }
            if stralloc_catb(
                   &mut querystr as (*mut stralloc),
                   ipstr.as_mut_ptr() as (*const u8),
                   ip4_fmt(ipstr.as_mut_ptr(),ip.as_mut_ptr() as (*const u8))
               ) == 0 {
                nomem();
            }
            if stralloc_cats(
                   &mut querystr as (*mut stralloc),
                   (*b":\0").as_ptr()
               ) == 0 {
                nomem();
            }
            buffer_put(buffer_1,querystr.s as (*const u8),querystr.len);
            buffer_puts(buffer_1,(*b"tx\n\0").as_ptr());
            buffer_flush(buffer_1);
            if resolve(q,type_.as_mut_ptr(),ip.as_mut_ptr()) == -1i32 {
                let mut x : *const u8 = error_str(errno);
                buffer_put(buffer_1,querystr.s as (*const u8),querystr.len);
                buffer_puts(buffer_1,(*b"ALERT:query failed; \0").as_ptr());
                buffer_puts(buffer_1,x);
                buffer_puts(buffer_1,(*b"\n\0").as_ptr());
            } else {
                parsepacket(
                    tx.packet as (*const u8),
                    tx.packetlen,
                    q as (*const u8),
                    type_.as_mut_ptr() as (*const u8),
                    control as (*const u8)
                );
            }
            if dns_domain_equal(
                   q as (*const u8),
                   (*b"\tlocalhost\0\0").as_ptr()
               ) != 0 {
                buffer_put(buffer_1,querystr.s as (*const u8),querystr.len);
                buffer_puts(
                    buffer_1,
                    (*b"ALERT:some caches do not handle localhost internally\n\0").as_ptr(
                    )
                );
                address_add(q as (*const u8),(*b"\x7F\0\0\x01\0").as_ptr());
            }
            if dd(q as (*const u8),(*b"\0").as_ptr(),ip.as_mut_ptr()) == 4i32 {
                buffer_put(buffer_1,querystr.s as (*const u8),querystr.len);
                buffer_puts(
                    buffer_1,
                    (*b"ALERT:some caches do not handle IP addresses internally\n\0").as_ptr(
                    )
                );
                address_add(q as (*const u8),ip.as_mut_ptr() as (*const u8));
            }
            buffer_flush(buffer_1);
        }
        i = i + 1;
    }
    _exit(0i32);
    0
}
