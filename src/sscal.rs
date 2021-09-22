/// Scales a vector by a constant.
pub fn sscal(n: isize, a: f32, x: &mut [f32], incx: isize) -> bool {
    if n <= 0 || incx <= 0 {
        return false;
    }

    if x.len() < 1 + ((n as usize) - 1) * (incx as usize) {
        return false;
    }

    if a == 1.0_f32 {
        return true;
    }

    let mut ix: usize = 0;

    let n_usize = n as usize;
    let incx_usize = incx as usize;
    for _ in 0 .. n_usize {
        x[ix] *= a;
        ix += incx_usize;
    }

    return true;
}