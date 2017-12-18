extern {
    fn alloc(n : u32) -> *mut u8;
    fn alloc_free(x : *mut u8);
    fn byte_copy(to : *mut u8, n : u32, from : *mut u8);
    fn byte_diff(s : *mut u8, n : u32, t : *mut u8) -> i32;
    fn byte_zero(s : *mut u8, n : u32);
    fn cache_init(arg1 : u32) -> i32;
    fn close(arg1 : i32) -> i32;
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
    fn dns_random_init(arg1 : *const u8);
    fn droproot(arg1 : *const u8);
    fn env_get(arg1 : *const u8) -> *mut u8;
    static mut errno : i32;
    static mut error_pipe : i32;
    static mut error_proto : i32;
    static mut error_timeout : i32;
    fn iopause(
        arg1 : *mut pollfd, arg2 : u32, arg3 : *mut taia, arg4 : *mut taia
    );
    fn ip4_scan(arg1 : *const u8, arg2 : *mut u8) -> u32;
    fn log_query(
        arg1 : *mut usize,
        arg2 : *const u8,
        arg3 : u32,
        arg4 : *const u8,
        arg5 : *const u8,
        arg6 : *const u8
    );
    fn log_querydone(arg1 : *mut usize, arg2 : u32);
    fn log_querydrop(arg1 : *mut usize);
    fn log_startup();
    fn log_tcpclose(arg1 : *const u8, arg2 : u32);
    fn log_tcpopen(arg1 : *const u8, arg2 : u32);
    fn ndelay_on(arg1 : i32) -> i32;
    fn okclient(arg1 : *mut u8) -> i32;
    fn query_forwardonly();
    fn query_get(
        arg1 : *mut query, arg2 : *mut pollfd, arg3 : *mut taia
    ) -> i32;
    fn query_io(
        arg1 : *mut query, arg2 : *mut pollfd, arg3 : *mut taia
    );
    fn query_start(
        arg1 : *mut query,
        arg2 : *mut u8,
        arg3 : *mut u8,
        arg4 : *mut u8,
        arg5 : *mut u8
    ) -> i32;
    fn read(
        arg1 : i32, arg2 : *mut ::std::os::raw::c_void, arg3 : usize
    ) -> isize;
    static mut response : *mut u8;
    fn response_hidettl();
    fn response_id(arg1 : *const u8);
    static mut response_len : u32;
    fn response_tc();
    fn roots_init() -> i32;
    fn scan_ulong(arg1 : *const u8, arg2 : *mut usize) -> u32;
    fn socket_accept4(
        arg1 : i32, arg2 : *mut u8, arg3 : *mut u16
    ) -> i32;
    fn socket_bind4_reuse(
        arg1 : i32, arg2 : *mut u8, arg3 : u16
    ) -> i32;
    fn socket_listen(arg1 : i32, arg2 : i32) -> i32;
    fn socket_recv4(
        arg1 : i32,
        arg2 : *mut u8,
        arg3 : i32,
        arg4 : *mut u8,
        arg5 : *mut u16
    ) -> i32;
    fn socket_send4(
        arg1 : i32,
        arg2 : *const u8,
        arg3 : i32,
        arg4 : *const u8,
        arg5 : u16
    ) -> i32;
    fn socket_tcp() -> i32;
    fn socket_tryreservein(arg1 : i32, arg2 : i32);
    fn socket_udp() -> i32;
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
    static mut strerr_sys : strerr;
    fn taia_add(
        arg1 : *mut taia, arg2 : *const taia, arg3 : *const taia
    );
    fn taia_less(arg1 : *const taia, arg2 : *const taia) -> i32;
    fn taia_now(arg1 : *mut taia);
    fn taia_uint(arg1 : *mut taia, arg2 : u32);
    fn uint16_pack_big(arg1 : *mut u8, arg2 : u16);
    fn write(
        __fd : i32, __buf : *const ::std::os::raw::c_void, __nbyte : usize
    ) -> isize;
}

static mut myipoutgoing : [u8; 4] = [0u8; 4];

static mut myipincoming : [u8; 4] = [0u8; 4];

static mut buf : [u8; 1024] = [0u8; 1024];

#[no_mangle]
pub static mut numqueries : usize = 0usize;

static mut udp53 : i32 = 0i32;

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

