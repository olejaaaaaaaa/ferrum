#![allow(warnings)]
use std::{borrow::Cow, error::Error, ffi::{c_void, CStr}, fs::File, io::{Read, Write}};
use ash::{vk::{DebugUtilsMessageSeverityFlagsEXT, ExtensionProperties, LayerProperties, MemoryHeapFlags}, Entry};

use ash::vk;
use log::info;
use core::ffi::{self, c_char};

pub unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {

    let callback_data = *p_callback_data;

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        ffi::CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    if message_severity == DebugUtilsMessageSeverityFlagsEXT::VERBOSE {
        log::warn!("{}", message);
    }

    if message_severity == DebugUtilsMessageSeverityFlagsEXT::ERROR {
        log::error!("{}", message)
    }

    // if message_severity == DebugUtilsMessageSeverityFlagsEXT::INFO {
    //     log::info!("{}", message)
    // }

    vk::FALSE
}

pub fn find_memorytype_index(
    memory_req: &vk::MemoryRequirements,
    memory_prop: &vk::PhysicalDeviceMemoryProperties,
    flags: vk::MemoryPropertyFlags,
) -> Option<u32> {
    memory_prop.memory_types[..memory_prop.memory_type_count as _]
        .iter()
        .enumerate()
        .find(|(index, memory_type)| {
            (1 << index) & memory_req.memory_type_bits != 0
                && memory_type.property_flags & flags == flags
        })
        .map(|(index, _memory_type)| index as _)
}

