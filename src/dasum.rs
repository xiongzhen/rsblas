/// sums the absolute values of the elements of an `f64` vector
pub fn dasum(n: isize, x: &[f64], incx: isize) -> f64 {
    if n <= 0 || incx <= 0 {
        return f64::NAN;
    }

    let n_usize: usize = n as usize;
    let incx_usize: usize = incx as usize;
    if x.len() < 1 + (n_usize - 1) * incx_usize {
        return f64::NAN;
    }

    let mut ix: usize = 0;
    let mut result: f64 = 0.0f64;
    for _ in 0 .. n_usize {
        let value = x[ix];

        result += if value >= 0.0 {
            value
        } else {
            -value
        };

        ix += incx_usize;
    }

    result
}