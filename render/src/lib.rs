mod device;
mod system;

pub use system::System as System;
pub use system::NativeApi as NativeApi;
pub use system::SystemInitializationParams as SystemInitializationParams;
pub use device::Device as Device;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
