macro_rules! _mm256_set_dup_ps {
    ($x:expr) => (_mm256_set_ps($x, $x, $x, $x, $x, $x, $x, $x))
}

macro_rules! _mm256_set_slice_ps {
    ($xs:expr, $i:expr) => (
        _mm256_set_ps(
            $xs[7 * $i], $xs[6 * $i], $xs[5 * $i], $xs[4 * $i],
            $xs[3 * $i], $xs[2 * $i], $xs[$i], $xs[0]
        )
    )
}

macro_rules! _mm256_get_ps {
    ($xs:expr, $i:expr, $src:expr) => (
        let values = &mut [0.0_f32; 8];
        _mm256_storeu_ps(values.as_mut_ptr(), $src);
        $xs[0] = values[0];
        if $i != 0 {
            $xs[$i] = values[1];
            $xs[2 * $i] = values[2];
            $xs[3 * $i] = values[3];
            $xs[4 * $i] = values[4];
            $xs[5 * $i] = values[5];
            $xs[6 * $i] = values[6];
            $xs[7 * $i] = values[7];
        }
    )
}