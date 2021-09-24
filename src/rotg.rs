use std::ops::{Add, Mul, Div, Neg};

trait RotgAbs {
    fn abs(self) -> Self;
}
impl RotgAbs for f32 {
    fn abs(self) -> Self {
        f32::abs(self)
    }
}
impl RotgAbs for f64 {
    fn abs(self) -> Self {
        f64::abs(self)
    }
}

trait RotgSqrt {
    fn sqrt(self) -> Self;
}
impl RotgSqrt for f32 {
    fn sqrt(self) -> Self { f32::sqrt(self) }
}
impl RotgSqrt for f64 {
    fn sqrt(self) -> Self { f64::sqrt(self) }
}

trait RotgZeroOne {
    fn zero() -> Self;
    fn one()  -> Self;
}
impl RotgZeroOne for f32 {
    fn zero() -> Self { 0.0_f32 }
    fn one()  -> Self { 1.0_f32 }
}
impl RotgZeroOne for f64 {
    fn zero() -> Self { 0.0_f64 }
    fn one()  -> Self { 1.0_f64 }
}

fn rotg<T>(a: &mut T, b: &mut T, c: &mut T, s: &mut T) -> bool
where T: Default + PartialOrd + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Neg<Output = T> + Copy + RotgAbs + RotgSqrt + RotgZeroOne,
{
    
    let anorm: T = T::abs(*a);
    let bnorm: T = T::abs(*b);
    let scale: T = anorm + bnorm;

    let roe: T = if anorm > bnorm { *a } else { *b };
    
    let r: T;
    let z: T;
    if scale == T::zero() {
        *c = T::one();
        *s = T::zero();
        r = T::zero();
        z = T::zero();
    } else {
        let a_scale = (*a) / scale;
        let b_scale = (*b) / scale;
        let r_scale = scale * T::sqrt(a_scale * a_scale + b_scale * b_scale);
        r = {if roe >= T::zero() {T::one()} else {-T::one()}} * r_scale;
        *c = (*a) / r;
        *s = (*b) / r;
        if anorm > bnorm {
            z = *s;
        } else if bnorm >= anorm && *c != T::zero() {
            z = T::one() / *c;
        } else {
            z = T::one();
        }
    }

    *a = r;
    *b = z;

    true
}

/// construct givens plane rotation
pub fn srotg(a: &mut f32, b: &mut f32, c: &mut f32, s: &mut f32) -> bool {
    return rotg::<f32>(a, b, c, s);
}

/// construct givens plane rotation
pub fn drotg(a: &mut f64, b: &mut f64, c: &mut f64, s: &mut f64) -> bool {
    return rotg::<f64>(a, b, c, s);
}