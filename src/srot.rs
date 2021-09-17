use std::arch::x86_64::*;

/// Applies an `f32` plane rotation to 2 _n_-element `f32` vectors: `x` and `y`, with respective strides `incx` and `incy`.
pub fn srot(n: usize, x: &mut [f32], incx: usize, y: &mut [f32], incy: usize, c: f32, s: f32) -> bool {
    if n == 0 {
        return true;
    }

    if x.len() < 1 + (n - 1) * incx {
        return false;
    }
    if y.len() < 1 + (n - 1) * incy {
        return false;
    }

    if incx == 1 && incy == 1 {
        
        let mut offset_x = 0;
        let mut offset_y = 0;

        let mut n_8: usize = 0;
        if std::is_x86_feature_detected!("avx") {
            n_8 = n / 8;

            let c_packed: __m256 = unsafe { _mm256_set_dup_ps!(c) };
            let s_packed: __m256 = unsafe { _mm256_set_dup_ps!(s) };

            for _ in 0 .. n_8 {
                unsafe {
                    let x_packed: __m256 = _mm256_set_slice_ps!(x[offset_x..], incx);
                    let y_packed: __m256 = _mm256_set_slice_ps!(y[offset_y..], incy);

                    let c_x: __m256 = _mm256_mul_ps(c_packed, x_packed);
                    let c_y: __m256 = _mm256_mul_ps(c_packed, y_packed);
                    let s_x: __m256 = _mm256_mul_ps(s_packed, x_packed);
                    let s_y: __m256 = _mm256_mul_ps(s_packed, y_packed);

                    let s_temp: __m256 = _mm256_add_ps(c_x, s_y);
                    let y_temp: __m256 = _mm256_sub_ps(c_y, s_x);

                    _mm256_get_ps!(y[offset_x..], incx, y_temp);
                    _mm256_get_ps!(x[offset_y..], incy, s_temp);
                }

                offset_x += 8 * incx;
                offset_y += 8 * incy;
            }
        }

        for _ in (n_8 * 8) .. n {
            let s_temp = c * x[offset_x] + s * y[offset_y];
            y[offset_y] = c * y[offset_y] - s * x[offset_x];
            x[offset_x] = s_temp;

            offset_x += incx;
            offset_y += incy;
        }
    }

    true
}