#[derive(Copy)]
#[repr(C)]
pub struct query {
    pub loop : u32,
    pub level : u32,
    pub name : [*mut u8; 5],
    pub control : [*mut u8; 5],
    pub ns : [[*mut u8; 16]; 5],
    pub servers : [[u8; 64]; 5],
    pub alias : [*mut u8; 16],
    pub aliasttl : [u32; 16],
    pub localip : [u8; 4],
    pub type_ : [u8; 2],
    pub class : [u8; 2],
    pub dt : dns_transmit,
}

impl Clone for query {
    fn clone(&self) -> Self { *self }
}

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

#[derive(Copy)]
#[repr(C)]
pub struct udpclient {
    pub q : query,
    pub start : taia,
    pub active : usize,
    pub io : *mut pollfd,
    pub ip : [u8; 4],
    pub port : u16,
    pub id : [u8; 2],
}

impl Clone for udpclient {
    fn clone(&self) -> Self { *self }
}

static mut u
    : [udpclient; 200]
    = [udpclient {
           q: query {
                  loop: 0u32,
                  level: 0u32,
                  name: [0 as (*mut u8); 5],
                  control: [0 as (*mut u8); 5],
                  ns: [[0 as (*mut u8); 16]; 5],
                  servers: [[0u8; 64]; 5],
                  alias: [0 as (*mut u8); 16],
                  aliasttl: [0u32; 16],
                  localip: [0u8; 4],
                  type_: [0u8; 2],
                  class: [0u8; 2],
                  dt: dns_transmit {
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
                      }
              },
           start: taia { sec: tai { x: 0usize }, nano: 0usize, atto: 0usize },
           active: 0usize,
           io: 0 as (*mut pollfd),
           ip: [0u8; 4],
           port: 0u16,
           id: [0u8; 2]
       }; 200];

#[no_mangle]
pub static mut uactive : i32 = 0i32;

#[no_mangle]
pub unsafe extern fn u_drop(mut j : i32) {
    if u[j as (usize)].active == 0 {
    } else {
        log_querydrop(&mut u[j as (usize)].active as (*mut usize));
        u[j as (usize)].active = 0usize;
        uactive = uactive - 1;
    }
}

#[no_mangle]
pub unsafe extern fn u_respond(mut j : i32) {
    if u[j as (usize)].active == 0 {
    } else {
        response_id(u[j as (usize)].id.as_mut_ptr() as (*const u8));
        if response_len > 512u32 {
            response_tc();
        }
        socket_send4(
            udp53,
            response as (*const u8),
            response_len as (i32),
            u[j as (usize)].ip.as_mut_ptr() as (*const u8),
            u[j as (usize)].port
        );
        log_querydone(
            &mut u[j as (usize)].active as (*mut usize),
            response_len
        );
        u[j as (usize)].active = 0usize;
        uactive = uactive - 1;
    }
}

unsafe extern fn packetquery(
    mut buf : *mut u8,
    mut len : u32,
    mut q : *mut *mut u8,
    mut qtype : *mut u8,
    mut qclass : *mut u8,
    mut id : *mut u8
) -> i32 {
    let mut pos : u32;
    let mut header : [u8; 12];
    errno = error_proto;
    pos = dns_packet_copy(
              buf as (*const u8),
              len,
              0u32,
              header.as_mut_ptr(),
              12u32
          );
    if pos == 0 {
        0i32
    } else if header[2usize] as (i32) & 128i32 != 0 {
        0i32
    } else if header[2usize] as (i32) & 1i32 == 0 {
        0i32
    } else if header[2usize] as (i32) & 120i32 != 0 {
        0i32
    } else if header[2usize] as (i32) & 2i32 != 0 {
        0i32
    } else if byte_diff(
                  header.as_mut_ptr().offset(4isize),
                  2u32,
                  (*b"\0\x01\0").as_ptr() as (*mut u8)
              ) != 0 {
        0i32
    } else {
        pos = dns_packet_getname(buf as (*const u8),len,pos,q);
        (if pos == 0 {
             0i32
         } else {
             pos = dns_packet_copy(buf as (*const u8),len,pos,qtype,2u32);
             (if pos == 0 {
                  0i32
              } else {
                  pos = dns_packet_copy(buf as (*const u8),len,pos,qclass,2u32);
                  (if pos == 0 {
                       0i32
                   } else if byte_diff(
                                 qclass,
                                 2u32,
                                 (*b"\0\x01\0").as_ptr() as (*mut u8)
                             ) != 0 && (byte_diff(
                                            qclass,
                                            2u32,
                                            (*b"\0\xFF\0").as_ptr() as (*mut u8)
                                        ) != 0) {
                       0i32
                   } else {
                       byte_copy(id,2u32,header.as_mut_ptr());
                       1i32
                   })
              })
         })
    }
}

