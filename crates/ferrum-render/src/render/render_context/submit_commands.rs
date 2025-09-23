use ash::vk::{self, CommandBuffer, CommandBufferLevel, Fence, FenceCreateInfo, QueueFlags};
use crate::{GpuBuffer, RenderContext};

impl RenderContext {

    pub fn submit_commands<T: FnOnce(&mut CommandBuffer)>(&self, callback: T) {

        let device = self.device.raw_device();

        let fence = unsafe { device.create_fence(&FenceCreateInfo::default(), None).unwrap() };
        let mut command_buffer = self.device.command_pool.create_command_buffers(self.device.raw_device(), 1, CommandBufferLevel::PRIMARY)[0];

        let begin_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe {
            device
                .begin_command_buffer(command_buffer, &begin_info)
                .expect("Failed to begin command buffer");
        }

        callback(&mut command_buffer);

        unsafe {
            device
                .end_command_buffer(command_buffer)
                .expect("Failed to end command buffer");
        }

        let binding = [command_buffer];
        let submit_info = [vk::SubmitInfo::default()
            .command_buffers(&binding)
        ];

        unsafe {

            device
                .queue_submit(self.device.universal_queue.raw_queue(QueueFlags::GRAPHICS), &submit_info, fence)
                .expect("Failed to submit queue");

            device.wait_for_fences(&[fence], true, u64::MAX).unwrap();

            device.destroy_fence(fence, None);
        }

    }
}


