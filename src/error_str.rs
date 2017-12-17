extern {
    static mut error_acces : i32;
    static mut error_again : i32;
    static mut error_connrefused : i32;
    static mut error_exist : i32;
    static mut error_inprogress : i32;
    static mut error_intr : i32;
    static mut error_io : i32;
    static mut error_isdir : i32;
    static mut error_nodevice : i32;
    static mut error_noent : i32;
    static mut error_nomem : i32;
    static mut error_perm : i32;
    static mut error_pipe : i32;
    static mut error_proto : i32;
    static mut error_timeout : i32;
    static mut error_txtbsy : i32;
    static mut error_wouldblock : i32;
}

#[no_mangle]
pub unsafe extern fn error_str(mut i : i32) -> *const u8 {
    if i == 0i32 {
        (*b"no error\0").as_ptr()
    } else if i == error_intr {
        (*b"interrupted system call\0").as_ptr()
    } else if i == error_nomem {
        (*b"out of memory\0").as_ptr()
    } else if i == error_noent {
        (*b"file does not exist\0").as_ptr()
    } else if i == error_txtbsy {
        (*b"text busy\0").as_ptr()
    } else if i == error_io {
        (*b"input/output error\0").as_ptr()
    } else if i == error_exist {
        (*b"file already exists\0").as_ptr()
    } else if i == error_timeout {
        (*b"timed out\0").as_ptr()
    } else if i == error_inprogress {
        (*b"operation in progress\0").as_ptr()
    } else if i == error_again {
        (*b"temporary failure\0").as_ptr()
    } else if i == error_wouldblock {
        (*b"input/output would block\0").as_ptr()
    } else if i == error_pipe {
        (*b"broken pipe\0").as_ptr()
    } else if i == error_perm {
        (*b"permission denied\0").as_ptr()
    } else if i == error_acces {
        (*b"access denied\0").as_ptr()
    } else if i == error_nodevice {
        (*b"device not configured\0").as_ptr()
    } else if i == error_proto {
        (*b"protocol error\0").as_ptr()
    } else if i == error_isdir {
        (*b"is a directory\0").as_ptr()
    } else if i == error_connrefused {
        (*b"connection refused\0").as_ptr()
    } else if i == 3i32 {
        (*b"no such process\0").as_ptr()
    } else if i == 7i32 {
        (*b"argument list too long\0").as_ptr()
    } else if i == 8i32 {
        (*b"exec format error\0").as_ptr()
    } else if i == 9i32 {
        (*b"file descriptor not open\0").as_ptr()
    } else if i == 10i32 {
        (*b"no child processes\0").as_ptr()
    } else if i == 11i32 {
        (*b"operation would cause deadlock\0").as_ptr()
    } else if i == 14i32 {
        (*b"bad address\0").as_ptr()
    } else if i == 15i32 {
        (*b"not a block device\0").as_ptr()
    } else if i == 16i32 {
        (*b"device busy\0").as_ptr()
    } else if i == 18i32 {
        (*b"cross-device link\0").as_ptr()
    } else if i == 19i32 {
        (*b"device does not support operation\0").as_ptr()
    } else if i == 20i32 {
        (*b"not a directory\0").as_ptr()
    } else if i == 22i32 {
        (*b"invalid argument\0").as_ptr()
    } else if i == 23i32 {
        (*b"system cannot open more files\0").as_ptr()
    } else if i == 24i32 {
        (*b"process cannot open more files\0").as_ptr()
    } else if i == 25i32 {
        (*b"not a tty\0").as_ptr()
    } else if i == 27i32 {
        (*b"file too big\0").as_ptr()
    } else if i == 28i32 {
        (*b"out of disk space\0").as_ptr()
    } else if i == 29i32 {
        (*b"unseekable descriptor\0").as_ptr()
    } else if i == 30i32 {
        (*b"read-only file system\0").as_ptr()
    } else if i == 31i32 {
        (*b"too many links\0").as_ptr()
    } else if i == 33i32 {
        (*b"input out of range\0").as_ptr()
    } else if i == 34i32 {
        (*b"output out of range\0").as_ptr()
    } else if i == 37i32 {
        (*b"operation already in progress\0").as_ptr()
    } else if i == 38i32 {
        (*b"not a socket\0").as_ptr()
    } else if i == 39i32 {
        (*b"destination address required\0").as_ptr()
    } else if i == 40i32 {
        (*b"message too long\0").as_ptr()
    } else if i == 41i32 {
        (*b"incorrect protocol type\0").as_ptr()
    } else if i == 42i32 {
        (*b"protocol not available\0").as_ptr()
    } else if i == 43i32 {
        (*b"protocol not supported\0").as_ptr()
    } else if i == 44i32 {
        (*b"socket type not supported\0").as_ptr()
    } else if i == 102i32 {
        (*b"operation not supported\0").as_ptr()
    } else if i == 46i32 {
        (*b"protocol family not supported\0").as_ptr()
    } else if i == 47i32 {
        (*b"address family not supported\0").as_ptr()
    } else if i == 48i32 {
        (*b"address already used\0").as_ptr()
    } else if i == 49i32 {
        (*b"address not available\0").as_ptr()
    } else if i == 50i32 {
        (*b"network down\0").as_ptr()
    } else if i == 51i32 {
        (*b"network unreachable\0").as_ptr()
    } else if i == 52i32 {
        (*b"network reset\0").as_ptr()
    } else if i == 53i32 {
        (*b"connection aborted\0").as_ptr()
    } else if i == 54i32 {
        (*b"connection reset\0").as_ptr()
    } else if i == 55i32 {
        (*b"out of buffer space\0").as_ptr()
    } else if i == 56i32 {
        (*b"already connected\0").as_ptr()
    } else if i == 57i32 {
        (*b"not connected\0").as_ptr()
    } else if i == 58i32 {
        (*b"socket shut down\0").as_ptr()
    } else if i == 59i32 {
        (*b"too many references\0").as_ptr()
    } else if i == 62i32 {
        (*b"symbolic link loop\0").as_ptr()
    } else if i == 63i32 {
        (*b"file name too long\0").as_ptr()
    } else if i == 64i32 {
        (*b"host down\0").as_ptr()
    } else if i == 65i32 {
        (*b"host unreachable\0").as_ptr()
    } else if i == 66i32 {
        (*b"directory not empty\0").as_ptr()
    } else if i == 67i32 {
        (*b"too many processes\0").as_ptr()
    } else if i == 68i32 {
        (*b"too many users\0").as_ptr()
    } else if i == 69i32 {
        (*b"disk quota exceeded\0").as_ptr()
    } else if i == 70i32 {
        (*b"stale NFS file handle\0").as_ptr()
    } else if i == 71i32 {
        (*b"too many levels of remote in path\0").as_ptr()
    } else if i == 72i32 {
        (*b"RPC structure is bad\0").as_ptr()
    } else if i == 73i32 {
        (*b"RPC version mismatch\0").as_ptr()
    } else if i == 74i32 {
        (*b"RPC program unavailable\0").as_ptr()
    } else if i == 75i32 {
        (*b"program version mismatch\0").as_ptr()
    } else if i == 76i32 {
        (*b"bad procedure for program\0").as_ptr()
    } else if i == 77i32 {
        (*b"no locks available\0").as_ptr()
    } else if i == 78i32 {
        (*b"system call not available\0").as_ptr()
    } else if i == 79i32 {
        (*b"bad file type\0").as_ptr()
    } else if i == 80i32 {
        (*b"authentication error\0").as_ptr()
    } else if i == 81i32 {
        (*b"not authenticated\0").as_ptr()
    } else if i == 99i32 {
        (*b"not a stream device\0").as_ptr()
    } else if i == 101i32 {
        (*b"timer expired\0").as_ptr()
    } else if i == 98i32 {
        (*b"out of stream resources\0").as_ptr()
    } else if i == 91i32 {
        (*b"no message of desired type\0").as_ptr()
    } else if i == 94i32 {
        (*b"bad message type\0").as_ptr()
    } else if i == 90i32 {
        (*b"identifier removed\0").as_ptr()
    } else if i == 97i32 {
        (*b"link severed\0").as_ptr()
    } else if i == 95i32 {
        (*b"multihop attempted\0").as_ptr()
    } else {
        (*b"unknown error\0").as_ptr()
    }
}
