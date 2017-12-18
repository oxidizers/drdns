extern {
    fn lseek(arg1 : i32, arg2 : isize, arg3 : i32) -> isize;
}

#[no_mangle]
pub unsafe extern fn seek_set(mut fd : i32, mut pos : usize) -> i32 {
    if lseek(fd,pos as (isize),0i32) == -1isize { -1i32 } else { 0i32 }
}
