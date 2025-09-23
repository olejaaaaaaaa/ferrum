
pub mod instance;
pub mod phys_dev;
pub mod features;
pub mod extensions;
pub mod queue_families;
pub mod allocation_callback;

use std::marker::PhantomData;
use std::sync::Arc;
use allocation_callback::WithAllocationCallback;
use crate::VulkanError;
use crate::VulkanResult;
use ash::vk::DeviceQueueCreateInfo;
use ash::vk::DeviceCreateInfo;


pub struct Device {
    pub raw: Arc<ash::Device>,
    #[cfg(debug_assertions)]
    destroyed: bool
}

pub struct DeviceBuilder<S = ()> {
    state: S
}

impl DeviceBuilder<()> {
    pub fn new() -> DeviceBuilder<()> {
        Self { state: () }
    }
}

impl<'n> DeviceBuilder<WithAllocationCallback<'n>> {

    pub fn build(self) -> VulkanResult<Device> {

        let mut priorities: Vec<Vec<f32>> = vec![];

        for i in self.state.families {
            priorities.push((1..i.queue_count+1).map(|ndx| 1.0 / (ndx as f32)).collect::<Vec<f32>>());
        }

        let mut queue_infos = vec![];

        for (index, _) in self.state.families.iter().enumerate() {
            let queue_info = DeviceQueueCreateInfo::default()
                .queue_family_index(index as u32)
                .queue_priorities(&priorities[index as usize]);

            queue_infos.push(queue_info);
        }

        let create_info = DeviceCreateInfo::default()
            .queue_create_infos(&queue_infos)
            .enabled_extension_names(&self.state.extensions)
            .enabled_features(&self.state.features);

        let device = unsafe { self.state.instance.create_device(*self.state.phys_dev, &create_info, self.state.callback)
            .map_err(|e| VulkanError::Unknown)?
        };

        Ok(Device {
            raw: Arc::new(device),
            //_instance_life: PhantomData,
            #[cfg(debug_assertions)]
            destroyed: false
        })
    }
}


#[cfg(debug_assertions)]
impl Drop for Device {
    fn drop(&mut self) {
        if !self.destroyed {
            log::warn!("Device was not destroyed before being dropped!");
        }
    }
}