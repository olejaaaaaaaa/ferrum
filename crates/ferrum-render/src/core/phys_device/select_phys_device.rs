// use super::surface_loader::WithSurfaceLoader;
// use super::PhysicalDeviceBuilder;
// use ash::vk::SurfaceKHR;

// pub struct WithSelectPhysDevice<'n> {
//     pub instance: &'n ash::Instance,
//     pub api_version: u32,
//     pub surface: &'n SurfaceKHR,
//     pub surface_loader: &'n ash::khr::surface::Instance,
//     pub select_fn: Box<dyn FnOnce(&[super::physcial_device_info::PhysicalDeviceInfo]) -> usize>
// }

// impl<'n> PhysicalDeviceBuilder<WithSurfaceLoader<'n>> {
//     /// Select function for choosing physical device from list of available devices
//     ///
//     /// If None, first device from list will be chosen
//     pub fn select_physical_device(self, select_fn: Option<impl FnOnce(&[super::PhysicalDeviceInfo]) -> usize + 'static>) -> PhysicalDeviceBuilder<WithSelectPhysDevice<'n>> {

//         let select_fn = match select_fn {
//             Some(f) => Box::new(f) as Box<dyn FnOnce(&[super::PhysicalDeviceInfo]) -> usize>,
//             None => Box::<fn(&[super::PhysicalDeviceInfo]) -> usize>::new(default_select_device),
//         };

//         PhysicalDeviceBuilder { state: WithSelectPhysDevice {
//             instance: self.state.instance,
//             api_version: self.state.api_version,
//             surface: self.state.surface,
//             surface_loader: self.state.surface_loader,
//             select_fn
//         }}
//     }
// }

pub fn default_select_device(phys_infos: &[super::physcial_device_info::PhysicalDeviceInfo]) -> usize {
    0
}