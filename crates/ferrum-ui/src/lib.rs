
// use std::{mem::ManuallyDrop, sync::{Arc, Mutex}};

// use ash::vk::{Format, Queue, SurfaceFormatKHR, SwapchainKHR};
// use egui_winit_ash_integration::{AllocatorTrait, Integration};
// use gpu_allocator::{vulkan::{Allocator, AllocatorCreateDesc}, AllocationSizes};
// use winit::raw_window_handle::HasWindowHandle;

// pub struct UiContext {
//    // pub ctx: Integration<Arc<Mutex<Allocator>>>
// }

// pub struct UiContextParams {
//     instance: ash::Instance,
//     device: ash::Device,
//     phys_dev: ash::vk::PhysicalDevice,
//     width: u32,
//     height: u32,
//     scale_factor: f64,
//     swapchain_loader: ash::khr::surface::Instance,
//     swapchain: SwapchainKHR,
//     format: SurfaceFormatKHR,
//     queue: Queue,
//     window: winit::window::Window
// }

// impl UiContext {
//     fn new(params: UiContextParams) {

//         let mut allocator = {
//             Allocator::new(&AllocatorCreateDesc {
//                 instance: params.instance.clone(),
//                 device: params.device.clone(),
//                 physical_device: params.phys_dev.clone(),
//                 debug_settings: Default::default(),
//                 buffer_device_address: false,
//                 allocation_sizes: AllocationSizes::default(),
//             }).expect("Error create ui allocator")
//         };

//         let allocator: Arc<std::sync::Mutex<gpu_allocator::vulkan::Allocator>> = Arc::new(Mutex::new(allocator));


//         /*

//             display_target: &H,
//             physical_width: u32,
//             physical_height: u32,
//             scale_factor: f64,
//             font_definitions: egui::FontDefinitions,
//             style: egui::Style,
//             device: Device,
//             allocator: A,
//             qfi: u32,
//             queue: vk::Queue,
//             swapchain_loader: Swapchain,
//             swapchain: vk::SwapchainKHR,
//             surface_format: vk::SurfaceFormatKHR,

//         */

//         let egui_integration = ManuallyDrop::new(egui_winit_ash_integration::Integration::new(
//             &params.window.window_handle(),
//             params.width,
//             params.height,
//             1.0,
//             egui::FontDefinitions::default(),
//             egui::Style::default(),
//             params.device,
//             allocator,
//             0,
//             params.queue,
//             params.swapchain_loader,
//             params.swapchain,
//             params.format
//         ));
//     }
// }