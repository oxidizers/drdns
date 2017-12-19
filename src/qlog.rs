use buffer::Buffer;

extern "C" {
    static mut buffer_2: *mut Buffer;
}

unsafe extern "C" fn put(mut c: u8) {
    buffer_put(buffer_2, &mut c as (*mut u8) as (*const u8), 1u32);
}

unsafe extern "C" fn hex(mut c: u8) {
    put(
        (*b"0123456789abcdef\0")[(c as (i32) >> 4i32 & 15i32) as (usize)],
    );
    put((*b"0123456789abcdef\0")[(c as (i32) & 15i32) as (usize)]);
}

unsafe extern "C" fn octal(mut c: u8) {
    put(b'\\');
    put((b'0' as (i32) + (c as (i32) >> 6i32 & 7i32)) as (u8));
    put((b'0' as (i32) + (c as (i32) >> 3i32 & 7i32)) as (u8));
    put((b'0' as (i32) + (c as (i32) & 7i32)) as (u8));
}

#[no_mangle]
pub unsafe extern "C" fn qlog(
    mut ip: *const u8,
    mut port: u16,
    mut id: *const u8,
    mut q: *const u8,
    mut qtype: *const u8,
    mut result: *const u8,
) {
    let mut ch: u8;
    let mut ch2: u8;
    hex(*ip.offset(0isize));
    hex(*ip.offset(1isize));
    hex(*ip.offset(2isize));
    hex(*ip.offset(3isize));
    put(b':');
    hex((port as (i32) >> 8i32) as (u8));
    hex((port as (i32) & 255i32) as (u8));
    put(b':');
    hex(*id.offset(0isize));
    hex(*id.offset(1isize));
    buffer_puts(buffer_2, result);
    hex(*qtype.offset(0isize));
    hex(*qtype.offset(1isize));
    put(b' ');
    if *q == 0 {
        put(b'.');
    } else {
        'loop1: loop {
            ch = *{
                let _old = q;
                q = q.offset(1isize);
                _old
            };
            'loop2: loop {
                if {
                    let _old = ch;
                    ch = (ch as (i32) - 1) as (u8);
                    _old
                } == 0
                {
                    break;
                }
                ch2 = *{
                    let _old = q;
                    q = q.offset(1isize);
                    _old
                };
                if ch2 as (i32) >= b'A' as (i32) && (ch2 as (i32) <= b'Z' as (i32)) {
                    ch2 = (ch2 as (i32) + 32i32) as (u8);
                }
                if ch2 as (i32) >= b'a' as (i32) && (ch2 as (i32) <= b'z' as (i32)) ||
                    ch2 as (i32) >= b'0' as (i32) && (ch2 as (i32) <= b'9' as (i32)) ||
                    ch2 as (i32) == b'-' as (i32) ||
                    ch2 as (i32) == b'_' as (i32)
                {
                    put(ch2);
                } else {
                    octal(ch2);
                }
            }
            if *q == 0 {
                break;
            }
            put(b'.');
        }
    }
    put(b'\n');
    buffer_flush(buffer_2);
}
