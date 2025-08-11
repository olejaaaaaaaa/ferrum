

use crate::{core::{
    Instance,
}, DeviceBuilder, QueueFamily};

use super::*;

impl GraphicsDeviceBuilder<WithQueueFamily> {

    pub fn with_device<F>(self, build_fn: F) -> GraphicsDevice
    where F: FnOnce(&Instance, &PhysicalDevice, &Vec<QueueFamily>) -> Device {

        let device = build_fn(&self.state.instance, &self.state.phys_dev, &self.state.queue_family);
        let universal_queue = UniversalQueue::new(&device.raw, self.state.queue_family);

        GraphicsDevice {
            instance: self.state.instance,
            phys_dev: self.state.phys_dev,
            logical_device: device,
            universal_queue
        }
    }

    pub fn with_default_device(self) -> GraphicsDevice {
        self.with_device(|instance, phys_dev, queue_family| {
            DeviceBuilder::new()
                .with_extensions(vec![
                    c"VK_KHR_swapchain"
                ])
                .queue_family(&queue_family)
                .with_instance(&instance.raw)
                .with_phys_dev(&phys_dev.raw)
                .build()
        })
    }
}