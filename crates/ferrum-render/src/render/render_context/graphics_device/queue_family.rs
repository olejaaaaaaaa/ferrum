
use crate::core::{
    Instance,
    PhysicalDevice,
    QueueFamilies,
    QueueFamiliesBuilder,
    Surface
};

use super::{GraphicsDevice, GraphicsDeviceBuilder};
use super::phys_dev::WithPhysicalDevice;
pub struct WithQueueFamily {
    pub instance: Instance,
    pub phys_dev: PhysicalDevice,
    pub queue_family: QueueFamilies
}

impl<'n> GraphicsDeviceBuilder<WithPhysicalDevice> {

pub fn with_queue_families<F>(self, surface: &'n Surface, build_fn: F) -> GraphicsDeviceBuilder<WithQueueFamily>
    where F: FnOnce(&Surface, &PhysicalDevice) -> QueueFamilies {

        let queue_family = build_fn(surface, &self.state.phys_dev,);

        GraphicsDeviceBuilder {
            state: WithQueueFamily {
                instance: self.state.instance,
                phys_dev: self.state.phys_dev,
                queue_family
            }
        }
    }

    pub fn with_default_queue_families(self, surface: &'n Surface) -> GraphicsDeviceBuilder<WithQueueFamily> {

        self.with_queue_families(surface, |surface, phys_dev| {
            QueueFamiliesBuilder::new()
                .with_phys_dev(&phys_dev.raw)
                .with_surface(&surface.raw)
                .with_surface_loader(&surface.raw_loader)
                .with_queue_family_properties(&*phys_dev.phys_info.queue_family_prop)
                .build()
        })

    }
}