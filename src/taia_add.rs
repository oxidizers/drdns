#[derive(Copy)]
#[repr(C)]
pub struct tai {
    pub x: usize,
}

impl Clone for tai {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct taia {
    pub sec: tai,
    pub nano: usize,
    pub atto: usize,
}

impl Clone for taia {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn taia_add(mut t: *mut taia, mut u: *const taia, mut v: *const taia) {
    (*t).sec.x = (*u).sec.x.wrapping_add((*v).sec.x);
    (*t).nano = (*u).nano.wrapping_add((*v).nano);
    (*t).atto = (*u).atto.wrapping_add((*v).atto);
    if (*t).atto > 999999999usize {
        (*t).atto = (*t).atto.wrapping_sub(1000000000usize);
        (*t).nano = (*t).nano.wrapping_add(1usize);
    }
    if (*t).nano > 999999999usize {
        (*t).nano = (*t).nano.wrapping_sub(1000000000usize);
        (*t).sec.x = (*t).sec.x.wrapping_add(1usize);
    }
}