#[no_mangle]
pub unsafe extern fn u_new() {
    let mut j : i32;
    let mut i : i32;
    let mut x : *mut udpclient;
    let mut len : i32;
    static mut q : *mut u8 = 0i32 as (*mut u8);
    let mut qtype : [u8; 2];
    let mut qclass : [u8; 2];
    j = 0i32;
    'loop1: loop {
        if !(j < 200i32) {
            break;
        }
        if u[j as (usize)].active == 0 {
            break;
        }
        j = j + 1;
    }
    if j >= 200i32 {
        j = 0i32;
        i = 1i32;
        'loop6: loop {
            if !(i < 200i32) {
                break;
            }
            if taia_less(
                   &mut u[i as (usize)].start as (*mut taia) as (*const taia),
                   &mut u[j as (usize)].start as (*mut taia) as (*const taia)
               ) != 0 {
                j = i;
            }
            i = i + 1;
        }
        errno = error_timeout;
        u_drop(j);
    }
    x = u.as_mut_ptr().offset(j as (isize));
    taia_now(&mut (*x).start as (*mut taia));
    len = socket_recv4(
              udp53,
              buf.as_mut_ptr(),
              ::std::mem::size_of::<[u8; 1024]>() as (i32),
              (*x).ip.as_mut_ptr(),
              &mut (*x).port as (*mut u16)
          );
    if len == -1i32 {
    } else if len as (usize) >= ::std::mem::size_of::<[u8; 1024]>() {
    } else {
        if (*x).port as (i32) < 1024i32 {
            if (*x).port as (i32) != 53i32 {
                return;
            }
        }
        (if okclient((*x).ip.as_mut_ptr()) == 0 {
         } else if packetquery(
                       buf.as_mut_ptr(),
                       len as (u32),
                       &mut q as (*mut *mut u8),
                       qtype.as_mut_ptr(),
                       qclass.as_mut_ptr(),
                       (*x).id.as_mut_ptr()
                   ) == 0 {
         } else {
             (*x).active = {
                               numqueries = numqueries.wrapping_add(1usize);
                               numqueries
                           };
             uactive = uactive + 1;
             log_query(
                 &mut (*x).active as (*mut usize),
                 (*x).ip.as_mut_ptr() as (*const u8),
                 (*x).port as (u32),
                 (*x).id.as_mut_ptr() as (*const u8),
                 q as (*const u8),
                 qtype.as_mut_ptr() as (*const u8)
             );
             let switch1
                 = query_start(
                       &mut (*x).q as (*mut query),
                       q,
                       qtype.as_mut_ptr(),
                       qclass.as_mut_ptr(),
                       myipoutgoing.as_mut_ptr()
                   );
             if switch1 == 1i32 {
                 u_respond(j);
             } else if switch1 == -1i32 {
                 u_drop(j);
                 return;
             }
         })
    }
}

static mut tcp53 : i32 = 0i32;

#[derive(Copy)]
#[repr(C)]
pub struct tcpclient {
    pub q : query,
    pub start : taia,
    pub timeout : taia,
    pub active : usize,
    pub io : *mut pollfd,
    pub ip : [u8; 4],
    pub port : u16,
    pub id : [u8; 2],
    pub tcp : i32,
    pub state : i32,
    pub buf : *mut u8,
    pub len : u32,
    pub pos : u32,
}

