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
pub unsafe extern "C" fn taia_less(mut t: *const taia, mut u: *const taia) -> i32 {
    if (*t).sec.x < (*u).sec.x {
        1i32
    } else if (*t).sec.x > (*u).sec.x {
        0i32
    } else if (*t).nano < (*u).nano {
        1i32
    } else if (*t).nano > (*u).nano {
        0i32
    } else {
        ((*t).atto < (*u).atto) as (i32)
    }
}
