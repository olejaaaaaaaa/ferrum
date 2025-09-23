use ash::vk::*;

use super::PhysicalDeviceBuilder;
use super::memory_properties::MemoryProperties;
use super::queue_family_properties::QueueFamilyProperties;
use super::physical_device_properties::PhysicalProperties;
use super::features::Features;


/// All Information about Physical Device
#[derive(Default, Clone)]
pub struct PhysicalDeviceInfo {
    /// physical device properties
    pub phys_prop: PhysicalProperties,
    /// device memory properties
    pub memory_prop: MemoryProperties,
    /// device features
    pub features: Features,
    /// queue families properties
    pub queue_family_prop: QueueFamilyProperties,
    /// device extensions
    pub extensions: Vec<ExtensionProperties>,
    /// device layers
    pub layers: Vec<LayerProperties>,
    /// is support surface
    pub is_support_surface: bool
}

use crate::errors::*;

impl<S> PhysicalDeviceBuilder<S> {

    ///Get information about physical device
    pub fn phys_device_info(&self, phys_dev: &ash::vk::PhysicalDevice, instance: &ash::Instance, api_version: u32, surface: &SurfaceKHR, surface_loader: &ash::khr::surface::Instance) -> VulkanResult<PhysicalDeviceInfo> {

        let extensions = unsafe { instance.enumerate_device_extension_properties(*phys_dev).map_err(|e| {
            VulkanError::PhysicalDevice(
                PhysicalDeviceError::EnumerateDeviceExtensionPropertiesFailed(e)
            )
        })}?;

        let layers = unsafe { instance.enumerate_device_layer_properties(*phys_dev).map_err(|e| {
            VulkanError::PhysicalDevice(
                PhysicalDeviceError::EnumerateDeviceLayerPropertiesFailed(e)
            )
        })}?;

        let data = Self::get_physical_properties(&instance, &phys_dev, api_version);
        let is_support_surface = Self::check_support_surface(&phys_dev, surface, surface_loader, data.count_queue_family_prop);

        Ok(PhysicalDeviceInfo{
            phys_prop: data.phys_prop,
            memory_prop: data.memory_prop,
            queue_family_prop: data.queue_family_prop,
            features: data.features,
            extensions,
            layers,
            is_support_surface
        })
    }
}

