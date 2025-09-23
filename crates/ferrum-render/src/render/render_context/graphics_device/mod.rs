
use std::sync::Arc;

mod app;
mod instance;
mod device;
mod queue_family;
mod phys_dev;
mod window;
mod universal_queue;
use universal_queue::UniversalQueue;

use crate::{
    CommandPool, Device, Instance, PhysicalDevice,
};

pub struct GraphicsDeviceBuilder<S> {
    pub state: S,
}

pub struct GraphicsDevice {
    pub instance: Instance,
    pub phys_dev: PhysicalDevice,
    pub logical_device: Device,
    pub universal_queue: UniversalQueue,
    pub command_pool: CommandPool,
    #[cfg(feature = "vma")]
    pub allocator: vk_mem::Allocator,
    #[cfg(feature = "gpu-allocator")]
    pub allocator: gpu_allocator::vulkan::Allocator,
}

impl GraphicsDevice {

    pub fn raw_instance(&self) -> &ash::Instance {
        &self.instance.raw
    }

    pub fn raw_device(&self) -> &ash::Device {
        &self.logical_device.raw
    }

    pub fn clone_raw_device(&self) -> Arc<ash::Device> {
        self.logical_device.raw.clone()
    }

}
