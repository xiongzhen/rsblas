use std::ops::{MulAssign};

trait ScalQuickReturn {
    fn quick_return(a: Self) -> bool;
}

impl ScalQuickReturn for f32 {
    fn quick_return(a: Self) -> bool {
        a == 1.0_f32
    }
}
impl ScalQuickReturn for f64 {
    fn quick_return(a: Self) -> bool {
        a == 1.0_f64
    }
}

fn scal<T>(n: isize, a: T, x: &mut [T], incx: isize) -> bool
where T: Default + MulAssign + Copy + ScalQuickReturn,
{
    if n <= 0 || incx <= 0 {
        return false;
    }

    if x.len() < 1 + ((n as usize) - 1) * (incx as usize) {
        return false;
    }

    if <T as ScalQuickReturn>::quick_return(a) {
        return true;
    }

    let mut ix: usize = 0;

    let n_usize = n as usize;
    let incx_usize = incx as usize;
    for _ in 0 .. n_usize {
        x[ix] *= a;
        ix += incx_usize;
    }

    true
}

/// Scales an `f32` vector by a constant.
pub fn sscal(n: isize, a: f32, x: &mut [f32], incx: isize) -> bool {
    scal::<f32>(n, a, x, incx)
}

/// Scales an `f64` vector by a constant.
pub fn dscal(n: isize, a: f64, x: &mut [f64], incx: isize) -> bool {
    scal::<f64>(n, a, x, incx)
}