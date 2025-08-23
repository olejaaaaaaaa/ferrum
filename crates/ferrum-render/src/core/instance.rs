use std::ffi::CStr;

use ash::{
    ext::debug_utils,
    prelude::VkResult,
    vk::*,
    Entry
};

use crate::VulkanError;
use crate::{vulkan_debug_callback, InstanceError, VulkanResult};

#[derive(Default)]
pub struct InstanceBuilder<'n> {
    app: Option<ash::vk::ApplicationInfo<'n>>,
    entry: Option<ash::Entry>,
    api_version: Option<u32>,
    flags: Option<InstanceCreateFlags>,
    extensions: Vec<*const i8>,
    layers: Vec<*const i8>,
    debug_extensions: Vec<*const i8>,
    debug_layers: Vec<*const i8>,
    allocation_callbacks: Option<AllocationCallbacks<'static>>
}

pub struct Instance {
    pub entry: ash::Entry,
    pub raw: ash::Instance,
    pub api_version: u32,
    allocation_callbacks: Option<AllocationCallbacks<'static>>,
    #[cfg(debug_assertions)]
    pub debug_callback: ash::vk::DebugUtilsMessengerEXT,
    #[cfg(debug_assertions)]
    pub debug_utils_loader: ash::ext::debug_utils::Instance,
    #[cfg(debug_assertions)]
    destroyed: bool
}

impl Instance {

    pub fn raw(&self) -> &ash::Instance {
        &self.raw
    }

    pub fn entry(&self) -> &Entry {
        &self.entry
    }

    #[cfg(debug_assertions)]
    pub fn destroy(&mut self) {
        self.destroyed = true;
        unsafe { self.raw().destroy_instance(self.allocation_callbacks.as_ref()); }
    }

    #[cfg(not(debug_assertions))]
    pub fn destroy(&mut self) {
        unsafe { self.raw().destroy_instance(self.allocation_callbacks.as_ref()); }
    }
}

impl<'n> InstanceBuilder<'n> {

    /// Создание пустого builder
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn with_app(mut self, app: ApplicationInfo<'n>) -> Self {
        self.app = Some(app);
        self
    }

    pub fn with_api_version(mut self, api_version: u32) -> Self {
        self.api_version = Some(api_version);
        self
    }

    pub fn with_entry(mut self, entry: Entry) -> Self {
        self.entry = Some(entry);
        self
    }

