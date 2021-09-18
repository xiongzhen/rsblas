#[macro_use]
mod macros;

mod srot;
pub use srot::srot;

mod drot;
pub use drot::drot;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
