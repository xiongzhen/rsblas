use std::arch::x86_64::*;

/// Applies an `f64` plane rotation to 2 _n_-element `f64` vectors: `x` and `y`, with respective strides `incx` and `incy`.
/// 
/// <link rel="stylesheet"
/// href="https://cdn.jsdelivr.net/npm/katex@0.10.0/dist/katex.min.css"
/// integrity="sha384-9eLZqc9ds8eNjO3TmqPeYcDj8n+Qfa4nuSiGYa6DjLNcv9BtN69ZIulL9+8CqC9Y"
/// crossorigin="anonymous">
/// <script src="https://cdn.jsdelivr.net/npm/katex@0.10.0/dist/katex.min.js"
///   integrity="sha384-K3vbOmF2BtaVai+Qk37uypf7VrgBubhQreNQe9aGsz9lB63dIFiQVlJbr92dw2Lx"
///   crossorigin="anonymous"></script>
/// <script src="https://cdn.jsdelivr.net/npm/katex@0.10.0/dist/contrib/auto-render.min.js"
///   integrity="sha384-kmZOZB5ObwgQnS/DuDg6TScgOiWWBiVt0plIRkZCmE6rDZGrEOQeHM5PcHi+nyqe"
///   crossorigin="anonymous"></script>
/// <script>
/// document.addEventListener("DOMContentLoaded", function() {
///   renderMathInElement(document.body, {
///       delimiters: [
///           {left: "$$", right: "$$", display: true},
///           {left: "\\(", right: "\\)", display: false},
///           {left: "$", right: "$", display: false},
///           {left: "\\[", right: "\\]", display: true}
///       ]
///   });
/// });
/// </script>
/// 
/// $$ \\begin{bmatrix}
///      x^\\prime \\\\ y^\\prime
///    \\end{bmatrix} \leftarrow
///    \\begin{bmatrix}
///      c & s \\\\
///     -s & c
///    \\end{bmatrix}
///    \\cdot
///    \\begin{bmatrix}
///      x \\\\ y
///    \\end{bmatrix}
/// $$
/// 
/// - `n: usize`<br>Number of planar points, in `x` and `y`, to be rotated.
///   - _on entry_:. if `n = 0`, this function returns immediately.
/// 
/// - `x: &mut [f64]`<br>Array of dimension at least `(n - 1) * incx + 1`.
///    - _on entry_: the _n_-elements are `x[i * incx] for i = 0..n`
///    - _on exit_: the rotated values are updated in-place.
/// 
/// - `incx: usize`<br>Increment between elements of `x` as input and output.
///   - _on entry_: if `incx == 0`, this function returns immediately.
/// 
/// - `y: &mut [f64]`<br>Array of dimension at least `(n - 1) * incy + 1`.
///   - _on entry_: the _n_-elements are `y[i * incy] for i = 0..n`
///   - _on exit_: the rotated values are updated in-place.
/// 
/// - `incy: usize`<br>Increment between elements of `y` as input and output.
///   - _on entry_: if `incy = 0`, this function returns immediately.
/// 
/// - `c: f64`<br>Cosine of the angle of rotation.
/// 
/// - `s: f64`<br>Sine of the angle of rotation.
/// 
/// If coefficients `c` and `s` satisfy $c^2+s^2=1$, the rotation matrix is orthogonal, and the transformation is called a Givens plane rotation.
/// 
/// If `c = 1` and `s = 0`, this function returns immediately.
/// 
/// Reference:
/// 1. [https://www.hpc.nec/documents/sdk/SDK_NLC/UsersGuide/man/drot.html](https://www.hpc.nec/documents/sdk/SDK_NLC/UsersGuide/man/drot.html)
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
    if incx == 0 || incy == 0 {
        return false;
    }

    if c == 1.0 && s == 0.0 {
        return true;
    }

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

                let x_temp: __m256d = _mm256_add_pd(c_x, s_y);
                let y_temp: __m256d = _mm256_sub_pd(c_y, s_x);

                _mm256_get_pd!(x[offset_x..], incx, x_temp);
                _mm256_get_pd!(y[offset_y..], incy, y_temp);
            }

            offset_x += 4 * incx;
            offset_y += 4 * incy;
        }
    }

    for _ in (n_4 * 4) .. n {
        let x_temp = c * x[offset_x] + s * y[offset_y];
        y[offset_y] = c * y[offset_y] - s * x[offset_x];
        x[offset_x] = x_temp;

        offset_x += incx;
        offset_y += incy;
    }

    true
}