    /// Флаги для экземпляра
    pub fn with_instance_flags(mut self, flags: InstanceCreateFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    /// Добавляем расширения для экземпляра vulkan
    pub fn with_extensions(mut self, names: Vec<&'static CStr>) -> Self {
        self.extensions.extend(names.iter().map(|name| name.as_ptr()));
        self
    }

    /// Добавляем слои для экземпляра vulkan
    pub fn with_layers(mut self, names: Vec<&'static CStr>) -> Self {
        self.layers.extend(names.iter().map(|name| name.as_ptr()));
        self
    }

    /// Добавляем слои, которые будут включены, только в debug режиме
    pub fn with_debug_layers(mut self, names: Vec<&'static CStr>) -> Self {
        self.debug_layers.extend(names.iter().map(|name| name.as_ptr()));
        self
    }

    /// Добавляем расширения, которые будут включены, только в debug режиме
    pub fn with_debug_extensions(mut self, names: Vec<&'static CStr>) -> Self {
        self.debug_extensions.extend(names.iter().map(|name| name.as_ptr()));
        self
    }

    /// Добавляем callback функцию, для слежением за аллокациями/деаллокациями экземпляра Vulkan
    pub fn with_allocation_callbacks(mut self, callbacks: AllocationCallbacks<'static>) -> Self {
        self.allocation_callbacks = Some(callbacks);
        self
    }

    pub fn build(self) -> VulkanResult<Instance> {

        let app = self.app.ok_or(crate::VulkanError::Instance(InstanceError::MissingApp))?;
        let entry = self.entry.unwrap();
        let flags = self.flags.unwrap_or(InstanceCreateFlags::default());
        let allocation_callbacks = self.allocation_callbacks;
        let mut layers = self.layers;
        let mut ext = self.extensions;

        #[cfg(debug_assertions)]
        add_debug_extensions(&mut ext, &self.debug_extensions);

        #[cfg(debug_assertions)]
        add_debug_layers(&mut layers, &self.debug_layers);

        assert!(check_support_layers(&entry, &layers).is_ok());


        for i in &ext {
            unsafe { println!("ext {:?}", CStr::from_ptr(i.clone())); }
        }

        let create_info = InstanceCreateInfo::default()
            .application_info(&app)
            .enabled_extension_names(&ext)
            .enabled_layer_names(&layers)
            .flags(flags);

        let instance = unsafe { entry.create_instance(&create_info, allocation_callbacks.as_ref()).expect("Error create Instance") };

        #[cfg(debug_assertions)]
        let (debug_utils_loader, debug_callback) = create_debug_utils_messenger(&entry, &instance)?;

        Ok(Instance {
            raw: instance,
            entry: entry,
            api_version: app.api_version,
            allocation_callbacks: allocation_callbacks,
            #[cfg(debug_assertions)]
            debug_callback,
            #[cfg(debug_assertions)]
            debug_utils_loader,
            #[cfg(debug_assertions)]
            destroyed: false
        })
    }

}

pub fn load_instance_extension_props(entry: &Entry, layer_name: Option<&CStr>) -> VkResult<Vec<ExtensionProperties>> {
    unsafe { entry.enumerate_instance_extension_properties(layer_name) }
}

pub fn load_instance_layer_props(entry: &Entry) -> VkResult<Vec<LayerProperties>> {
    unsafe { entry.enumerate_instance_layer_properties() }
}

/// check if required layers available for current vulkan instance
fn check_support_layers(entry: &Entry, required_layers: &Vec<*const i8>) -> VulkanResult<()> {

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

/// TODO
fn check_support_extensions() {

}

fn create_debug_utils_messenger(entry: &Entry, instance: &ash::Instance) -> VulkanResult<(ash::ext::debug_utils::Instance, DebugUtilsMessengerEXT)> {

    use ash::vk;

    let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
    .message_severity(
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO |
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING |
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
    )
    .message_type(
        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL |
        vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION |
        vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
    )
    .pfn_user_callback(Some(vulkan_debug_callback));

    let loader = debug_utils::Instance::new(entry, instance);
    let callback = unsafe {
        loader.create_debug_utils_messenger(&debug_info, None)
            .map_err(|e| VulkanError::Instance(InstanceError::DebugUtilsMessengerCreationFailed(e)))?
    };

    Ok((loader, callback))
}

fn add_debug_layers(layers: &mut Vec<*const i8>, debug_layers: &Vec<*const i8>) {
    layers.extend(debug_layers);
}

fn add_debug_extensions(ext: &mut Vec<*const i8>, debug_ext: &Vec<*const i8>) {
    ext.extend(debug_ext);
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::{App, AppBuilder};

    #[test]
    fn test_build_instance_with_defaults() {

        let insatnce = InstanceBuilder::new()
            .build();

        assert_eq!(insatnce.is_ok(), false);

        let app: App<'_> = AppBuilder::new()
            .build()
            .unwrap();

        let insatnce = InstanceBuilder::new()
            .with_app(app.raw)
            .build();

        assert_eq!(insatnce.is_ok(), true)
    }

    #[test]
    fn test_custom_instance() {

    }

    #[test]
    fn test_check_support_layers() {

        let entry = unsafe { Entry::load().unwrap() };

        let required = vec![
            c"VK_LAYER_KHRONOS_validation".as_ptr()
        ];

        assert!(check_support_layers(&entry, &required).is_ok());

        let required2 = vec![
            c"VK_LAYER_MINECRAFT_validation".as_ptr()
        ];

        assert!(check_support_layers(&entry, &required2).is_err());
    }
}


#[cfg(debug_assertions)]
impl Drop for Instance {
    fn drop(&mut self) {
        if !self.destroyed {
            log::warn!("Instance не был уничтожен");
        }
    }
}