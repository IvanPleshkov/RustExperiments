pub trait SafeClone : Sized {
    type Err: Sized;
    fn safe_clone(&self) -> Result<Self, Self::Err>;
}
