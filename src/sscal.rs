use std::arch::x86_64::*;

#[target_feature(enable="avx")]
pub unsafe fn sscal(n: usize, a: f32, x: &mut [f32], incx: usize) -> bool {
    if n == 0 {
        return true;
    }

    if incx == 0 {
        return false;
    }
    if x.len() < 1 + (n - 1) * incx {
        return false;
    }

    if a == 1.0_f32 {
        return true;
    }

    let mut offset_x: usize = 0;

    let mut n_8 = 0_usize;
    if std::is_x86_feature_detected!("avx") {
        n_8 = n / 8;
        let a_packed: __m256 = _mm256_set1_ps(a);
        for _ in 0..n_8 {
            let x_packed: __m256 = _mm256_set_slice_ps!(x[offset_x ..], incx);
            let ax: __m256 = _mm256_mul_ps(a_packed, x_packed);
            _mm256_get_ps!(x[offset_x ..], incx, ax);

            offset_x += 8;
        }
    }

    for _ in (n_8 * 8) .. n {
        x[offset_x] *= a;
        offset_x += incx;
    }

    return true;
}