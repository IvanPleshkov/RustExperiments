mod device;
mod system;

pub use self::device::*;
pub use self::system::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
