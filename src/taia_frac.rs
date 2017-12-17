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
pub unsafe extern fn taia_frac(mut t : *const taia) -> f64 {
    ((*t).atto as (f64) * 0.000000001f64 + (*t).nano as (f64)) * 0.000000001f64
}
