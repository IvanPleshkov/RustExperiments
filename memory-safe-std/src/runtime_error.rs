pub trait Exception {
    fn message(&self) -> String;
}

#[derive(Debug)]
pub enum RuntimeError {
    Exception,
    ThreadTerminated,
    OutOfMemory,
}
