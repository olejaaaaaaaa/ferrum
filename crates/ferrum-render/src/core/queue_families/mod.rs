
pub mod phys_dev;
pub mod surface;
pub mod surface_loader;
pub mod queue_family_properties;
use queue_family_properties::WithQueueFamilyProperties;

#[derive(Debug)]
pub struct QueueFamilies {
    pub index: Vec<u32>,
    pub properties: Vec<ash::vk::QueueFamilyProperties>,
    pub supports_present: Vec<bool>,
}

pub struct QueueFamiliesBuilder<T> {
    pub state: T
}

impl QueueFamiliesBuilder<()> {
    pub fn new() -> Self {
        Self { state: () }
    }
}

impl<'n> QueueFamiliesBuilder<WithQueueFamilyProperties<'n>> {

    pub fn build(self) -> QueueFamilies {

        let mut queue = QueueFamilies {
            index: vec![],
            properties: vec![],
            supports_present: vec![]
        };

        for (index, prop) in self.state.properties.iter().enumerate() {

            let support = unsafe { self.state.surface_loader.get_physical_device_surface_support(*self.state.phys_dev, index as u32, *self.state.surface).unwrap_or(false) };

            queue.index.push(index as u32);
            queue.properties.push(*prop);
            queue.supports_present.push(support);
        }

        queue
    }
}