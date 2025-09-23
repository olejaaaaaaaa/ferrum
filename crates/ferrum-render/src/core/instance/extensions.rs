use std::ffi::CStr;

use ash::{
    prelude::VkResult,
        vk::{
            ApplicationInfo,
            ExtensionProperties,
            InstanceCreateFlags
        }
};

use super::{
    layers::WithLayers,
    InstanceBuilder
};

pub struct WithExtensions<'n> {
    pub entry: ash::Entry,
    pub app_info: ApplicationInfo<'n>,
    pub api_version: u32,
    pub layers: Vec<*const i8>,
    pub extensions: Vec<*const i8>
}

impl<'n> InstanceBuilder<WithLayers<'n>> {

    pub fn with_extensions<F, N>(self, extensions: F) -> InstanceBuilder<WithExtensions<'n>>
    where F: IntoIterator<Item = N>, N: AsRef<CStr>
    {

        #[allow(unused_mut)]
        let mut extensions = extensions.into_iter().map(|x| x.as_ref().as_ptr()).collect::<Vec<_>>();

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        {
            let required_portability = extensions.iter().any(|x| {
                let name = unsafe { CStr::from_ptr(*x) };
                if name == ash::khr::portability_enumeration::NAME {
                    return true;
                }
                false
            });

            if required_portability { extensions.push(ash::khr::portability_enumeration::NAME.as_ptr()) }

            let required_physical_device_properties2 = extensions.iter().any(|x| {
                let name = unsafe { CStr::from_ptr(*x) };
                if name == ash::khr::get_physical_device_properties2::NAME {
                    return true;
                }
                false
            });

            if required_physical_device_properties2 { extensions.push(ash::khr::portability_enumeration::NAME.as_ptr()) }
        }

        InstanceBuilder {

            state: WithExtensions {
                entry: self.state.entry,
                app_info: self.state.app_info,
                api_version: self.state.api_version,
                layers: self.state.layers,
                extensions
            },

            allocation_callback: self.allocation_callback,
            flags: self.flags
        }
    }
}

/// TODO
fn check_support_extensions() {

}

pub fn load_instance_extension_props(entry: &ash::Entry, layer_name: Option<&CStr>) -> VkResult<Vec<ExtensionProperties>> {
    unsafe { entry.enumerate_instance_extension_properties(layer_name) }
}