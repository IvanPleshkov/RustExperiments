use nalgebra::Vector3;
use std::any::Any;
use std::fmt::Display;

fn main() {
    let x = nalgebra::Vector3::new(0.0, 0.0, 0.0);
    log(Box::new(x))
}

fn log(value: Box<Any + 'static>) {
    let x: Result<Box<&str>, _> = value.downcast();
}
