use std::ffi::CStr;

use ash::{
    prelude::VkResult,
    vk::{
        ApplicationInfo,
        LayerProperties
    }
};

use super::{
    api_version::WithApiVersion,
    InstanceBuilder,
};

use crate::{
    InstanceError,
    VulkanError,
    VulkanResult
};

pub struct WithLayers<'n> {
    pub entry: ash::Entry,
    pub app_info: ApplicationInfo<'n>,
    pub api_version: u32,
    pub layers: Vec<*const i8>,
}

impl<'n> InstanceBuilder<WithApiVersion<'n>> {

    pub fn with_layers<F, N>(self, layers: F) -> InstanceBuilder<WithLayers<'n>>
    where F: IntoIterator<Item = N>, N: AsRef<CStr>
    {

        let layers = layers.into_iter().map(|x| x.as_ref().as_ptr()).collect::<Vec<_>>();

        InstanceBuilder {
            state: WithLayers {
                app_info: self.state.app_info,
                entry: self.state.entry,
                api_version: self.state.api_version,
                layers
            },
            allocation_callback: self.allocation_callback,
            flags: self.flags
        }
    }
}


pub fn load_instance_layer_props(entry: &ash::Entry) -> VkResult<Vec<LayerProperties>> {
    unsafe { entry.enumerate_instance_layer_properties() }
}

/// check if required layers available for current vulkan instance
pub fn check_support_layers(entry: &ash::Entry, required_layers: &Vec<*const i8>) -> VulkanResult<()> {

    let available_layers = load_instance_layer_props(entry).map_err(|e| VulkanError::Instance(InstanceError::EnumerateInstanceLayerPropertiesFailed(e)))?;

    for req in required_layers {

        let mut is_support = false;
        let current_layer_name = unsafe { CStr::from_ptr(*req) };

        for available in &available_layers {

            let available_layer_name = available.layer_name_as_c_str().expect("Error get layer name as CStr");

            if current_layer_name == available_layer_name {
                is_support = true;
            }
        }

        if !is_support {
            return Err(VulkanError::Instance(InstanceError::NotSupportRequiredLayer(current_layer_name)));
        }
    }

    Ok(())
}