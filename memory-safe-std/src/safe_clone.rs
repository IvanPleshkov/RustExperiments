use crate::runtime_error::RuntimeError;

pub trait SafeClone : Sized {
    fn safe_clone(&self) -> Result<Self, RuntimeError>;
}
