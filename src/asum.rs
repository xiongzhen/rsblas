use std::ops::{AddAssign};

trait ASumValue {
    fn a_sum_value(x: Self) -> Self;
}
impl ASumValue for f32 {
    fn a_sum_value(x: Self) -> Self {
        f32::abs(x)
    }
}
impl ASumValue for f64 {
    fn a_sum_value(x: Self) -> Self {
        f64::abs(x)
    }
}


fn asum<T>(n: isize, x: &[T], incx: isize) -> T
where T: Default + AddAssign + Copy + ASumValue,
{
    if n <= 0 || incx <= 0 {
        return <T as Default>::default();
    }

    let n_usize: usize = n as usize;
    let incx_usize: usize = incx as usize;
    if x.len() < 1 + (n_usize - 1) * incx_usize {
        return <T as Default>::default();
    }

    let mut ix: usize = 0;
    let mut result: T = <T as Default>::default();
    for _ in 0 .. n_usize {
        let value = x[ix];

        result += <T as ASumValue>::a_sum_value(value);

        ix += incx_usize;
    }

    result
}

/// sums the absolute values of the elements of an `f32` vector
pub fn sasum(n: isize, x: &[f32], incx: isize) -> f32 {
    asum::<f32>(n, x, incx)
}

/// sums the absolute values of the elements of an `f64` vector
pub fn dasum(n: isize, x: &[f64], incx: isize) -> f64 {
    asum::<f64>(n, x, incx)
}