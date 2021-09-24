#![allow(unused)]

use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul, Div};

mod utils;

pub trait Numeric {
    fn sqrt(self) -> Self;
    fn sin(self)  -> Self;
    fn acos(self) -> Self;
    fn acosh(self) -> Self;
}
impl Numeric for f32 {
    fn sqrt(self) -> Self { self.sqrt() }
    fn sin(self)  -> Self { self.sin() }
    fn acos(self) -> Self { self.acos() }
    fn acosh(self) -> Self { self.acosh() }
}
impl Numeric for f64 {
    fn sqrt(self) -> Self { self.sqrt() }
    fn sin(self)  -> Self { self.sin() }
    fn acos(self) -> Self { self.acos() }
    fn acosh(self) -> Self { self.acosh() }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

//impl<T> Numeric for Complex<T> {
//
//}

impl<T: Copy + Mul<Output = T> + Add<Output = T> + Numeric> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Self {
            re: re,
            im: im,
        }
    }
    pub fn abs(self) -> T {
        <T as Numeric>::sqrt(self.re * self.re + self.im * self.im)
    }
}
impl<T: Add<Output = T>> Add for Complex<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self{ re: self.re + other.re, im: self.im + other.im}
    }
}
impl<T: AddAssign> AddAssign for Complex<T> {
    fn add_assign(&mut self, other: Self) {
        (*self).re += other.re;
        (*self).im += other.im;
    }
}
impl<T: Sub<Output = T>> Sub for Complex<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self{ re: self.re - other.re, im: self.im - other.im }
    }
}
impl<T: SubAssign> SubAssign for Complex<T> {
    fn sub_assign(&mut self, other: Self) {
        (*self).re -= other.re;
        (*self).im -= other.im;
    }
}
impl<T: Neg<Output = T>> Neg for Complex<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self { re: -self.re, im: -self.im}
    }
}
impl<T: Copy + Mul<Output = T> + Sub<Output = T> + Add<Output = T>> Mul for Complex<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self{
            re: self.re * other.re - self.im * other.im,
            im: self.im * other.re + self.re * other.im,
        }
    }
}
impl<T: Copy + Div<Output = T> + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Numeric> Div for Complex<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let abs_other = Self::abs(other);
        Self::new(
            (self.re * other.re + self.im * other.im) / abs_other,
            (self.im * other.re - self.re * other.im) / abs_other,
        )
    }
}

pub type Complex32 = Complex<f32>;
pub type Complex64 = Complex<f64>;

#[macro_use]
mod macros;

mod asum;
pub use asum::sasum;
pub use asum::dasum;

mod axpy;
pub use axpy::saxpy;
pub use axpy::daxpy;

mod copy;
pub use copy::scopy;
pub use copy::dcopy;

mod dot;
pub use dot::sdot;
pub use dot::ddot;

mod rot;
pub use rot::srot;
pub use rot::drot;

mod rotg;
pub use rotg::srotg;
pub use rotg::drotg;

mod scal;
pub use scal::sscal;
pub use scal::dscal;

mod swap;
pub use swap::sswap;
pub use swap::dswap;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
