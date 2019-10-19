mod device;
mod system;
mod resource;
mod buffer;

pub use self::device::*;
pub use self::system::*;
pub use self::resource::*;
pub use self::buffer::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
