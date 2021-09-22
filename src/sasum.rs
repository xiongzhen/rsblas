/// sums the absolute values of the elements of an `f32` vector
pub fn sasum(n: isize, x: &[f32], incx: isize) -> f32 {
    if n <= 0 || incx <= 0 {
        return f32::NAN;
    }

    let n_usize: usize = n as usize;
    let incx_usize: usize = incx as usize;
    if x.len() < 1 + (n_usize - 1) * incx_usize {
        return f32::NAN;
    }

    let mut ix: usize = 0;
    let mut result: f32 = 0.0f32;
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