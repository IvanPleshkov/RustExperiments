use crate::device::Device;

pub trait System {
    fn get_devices(&self, index: usize) -> &[&dyn Device];

    fn get_devices_mut(&mut self, index: usize) -> &[&mut dyn Device];
}
