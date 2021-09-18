use std::arch::x86_64::*;

/// Applies an `f64` plane rotation to 2 _n_-element `f64` vectors: `x` and `y`, with respective strides `incx` and `incy`.
pub fn drot(n: usize, x: &mut [f64], incx: usize, y: &mut [f64], incy: usize, c: f64, s: f64) -> bool {
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

        let mut n_4: usize = 0;
        if std::is_x86_feature_detected!("avx") {
            n_4 = n / 4;

            let c_packed: __m256d = unsafe { _mm256_set_dup_pd!(c) };
            let s_packed: __m256d = unsafe { _mm256_set_dup_pd!(s) };

            for _ in 0 .. n_4 {
                unsafe {
                    let x_packed: __m256d = _mm256_set_slice_pd!(x[offset_x..], incx);
                    let y_packed: __m256d = _mm256_set_slice_pd!(y[offset_y..], incy);

                    let c_x: __m256d = _mm256_mul_pd(c_packed, x_packed);
                    let c_y: __m256d = _mm256_mul_pd(c_packed, y_packed);
                    let s_x: __m256d = _mm256_mul_pd(s_packed, x_packed);
                    let s_y: __m256d = _mm256_mul_pd(s_packed, y_packed);

                    let d_temp: __m256d = _mm256_add_pd(c_x, s_y);
                    let s_temp: __m256d = _mm256_sub_pd(c_y, s_x);

                    _mm256_get_pd!(y[offset_x..], incx, s_temp);
                    _mm256_get_pd!(x[offset_y..], incy, d_temp);
                }

                offset_x += 4 * incx;
                offset_y += 4 * incy;
            }
        }

        for _ in (n_4 * 4) .. n {
            let d_temp = c * x[offset_x] + s * y[offset_y];
            y[offset_y] = c * y[offset_y] - s * x[offset_x];
            x[offset_x] = d_temp;

            offset_x += incx;
            offset_y += incy;
        }
    }

    true
}