impl Clone for tcpclient {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut t
    : [tcpclient; 20]
    = [tcpclient {
           q: query {
                  loop: 0u32,
                  level: 0u32,
                  name: [0 as (*mut u8); 5],
                  control: [0 as (*mut u8); 5],
                  ns: [[0 as (*mut u8); 16]; 5],
                  servers: [[0u8; 64]; 5],
                  alias: [0 as (*mut u8); 16],
                  aliasttl: [0u32; 16],
                  localip: [0u8; 4],
                  type_: [0u8; 2],
                  class: [0u8; 2],
                  dt: dns_transmit {
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
                      }
              },
           start: taia { sec: tai { x: 0usize }, nano: 0usize, atto: 0usize },
           timeout: taia {
                        sec: tai { x: 0usize },
                        nano: 0usize,
                        atto: 0usize
                    },
           active: 0usize,
           io: 0 as (*mut pollfd),
           ip: [0u8; 4],
           port: 0u16,
           id: [0u8; 2],
           tcp: 0i32,
           state: 0i32,
           buf: 0 as (*mut u8),
           len: 0u32,
           pos: 0u32
       }; 20];

#[no_mangle]
pub static mut tactive : i32 = 0i32;

#[no_mangle]
pub unsafe extern fn t_free(mut j : i32) {
    if t[j as (usize)].buf.is_null() {
    } else {
        alloc_free(t[j as (usize)].buf);
        t[j as (usize)].buf = 0i32 as (*mut u8);
    }
}

#[no_mangle]
pub unsafe extern fn t_timeout(mut j : i32) {
    let mut now : taia;
    if t[j as (usize)].active == 0 {
    } else {
        taia_now(&mut now as (*mut taia));
        taia_uint(&mut t[j as (usize)].timeout as (*mut taia),10u32);
        taia_add(
            &mut t[j as (usize)].timeout as (*mut taia),
            &mut t[j as (usize)].timeout as (*mut taia) as (*const taia),
            &mut now as (*mut taia) as (*const taia)
        );
    }
}

#[no_mangle]
pub unsafe extern fn t_close(mut j : i32) {
    if t[j as (usize)].active == 0 {
    } else {
        t_free(j);
        log_tcpclose(
            t[j as (usize)].ip.as_mut_ptr() as (*const u8),
            t[j as (usize)].port as (u32)
        );
        close(t[j as (usize)].tcp);
        t[j as (usize)].active = 0usize;
        tactive = tactive - 1;
    }
}

#[no_mangle]
pub unsafe extern fn t_drop(mut j : i32) {
    log_querydrop(&mut t[j as (usize)].active as (*mut usize));
    errno = error_pipe;
    t_close(j);
}

#[no_mangle]
pub unsafe extern fn t_respond(mut j : i32) {
    if t[j as (usize)].active == 0 {
    } else {
        log_querydone(
            &mut t[j as (usize)].active as (*mut usize),
            response_len
        );
        response_id(t[j as (usize)].id.as_mut_ptr() as (*const u8));
        t[j as (usize)].len = response_len.wrapping_add(2u32);
        t_free(j);
        t[j as (usize)].buf = alloc(response_len.wrapping_add(2u32));
        (if t[j as (usize)].buf.is_null() {
             t_close(j);
         } else {
             uint16_pack_big(t[j as (usize)].buf,response_len as (u16));
             byte_copy(
                 t[j as (usize)].buf.offset(2isize),
                 response_len,
                 response
             );
             t[j as (usize)].pos = 0u32;
             t[j as (usize)].state = -1i32;
         })
    }
}

#[no_mangle]
pub unsafe extern fn t_rw(mut j : i32) {
    let mut x : *mut tcpclient;
    let mut ch : u8;
    static mut q : *mut u8 = 0i32 as (*mut u8);
    let mut qtype : [u8; 2];
    let mut qclass : [u8; 2];
    let mut r : i32;
    x = t.as_mut_ptr().offset(j as (isize));
    if (*x).state == -1i32 {
        r = write(
                (*x).tcp,
                (*x).buf.offset(
                    (*x).pos as (isize)
                ) as (*const ::std::os::raw::c_void),
                (*x).len.wrapping_sub((*x).pos) as (usize)
            ) as (i32);
        (if r <= 0i32 {
             t_close(j);
         } else {
             (*x).pos = (*x).pos.wrapping_add(r as (u32));
             if (*x).pos == (*x).len {
                 t_free(j);
                 (*x).state = 1i32;
             }
         })
    } else {
        r = read(
                (*x).tcp,
                &mut ch as (*mut u8) as (*mut ::std::os::raw::c_void),
                1usize
            ) as (i32);
        (if r == 0i32 {
             errno = error_pipe;
             t_close(j);
         } else if r < 0i32 {
             t_close(j);
         } else if (*x).state == 1i32 {
             (*x).len = ch as (u32);
             (*x).len = (*x).len << 8i32;
             (*x).state = 2i32;
         } else if (*x).state == 2i32 {
             (*x).len = (*x).len.wrapping_add(ch as (u32));
             (if (*x).len == 0 {
                  errno = error_proto;
                  t_close(j);
              } else {
                  (*x).buf = alloc((*x).len);
                  (if (*x).buf.is_null() {
                       t_close(j);
                   } else {
                       (*x).pos = 0u32;
                       (*x).state = 3i32;
                   })
              })
         } else if (*x).state != 3i32 {
         } else {
             *(*x).buf.offset(
                  {
                      let _old = (*x).pos;
                      (*x).pos = (*x).pos.wrapping_add(1u32);
                      _old
                  } as (isize)
              ) = ch;
             (if (*x).pos < (*x).len {
              } else if packetquery(
                            (*x).buf,
                            (*x).len,
                            &mut q as (*mut *mut u8),
                            qtype.as_mut_ptr(),
                            qclass.as_mut_ptr(),
                            (*x).id.as_mut_ptr()
                        ) == 0 {
                  t_close(j);
              } else {
                  (*x).active = {
                                    numqueries = numqueries.wrapping_add(1usize);
                                    numqueries
                                };
                  log_query(
                      &mut (*x).active as (*mut usize),
                      (*x).ip.as_mut_ptr() as (*const u8),
                      (*x).port as (u32),
                      (*x).id.as_mut_ptr() as (*const u8),
                      q as (*const u8),
                      qtype.as_mut_ptr() as (*const u8)
                  );
                  let switch2
                      = query_start(
                            &mut (*x).q as (*mut query),
                            q,
                            qtype.as_mut_ptr(),
                            qclass.as_mut_ptr(),
                            myipoutgoing.as_mut_ptr()
                        );
                  (if switch2 == 1i32 {
                       t_respond(j);
                   } else if switch2 == -1i32 {
                       t_drop(j);
                   } else {
                       t_free(j);
                       (*x).state = 0i32;
                   })
              })
         })
    }
}

#[no_mangle]
pub unsafe extern fn t_new() {
    let mut i : i32;
    let mut j : i32;
    let mut x : *mut tcpclient;
    j = 0i32;
    'loop1: loop {
        if !(j < 20i32) {
            break;
        }
        if t[j as (usize)].active == 0 {
            break;
        }
        j = j + 1;
    }
    if j >= 20i32 {
        j = 0i32;
        i = 1i32;
        'loop6: loop {
            if !(i < 20i32) {
                break;
            }
            if taia_less(
                   &mut t[i as (usize)].start as (*mut taia) as (*const taia),
                   &mut t[j as (usize)].start as (*mut taia) as (*const taia)
               ) != 0 {
                j = i;
            }
            i = i + 1;
        }
        errno = error_timeout;
        if t[j as (usize)].state == 0i32 {
            t_drop(j);
        } else {
            t_close(j);
        }
    }
    x = t.as_mut_ptr().offset(j as (isize));
    taia_now(&mut (*x).start as (*mut taia));
    (*x).tcp = socket_accept4(
                   tcp53,
                   (*x).ip.as_mut_ptr(),
                   &mut (*x).port as (*mut u16)
               );
    if (*x).tcp == -1i32 {
    } else {
        if (*x).port as (i32) < 1024i32 {
            if (*x).port as (i32) != 53i32 {
                close((*x).tcp);
                return;
            }
        }
        (if okclient((*x).ip.as_mut_ptr()) == 0 {
             close((*x).tcp);
         } else if ndelay_on((*x).tcp) == -1i32 {
             close((*x).tcp);
         } else {
             (*x).active = 1usize;
             tactive = tactive + 1;
             (*x).state = 1i32;
             t_timeout(j);
             log_tcpopen(
                 (*x).ip.as_mut_ptr() as (*const u8),
                 (*x).port as (u32)
             );
         })
    }
}

