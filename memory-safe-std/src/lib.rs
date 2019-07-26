pub mod runtime_error;
pub mod locale;
pub mod safe_clone;
pub mod safe_vec;
pub mod safe_string;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
