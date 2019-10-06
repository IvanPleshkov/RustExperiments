#[macro_use]
pub mod trace;
pub mod id_set;
pub mod mesh;
pub mod init_logger;

pub use self::trace::*;
pub use self::init_logger::*;

pub use f64 as real;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