#[no_mangle]
pub static mut io
    : [pollfd; 223]
    = [pollfd { fd: 0i32, events: 0i16, revents: 0i16 }; 223];

#[no_mangle]
pub static mut udp53io : *mut pollfd = 0 as (*mut pollfd);

#[no_mangle]
pub static mut tcp53io : *mut pollfd = 0 as (*mut pollfd);

#[no_mangle]
pub static mut seed : [u8; 128] = [0u8; 128];

fn main() {
    let ret = unsafe { _c_main() };
    ::std::process::exit(ret);
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

unsafe extern fn doit() {
    let mut j : i32;
    let mut deadline : taia;
    let mut stamp : taia;
    let mut iolen : i32;
    let mut r : i32;
    'loop1: loop {
        taia_now(&mut stamp as (*mut taia));
        taia_uint(&mut deadline as (*mut taia),120u32);
        taia_add(
            &mut deadline as (*mut taia),
            &mut deadline as (*mut taia) as (*const taia),
            &mut stamp as (*mut taia) as (*const taia)
        );
        iolen = 0i32;
        udp53io = io.as_mut_ptr().offset(
                      {
                          let _old = iolen;
                          iolen = iolen + 1;
                          _old
                      } as (isize)
                  );
        (*udp53io).fd = udp53;
        (*udp53io).events = 0x1i16;
        tcp53io = io.as_mut_ptr().offset(
                      {
                          let _old = iolen;
                          iolen = iolen + 1;
                          _old
                      } as (isize)
                  );
        (*tcp53io).fd = tcp53;
        (*tcp53io).events = 0x1i16;
        j = 0i32;
        'loop2: loop {
            if !(j < 200i32) {
                break;
            }
            if u[j as (usize)].active != 0 {
                u[j as (usize)].io = io.as_mut_ptr().offset(
                                         {
                                             let _old = iolen;
                                             iolen = iolen + 1;
                                             _old
                                         } as (isize)
                                     );
                query_io(
                    &mut u[j as (usize)].q as (*mut query),
                    u[j as (usize)].io,
                    &mut deadline as (*mut taia)
                );
            }
            j = j + 1;
        }
        j = 0i32;
        'loop4: loop {
            if !(j < 20i32) {
                break;
            }
            if t[j as (usize)].active != 0 {
                t[j as (usize)].io = io.as_mut_ptr().offset(
                                         {
                                             let _old = iolen;
                                             iolen = iolen + 1;
                                             _old
                                         } as (isize)
                                     );
                if t[j as (usize)].state == 0i32 {
                    query_io(
                        &mut t[j as (usize)].q as (*mut query),
                        t[j as (usize)].io,
                        &mut deadline as (*mut taia)
                    );
                } else {
                    if taia_less(
                           &mut t[j as (usize)].timeout as (*mut taia) as (*const taia),
                           &mut deadline as (*mut taia) as (*const taia)
                       ) != 0 {
                        deadline = t[j as (usize)].timeout;
                    }
                    (*t[j as (usize)].io).fd = t[j as (usize)].tcp;
                    (*t[j as (usize)].io).events = if t[j as (usize)].state > 0i32 {
                                                       0x1i32
                                                   } else {
                                                       0x4i32
                                                   } as (i16);
                }
            }
            j = j + 1;
        }
        iopause(
            io.as_mut_ptr(),
            iolen as (u32),
            &mut deadline as (*mut taia),
            &mut stamp as (*mut taia)
        );
        j = 0i32;
        'loop6: loop {
            if !(j < 200i32) {
                break;
            }
            if u[j as (usize)].active != 0 {
                r = query_get(
                        &mut u[j as (usize)].q as (*mut query),
                        u[j as (usize)].io,
                        &mut stamp as (*mut taia)
                    );
                if r == -1i32 {
                    u_drop(j);
                }
                if r == 1i32 {
                    u_respond(j);
                }
            }
            j = j + 1;
        }
        j = 0i32;
        'loop8: loop {
            if !(j < 20i32) {
                break;
            }
            if t[j as (usize)].active != 0 {
                if (*t[j as (usize)].io).revents != 0 {
                    t_timeout(j);
                }
                if t[j as (usize)].state == 0i32 {
                    r = query_get(
                            &mut t[j as (usize)].q as (*mut query),
                            t[j as (usize)].io,
                            &mut stamp as (*mut taia)
                        );
                    if r == -1i32 {
                        t_drop(j);
                    }
                    if r == 1i32 {
                        t_respond(j);
                    }
                } else if (*t[j as (usize)].io).revents != 0 || taia_less(
                                                                    &mut t[  j as (usize)
                                                                         ].timeout as (*mut taia) as (*const taia),
                                                                    &mut stamp as (*mut taia) as (*const taia)
                                                                ) != 0 {
                    t_rw(j);
                }
            }
            j = j + 1;
        }
        if !udp53io.is_null() {
            if (*udp53io).revents != 0 {
                u_new();
            }
        }
        if tcp53io.is_null() {
            continue;
        }
        if (*tcp53io).revents == 0 {
            continue;
        }
        t_new();
    }
}

