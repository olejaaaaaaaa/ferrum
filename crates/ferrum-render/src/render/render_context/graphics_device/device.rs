
use crate::render_context::graphics_device::universal_queue::UniversalQueue;
use super::queue_family::WithQueueFamily;
use super::{GraphicsDevice, GraphicsDeviceBuilder};
use crate::core::{Device, DeviceBuilder, CommandPool, CommandPoolBuilder};
use ash::vk;

impl<'n> GraphicsDeviceBuilder<WithQueueFamily> {

    pub fn with_device<F>(self, build_fn: F) -> GraphicsDevice
    where F: FnOnce(&ash::Instance, &ash::vk::PhysicalDevice, &Vec<vk::QueueFamilyProperties>) -> Device {

        let device = build_fn(
            &self.state.instance.raw,
            &self.state.phys_dev.raw,
            &self.state.queue_family.properties,
        );

        let universal_queue = UniversalQueue::new(&device.raw, self.state.queue_family.properties, self.state.queue_family.supports_present);

        #[cfg(feature = "vma")]
        let allocator =  {
            let create_info = vk_mem::AllocatorCreateInfo::new(&self.state.instance.raw, &device.raw, self.state.phys_dev.raw);
            unsafe { vk_mem::Allocator::new(create_info).expect("Error create vma allocator") }
        };

        #[cfg(feature = "gpu-allocator")]
        let allocator = {
            let create_info= gpu_allocator::vulkan::AllocatorCreateDesc {
                instance: device.instance.raw.clone(),
                device: device.raw_device().clone(),
                physical_device: device.phys_dev.raw,
                debug_settings: Default::default(),
                buffer_device_address: false,
                allocation_sizes: Default::default(),
            };

            gpu_allocator::vulkan::Allocator::new(&create_info).expect("Error create gpu-allocator")
        };

        let command_pool = CommandPoolBuilder::new()
            .device(&device.raw)
            .family_index(universal_queue.index(vk::QueueFlags::TRANSFER))
            .build()
            .expect("Error create CommandPool");

        GraphicsDevice {
            instance: self.state.instance,
            phys_dev: self.state.phys_dev,
            logical_device: device.into(),
            #[cfg(any(feature = "vma", feature = "gpu-allocator"))]
            allocator,
            universal_queue,
            command_pool
        }
    }

    pub fn with_default_device(self) -> GraphicsDevice {
        self.with_device(|instance, phys_dev, queue_family| {
            DeviceBuilder::new()
                .with_instance(instance)
                .with_phys_dev(phys_dev)
                .with_features(None)
                .with_extensions(&[
                    c"VK_KHR_swapchain",
                ])
                .queue_families(&queue_family)
                .with_allocation_callback(None)
                .build()
                .unwrap()
        })
    }
}