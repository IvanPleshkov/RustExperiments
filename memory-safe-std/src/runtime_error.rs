pub trait Exception {
    fn message(&self) -> String;
}

pub enum RuntimeError {
    ThreadTerminated,
    OutOfMemory,
    Exception,
}

pub enum SomeOrRuntimeError<T> {
    Some(T),
    Runtime(RuntimeError),
}
