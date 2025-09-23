
pub struct UniversalQueue {
    pub queue_families: Vec<ash::vk::QueueFamilyProperties>,
    pub is_surface_support: Vec<bool>,
    pub raw: Vec<Vec<ash::vk::Queue>>
}

impl UniversalQueue {

    pub fn raw_queue(&self, flags: ash::vk::QueueFlags) -> ash::vk::Queue {

        for (index, queue_family) in self.queue_families.iter().enumerate() {
            if self.is_surface_support[index] && queue_family.queue_flags.contains(flags) {
                return self.raw[index][0]
            }
        }

        panic!("Not found Queue")
    }

    pub fn index(&self, flags: ash::vk::QueueFlags) -> u32 {

        for (index, queue_family) in self.queue_families.iter().enumerate() {
            if self.is_surface_support[index] && queue_family.queue_flags.contains(flags) {
                return index as u32
            }
        }

        panic!("Not found Index")
    }

    pub fn new(device: &ash::Device, families: Vec<ash::vk::QueueFamilyProperties>, support_surface: Vec<bool>) -> Self {

        let mut queue = vec![];

        for (index, i) in families.iter().enumerate() {

            let mut queues = vec![];

            for j in 0..i.queue_count {
                let queue = unsafe { device.get_device_queue(index as u32, j) };
                queues.push(queue);
            }

            queue.push(queues)
        }

        Self { queue_families: families, raw: queue, is_surface_support: support_surface }
    }
}