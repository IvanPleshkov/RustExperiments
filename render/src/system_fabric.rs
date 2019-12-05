use crate::system::System;

pub trait SystemFabric {
    fn create() -> Option<Box<dyn System>>;
}
