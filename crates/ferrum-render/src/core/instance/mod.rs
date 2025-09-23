pub mod api_version;
// pub mod flags;
pub mod entry;
pub mod extensions;
pub mod layers;
// pub mod debug_messenger;
// pub mod allocation_callback;
pub mod app_info;
// pub mod push_next;

use ash::vk::*;

use extensions::WithExtensions;
use crate::layers::check_support_layers;

use crate::{
    InstanceError,
    VulkanResult,
    VulkanError
};

pub struct InstanceBuilder<S = ()> {
    pub allocation_callback: Option<&'static ash::vk::AllocationCallbacks<'static>>,
    pub flags: InstanceCreateFlags,
    //pub next: Vec<Box<T>>,
    pub state: S
}

impl InstanceBuilder<()> {
    pub fn new() -> InstanceBuilder<()> {
        InstanceBuilder {
            state: (),
            allocation_callback: None,
            flags: InstanceCreateFlags::default(),
            //next: vec![]
        }
    }
}

/// Required destroy after Device!
pub struct Instance {
    pub entry: ash::Entry,
    pub raw: ash::Instance,
    pub api_version: u32,
    pub allocation_callback: Option<&'static AllocationCallbacks<'static>>,
    pub debug_callback: Option<ash::vk::DebugUtilsMessengerEXT>,
    pub debug_utils_loader: Option<ash::ext::debug_utils::Instance>,
    #[cfg(debug_assertions)]
    pub destroyed: bool
}

impl Instance {

    #[cfg(debug_assertions)]
    pub fn destroy(&mut self) {
        self.destroyed = true;
        unsafe { self.raw.destroy_instance(self.allocation_callback); }
    }

    #[cfg(not(debug_assertions))]
    pub fn destroy(&mut self) {
        unsafe { self.raw.destroy_instance(self.allocation_callback); }
    }
}

impl<'n> InstanceBuilder<WithExtensions<'n>> {

    pub fn build(self) -> VulkanResult<Instance> {

        check_support_layers(&self.state.entry, &self.state.layers)?;

        let create_info = InstanceCreateInfo::default()
            .application_info(&self.state.app_info)
            .enabled_extension_names(&self.state.extensions)
            .enabled_layer_names(&self.state.layers)
            .flags(self.flags);

        // for i in self.state.push_next {
        //     create_info = create_info.push_next(i);
        // }

        let instance = unsafe {
            self.state.entry.create_instance(&create_info, self.allocation_callback)
                .map_err(|e|
                    VulkanError::Instance(InstanceError::InstanceCreationFailed(e))
                )?
        };

        Ok(Instance {
            raw: instance,
            entry: self.state.entry,
            api_version: self.state.api_version,
            allocation_callback: self.allocation_callback,
            debug_callback: None,
            debug_utils_loader: None,
            #[cfg(debug_assertions)]
            destroyed: false
        })
    }
}


#[cfg(debug_assertions)]
impl Drop for Instance {
    fn drop(&mut self) {
        if !self.destroyed {
            log::warn!("Instance was not destroyed before being dropped!");
        }
    }
}