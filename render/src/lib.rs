mod buffer;
mod device;
mod resource;
mod system;

pub use self::buffer::*;
pub use self::device::*;
pub use self::resource::*;
pub use self::system::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
