pub mod machine_integer;
pub mod big_integer;
pub mod rational;
pub mod machine_real;
pub mod big_real;
pub mod complex;

pub mod real_number;
pub mod complex_number;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
