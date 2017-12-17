extern {
    fn taia_frac(arg1 : *const taia) -> f64;
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

#[no_mangle]
pub unsafe extern fn taia_approx(mut t : *const taia) -> f64 {
    (*(&(*t).sec as (*const tai))).x as (f64) + taia_frac(t)
}
