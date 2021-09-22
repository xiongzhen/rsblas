#![allow(unused)]

use std::ops::{Add, AddAssign, Sub, SubAssign};



#[derive(Debug, Copy, Clone)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}
impl<T: Default> Default for Complex<T> {
    fn default() -> Complex<T> {
        Self {
            re: <T as Default>::default(),
            im: <T as Default>::default(),
        }
    }
}
impl<T: Add<Output = T>> Add for Complex<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
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
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}
impl<T: SubAssign> SubAssign for Complex<T> {
    fn sub_assign(&mut self, other: Self) {
        (*self).re -= other.re;
        (*self).im -= other.im;
    }
}


#[macro_use]
mod macros;

mod sasum;
pub use sasum::sasum;
mod dasum;
pub use dasum::dasum;

mod rot;
pub use rot::srot;
pub use rot::drot;

mod sscal;
pub use sscal::sscal;





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
