use std::ops::{Add, Mul, AddAssign};
use crate::utils::{check_inc, get_first_index};

trait AxpyQuickReturn {
    fn quick_return(a: Self) -> bool;
}
impl AxpyQuickReturn for f32 {
    fn quick_return(a: Self) -> bool {
        a == 0.0_f32
    }
}
impl AxpyQuickReturn for f64 {
    fn quick_return(a: Self) -> bool {
        a == 0.0_f64
    }
}


fn axpy<T>(n: isize, a: T, x: &[T], incx: isize, y: &mut [T], incy: isize) -> bool
where T: Default + Add<Output = T> + AddAssign + Mul<Output = T> + Copy + AxpyQuickReturn,
{
    if check_inc(n, x, incx) != true || check_inc(n, y, incy) != true {
        return false;
    }

    if <T as AxpyQuickReturn>::quick_return(a) {
        return true;
    }

    let n_usize = n as usize;
    if incx == 1 && incy == 1 {
        for i in 0 .. n_usize {
            y[i] += (a * x[i]);
        }
        return true;
    }

    let incx_abs: usize = if incx > 0 {incx as usize} else {(-incx) as usize};
    let mut ix: usize = get_first_index(n_usize, incx);
    
    let incy_abs: usize = if incy > 0 {incy as usize} else {(-incy) as usize};
    let mut iy: usize = get_first_index(n_usize, incy);

    for _ in 0 .. n_usize {
        y[iy] += a * x[ix];
        
        ix = if incx > 0 {
            ix + incx_abs
        } else {
            ix - incx_abs
        };
        iy = if incy > 0 {
            iy + incy_abs
        } else {
            iy - incy_abs
        };
    }

    true
}

/// adds a scalar multiple of an `f32` vector to another `f32` vector
pub fn saxpy(n: isize, a: f32, x: &[f32], incx: isize, y: &mut [f32], incy: isize) -> bool {
    axpy::<f32>(n, a, x, incx, y, incy)
}

/// adds a scalar multiple of an `f64` vector to another `f64` vector
pub fn daxpy(n: isize, a: f64, x: &[f64], incx: isize, y: &mut [f64], incy: isize) -> bool {
    axpy::<f64>(n, a, x, incx, y, incy)
}