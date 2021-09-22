use crate::utils::{check_inc, get_first_index};

fn swap<T>(n: isize, x: &mut [T], incx: isize, y: &mut [T], incy: isize) -> bool
where T: Default + Copy,
{
    if check_inc(n, x, incx) != true || check_inc(n, y, incy) != true {
        return false;
    }

    let n_usize = n as usize;
    if incx == 1 && incy == 1 {
        for i in 0 .. n_usize {
            let temp = x[i];
            x[i] = y[i];
            y[i] = temp;
        }
        return true;
    }

    let incx_abs: usize = if incx > 0 {incx as usize} else {(-incx) as usize};
    let mut ix: usize = get_first_index(n_usize, incx);
    
    let incy_abs: usize = if incy > 0 {incy as usize} else {(-incy) as usize};
    let mut iy: usize = get_first_index(n_usize, incy);

    for _ in 0 .. n_usize {
        let temp = x[ix];
        x[ix] = y[iy];
        y[iy] = temp;

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

/// swaps two `f32` vectors, it interchanges n values of vector `x` and vector `y`
pub fn sswap(n: isize, x: &mut [f32], incx: isize, y: &mut [f32], incy: isize) -> bool {
    swap::<f32>(n, x, incx, y, incy)
}

/// swaps two `f64` vectors, it interchanges n values of vector `x` and vector `y`
pub fn dswap(n: isize, x: &mut [f64], incx: isize, y: &mut [f64], incy: isize) -> bool {
    swap::<f64>(n, x, incx, y, incy)
}