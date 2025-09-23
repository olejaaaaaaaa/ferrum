// use std::ffi::c_void;

// use ash::vk;
// use ash::vk::*;
// use log::warn;
// use log::{debug};

// use crate::total_vram;
// use crate::errors::*;




// #[derive(Default)]
// pub struct PhysicalDeviceBuilder<'n> {
//     pub api_version: Option<u32>,
//     pub instance: Option<&'n ash::Instance>,
//     pub surface_load: Option<&'n ash::khr::surface::Instance>,
//     pub surface: Option<&'n ash::vk::SurfaceKHR>,
//     pub fn_select_phys_dev: Option<Box<dyn FnOnce(&[PhysicalDeviceInfo]) -> usize>>
// }

// impl<'n> PhysicalDeviceBuilder<'n> {

//     /// Create empty builder
//     pub fn new() -> Self {
//         Self { ..Default::default() }
//     }

//     /// VK_API_VERSION
//     pub fn with_api_version(mut self, api_version: u32) -> Self {
//         self.api_version = Some(api_version);
//         self
//     }

//     /// Required [`ash::khr::surface::Instance`] for ['ash::Device::get_physical_device_surface_support()']
//     pub fn with_surface(mut self, surface: &'n ash::vk::SurfaceKHR) -> Self {
//         self.surface = Some(surface);
//         self
//     }

//     /// Required [`ash::khr::surface::Instance`] for ['get_physical_device_surface_support']
//     pub fn with_surface_load(mut self, surface_load: &'n ash::khr::surface::Instance) -> Self {
//         self.surface_load = Some(surface_load);
//         self
//     }

//

//     fn get_properties_v1(&self, instance: &'n ash::Instance, phys_dev: &ash::vk::PhysicalDevice) -> (
//         Features,
//         MemoryProperties,
//         QueueFamilyProperties,
//         PhysicalProperties,
//         usize
//     ) {
//         unsafe  {

//             let _features = instance.get_physical_device_features(*phys_dev);
//             let _memory_prop = instance.get_physical_device_memory_properties(*phys_dev);
//             let mut _queue_family_prop = instance.get_physical_device_queue_family_properties(*phys_dev);
//             let count_familes = _queue_family_prop.len();

//             let _phys_prop = instance.get_physical_device_properties(*phys_dev);

//             (
//                 Features::V1(_features),
//                 MemoryProperties::V1(_memory_prop),
//                 QueueFamilyProperties::V1(_queue_family_prop),
//                 PhysicalProperties::V1(_phys_prop),
//                 count_familes
//             )
//         }
//     }

//     fn get_properties_v2(&self, instance: &'n ash::Instance, phys_dev: &ash::vk::PhysicalDevice) -> (
//         Features,
//         MemoryProperties,
//         QueueFamilyProperties,
//         PhysicalProperties,
//         usize
//     ) {

//         unsafe {

//             let mut features2 = vk::PhysicalDeviceFeatures2::default();
//             instance.get_physical_device_features2(*phys_dev, &mut features2);

//             let mut mem_props2 = vk::PhysicalDeviceMemoryProperties2::default();
//             instance.get_physical_device_memory_properties2(*phys_dev, &mut mem_props2);

//             // TODO: FIXME I must use get_physical_device_queue_family_properties2
//             let mut _queue_prop2: Vec<vk::QueueFamilyProperties2<'_>> = vec![];
//             let _queue_prop2 = instance.get_physical_device_queue_family_properties(*phys_dev);
//             let queue_prop_len = _queue_prop2.len();
//             let cached = _queue_prop2.iter().map(|x| x.clone()).collect::<Vec<_>>();

//             let mut props2 = vk::PhysicalDeviceProperties2::default();

//             instance.get_physical_device_properties2(*phys_dev, &mut props2);

//             (
//                 Features::V2(features2),
//                 MemoryProperties::V2(mem_props2),
//                 QueueFamilyProperties::V2 { raw: vec![], cached: cached },
//                 PhysicalProperties::V2(props2),
//                 queue_prop_len,
//             )
//         }
//     }

//     fn check_support_surface(phys_dev: &ash::vk::PhysicalDevice, surface: &SurfaceKHR, surface_load: &'n ash::khr::surface::Instance, queue_familes_len: usize) -> bool {

//         for index in 0..queue_familes_len {

//             let res = unsafe { surface_load.get_physical_device_surface_support(*phys_dev, index as u32, *surface) };

//             match res {
//                 Ok(support) => {
//                     if support {
//                         return true
//                     }
//                 },

//                 Err(err) => {
//                     log::warn!("Error check support surface: {}", err);
//                 }
//             }
//         }

//         return false;
//     }

//     pub fn select_physical_device<F>(mut self, choose_device: F) -> Self
//     where F: FnOnce(&[PhysicalDeviceInfo]) -> usize + 'static
//     {
//         self.fn_select_phys_dev = Some(Box::new(choose_device));
//         self
//     }

//     pub fn with_instance(mut self, instance: &'n ash::Instance) -> Self {
//         self.instance = Some(instance);
//         self
//     }

//     pub fn build(self) -> VulkanResult<PhysicalDevice> {

//         let instance = self.instance.unwrap();
//         let api_version = self.api_version.unwrap();

//         let phys_devs = unsafe { instance.enumerate_physical_devices().map_err(|e|
//             VulkanError::PhysicalDevice(
//                 PhysicalDeviceError::EnumeratePhysicalDeviceFailed(e))
//             )?
//         };

//         let mut phys_infos = vec![];

//         for phys_dev in &phys_devs {

//             let phys_info = self.phys_device_info(phys_dev, &instance, api_version);

//             if let Ok(phys_info) = phys_info {

//                 if phys_info.support_surface {
//                     phys_infos.push(phys_info);
//                 }
//             }
//         }

//         let index = self.fn_select_phys_dev.unwrap_or(Box::new(|_| 0))(&phys_infos);
//         let phys_dev = phys_devs[index];
//         let phys_info = &phys_infos[index];
//         let vram = total_vram(phys_info);

//         debug!(
//             "\nGPU NAME:        {:?}\
//             \nTYPE:             {:?}\
//             \nDRIVER VERSION:   {:?}\
//             \nVRAM:             {:?} MB\
//             \nAPI VERSION:      {:?}",
//             phys_info.phys_prop.device_name_as_c_str().unwrap(),
//             phys_info.phys_prop.device_type,
//             phys_info.phys_prop.driver_version,
//             vram / (1024 * 1024),
//             phys_info.phys_prop.api_version
//         );

//         Ok(PhysicalDevice {
//             raw: phys_dev,
//             phys_info: phys_info.clone(),
//             #[cfg(debug_assertions)]
//             destroyed: false
//         })
//     }
// }





// #[cfg(debug_assertions)]
// impl Drop for PhysicalDevice {
//     fn drop(&mut self) {
//         if !self.destroyed {
//             log::warn!("Physical Device is don't destroyed, before drop")
//         }
//     }
// }