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
pub unsafe extern "C" fn taia_sub(mut t: *mut taia, mut u: *const taia, mut v: *const taia) {
    let mut unano: usize = (*u).nano;
    let mut uatto: usize = (*u).atto;
    (*t).sec.x = (*u).sec.x.wrapping_sub((*v).sec.x);
    (*t).nano = unano.wrapping_sub((*v).nano);
    (*t).atto = uatto.wrapping_sub((*v).atto);
    if (*t).atto > uatto {
        (*t).atto = (*t).atto.wrapping_add(1000000000usize);
        (*t).nano = (*t).nano.wrapping_sub(1usize);
    }
    if (*t).nano > unano {
        (*t).nano = (*t).nano.wrapping_add(1000000000usize);
        (*t).sec.x = (*t).sec.x.wrapping_sub(1usize);
    }
}
