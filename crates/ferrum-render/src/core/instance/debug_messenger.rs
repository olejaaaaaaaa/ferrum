

pub fn create_debug_utils_messenger(entry: &ash::Entry, instance: &ash::Instance) -> VulkanResult<(ash::ext::debug_utils::Instance, ash::vk::DebugUtilsMessengerEXT)> {

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