#[no_mangle]
pub unsafe extern fn _c_main() -> i32 {
    let mut x : *mut u8;
    let mut cachesize : usize;
    x = env_get((*b"IP\0").as_ptr());
    if x.is_null() {
        strerr_die(
            111i32,
            (*b"dnscache: fatal: \0").as_ptr(),
            (*b"$IP not set\0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const strerr)
        );
    }
    if ip4_scan(x as (*const u8),myipincoming.as_mut_ptr()) == 0 {
        strerr_die(
            111i32,
            (*b"dnscache: fatal: \0").as_ptr(),
            (*b"unable to parse IP address \0").as_ptr(),
            x as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const strerr)
        );
    }
    udp53 = socket_udp();
    if udp53 == -1i32 {
        strerr_die(
            111i32,
            (*b"dnscache: fatal: \0").as_ptr(),
            (*b"unable to create UDP socket: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if socket_bind4_reuse(
           udp53,
           myipincoming.as_mut_ptr(),
           53u16
       ) == -1i32 {
        strerr_die(
            111i32,
            (*b"dnscache: fatal: \0").as_ptr(),
            (*b"unable to bind UDP socket: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    tcp53 = socket_tcp();
    if tcp53 == -1i32 {
        strerr_die(
            111i32,
            (*b"dnscache: fatal: \0").as_ptr(),
            (*b"unable to create TCP socket: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if socket_bind4_reuse(
           tcp53,
           myipincoming.as_mut_ptr(),
           53u16
       ) == -1i32 {
        strerr_die(
            111i32,
            (*b"dnscache: fatal: \0").as_ptr(),
            (*b"unable to bind TCP socket: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    droproot((*b"dnscache: fatal: \0").as_ptr());
    socket_tryreservein(udp53,131072i32);
    byte_zero(
        seed.as_mut_ptr(),
        ::std::mem::size_of::<[u8; 128]>() as (u32)
    );
    read(
        0i32,
        seed.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        ::std::mem::size_of::<[u8; 128]>()
    );
    dns_random_init(seed.as_mut_ptr() as (*const u8));
    close(0i32);
    x = env_get((*b"IPSEND\0").as_ptr());
    if x.is_null() {
        strerr_die(
            111i32,
            (*b"dnscache: fatal: \0").as_ptr(),
            (*b"$IPSEND not set\0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const strerr)
        );
    }
    if ip4_scan(x as (*const u8),myipoutgoing.as_mut_ptr()) == 0 {
        strerr_die(
            111i32,
            (*b"dnscache: fatal: \0").as_ptr(),
            (*b"unable to parse IP address \0").as_ptr(),
            x as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const strerr)
        );
    }
    x = env_get((*b"CACHESIZE\0").as_ptr());
    if x.is_null() {
        strerr_die(
            111i32,
            (*b"dnscache: fatal: \0").as_ptr(),
            (*b"$CACHESIZE not set\0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const strerr)
        );
    }
    scan_ulong(x as (*const u8),&mut cachesize as (*mut usize));
    if cache_init(cachesize as (u32)) == 0 {
        strerr_die(
            111i32,
            (*b"dnscache: fatal: \0").as_ptr(),
            (*b"not enough memory for cache of size \0").as_ptr(),
            x as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const strerr)
        );
    }
    if !env_get((*b"HIDETTL\0").as_ptr()).is_null() {
        response_hidettl();
    }
    if !env_get((*b"FORWARDONLY\0").as_ptr()).is_null() {
        query_forwardonly();
    }
    if roots_init() == 0 {
        strerr_die(
            111i32,
            (*b"dnscache: fatal: \0").as_ptr(),
            (*b"unable to read servers: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if socket_listen(tcp53,20i32) == -1i32 {
        strerr_die(
            111i32,
            (*b"dnscache: fatal: \0").as_ptr(),
            (*b"unable to listen on TCP socket: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    log_startup();
    doit();
    0
}