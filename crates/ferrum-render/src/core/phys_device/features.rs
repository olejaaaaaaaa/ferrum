
#[derive(Clone, Copy)]
pub enum Features {
    V1(ash::vk::PhysicalDeviceFeatures),
    V2(ash::vk::PhysicalDeviceFeatures2<'static>),
    None
}

impl Default for Features {
    fn default() -> Self {
        Features::None
    }
}