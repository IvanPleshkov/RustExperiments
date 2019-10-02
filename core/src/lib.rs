#[macro_use]
pub mod trace;
pub mod id_set;
pub mod mesh;

pub use self::trace::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
