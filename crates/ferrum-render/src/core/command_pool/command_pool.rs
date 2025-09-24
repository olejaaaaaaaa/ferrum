use ash::vk::{CommandBuffer, CommandBufferAllocateInfo, CommandBufferLevel, CommandPoolCreateFlags, CommandPoolCreateInfo};

use crate::{VulkanError, VulkanResult};

pub struct CommandPool {
    pub raw: ash::vk::CommandPool,
    pub queue_index: u32
}

impl CommandPool {

    pub fn create_command_buffers(&self, device: &ash::Device, count: u32, level: CommandBufferLevel) -> Vec<CommandBuffer>{

        let allocate_info = CommandBufferAllocateInfo::default()
            .command_buffer_count(count)
            .command_pool(self.raw)
            .level(level);

        let buffers = unsafe { device.allocate_command_buffers(&allocate_info).unwrap() };
        buffers
    }
}

#[derive(Default)]
pub struct CommandPoolBuilder<'n> {
    device: Option<&'n ash::Device>,
    family_index: Option<u32>
}

impl<'n> CommandPoolBuilder<'n> {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn device(mut self, device: &'n ash::Device) -> Self {
        self.device = Some(device);
        self
    }

    pub fn family_index(mut self, family_index: u32) -> Self {
        self.family_index = Some(family_index);
        self
    }

    pub fn build(self) -> VulkanResult<CommandPool> {

        let family_index = self.family_index.unwrap();

        let create_info = CommandPoolCreateInfo::default()
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(family_index);

        let command_pool = unsafe { self.device.unwrap().create_command_pool(&create_info, None).map_err(|e| {
            VulkanError::Unknown
        })}?;

        Ok(CommandPool {
            raw: command_pool,
            queue_index: family_index
        })
    }
}