#![allow(unused)]

#[derive(Debug)]
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
impl<T: std::ops::Add<Output = T>> std::ops::Add for Complex<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}
impl<T: std::ops::AddAssign> std::ops::AddAssign for Complex<T> {
    fn add_assign(&mut self, other: Self) {
        (*self).re += other.re;
        (*self).im += other.im;
    }
}
impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Complex<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}
impl<T: std::ops::SubAssign> std::ops::SubAssign for Complex<T> {
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

mod srot;
pub use srot::srot;
mod drot;
pub use drot::drot;

mod sscal;
pub use sscal::sscal;





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
