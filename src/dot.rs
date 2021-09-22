use std::ops::{Add, Mul, AddAssign};
use crate::utils::{check_inc, get_first_index};

fn dot<T>(n: isize, x: &[T], incx: isize, y: &[T], incy: isize) -> T
where T: Default + Mul<Output = T> + AddAssign + Copy,
{
    if n <= 0 {
        return <T as Default>::default();
    }

    if check_inc(n, x, incx) != true || check_inc(n, y, incy) != true {
        return <T as Default>::default();
    }

    let mut result: T = <T as Default>::default();

    let n_usize = n as usize;
    if incx == 1 && incy == 1 {
        for i in 0 .. n_usize {
            result += x[i] * y[i];
        }
        return result;
    }

    let incx_abs = if incx > 0 {incx as usize} else {(-incx) as usize};
    let mut ix: usize = get_first_index(n_usize, incx);

    let incy_abs = if incy > 0 {incy as usize} else {(-incy) as usize};
    let mut iy: usize = get_first_index(n_usize, incy);

    for _ in 0 .. n_usize {
        result += x[ix] * y[iy];

        ix = if incx > 0 {ix + incx_abs} else {ix - incx_abs};
        iy = if incy > 0 {iy + incy_abs} else {iy - incy_abs};
    }

    result

}

/// computes a dot product (inner product) of two `f32` vectors
pub fn sdot(n: isize, x: &[f32], incx: isize, y: &[f32], incy: isize) -> f32 {
    dot::<f32>(n, x, incx, y, incy)
}

/// computes a dot product (inner product) of two `f64` vectors
pub fn ddot(n: isize, x: &[f64], incx: isize, y: &[f64], incy: isize) -> f64 {
    dot::<f64>(n, x, incx, y, incy)
}