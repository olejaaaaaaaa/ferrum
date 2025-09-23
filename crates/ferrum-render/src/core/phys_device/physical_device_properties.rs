

#[derive(Clone, Copy)]
pub enum PhysicalProperties {
    V1(ash::vk::PhysicalDeviceProperties),
    V2(ash::vk::PhysicalDeviceProperties2<'static>),
    None
}

impl Default for PhysicalProperties {
    fn default() -> Self {
        PhysicalProperties::None
    }
}

impl std::ops::Deref for PhysicalProperties {
    type Target = ash::vk::PhysicalDeviceProperties;

    fn deref(&self) -> &Self::Target {
        match self {
            PhysicalProperties::V2(x) => &x.properties,
            PhysicalProperties::V1(x) => x,
            PhysicalProperties::None => panic!("Physical Properties is not initialize")
        }
    }
}

pub struct InternalData {
    pub features: super::features::Features,
    pub memory_prop: super::memory_properties::MemoryProperties,
    pub queue_family_prop: super::queue_family_properties::QueueFamilyProperties,
    pub phys_prop: PhysicalProperties,
    pub count_queue_family_prop: usize
}

use ash::vk::API_VERSION_1_0;

use crate::PhysicalDeviceBuilder;
impl<S> PhysicalDeviceBuilder<S> {
    pub fn get_physical_properties(instance: &ash::Instance, phys_dev: &ash::vk::PhysicalDevice, api_version: u32) -> InternalData {

        match api_version {
            _ => {

                unsafe  {

                    let _features = instance.get_physical_device_features(*phys_dev);
                    let _memory_prop = instance.get_physical_device_memory_properties(*phys_dev);
                    let mut _queue_family_prop = instance.get_physical_device_queue_family_properties(*phys_dev);
                    let count_familes = _queue_family_prop.len();

                    let _phys_prop = instance.get_physical_device_properties(*phys_dev);

                    InternalData {
                        features: super::features::Features::V1(_features),
                        memory_prop: super::memory_properties::MemoryProperties::V1(_memory_prop),
                        queue_family_prop: super::queue_family_properties::QueueFamilyProperties::V1(_queue_family_prop),
                        phys_prop: PhysicalProperties::V1(_phys_prop),
                        count_queue_family_prop: count_familes
                    }
                }
            },
            // _ => {

            //     use ash::vk;

            //     let mut features2 = vk::PhysicalDeviceFeatures2::default();
            //     unsafe { instance.get_physical_device_features2(*phys_dev, &mut features2) };

            //     let mut mem_props2 = vk::PhysicalDeviceMemoryProperties2::default();
            //     unsafe { instance.get_physical_device_memory_properties2(*phys_dev, &mut mem_props2) };

            //     // TODO: FIXME I must use get_physical_device_queue_family_properties2
            //     let mut _queue_prop2: Vec<vk::QueueFamilyProperties2<'_>> = vec![];
            //     let _queue_prop2 = unsafe { instance.get_physical_device_queue_family_properties(*phys_dev) };
            //     let queue_prop_len = _queue_prop2.len();
            //     let cached = _queue_prop2.iter().map(|x| x.clone()).collect::<Vec<_>>();

            //     let mut props2 = vk::PhysicalDeviceProperties2::default();

            //     unsafe { instance.get_physical_device_properties2(*phys_dev, &mut props2) };

            //     InternalData {
            //         features: super::features::Features::V2(features2),
            //         memory_prop: super::memory_properties::MemoryProperties::V2(mem_props2),
            //         queue_family_prop: super::queue_family_properties::QueueFamilyProperties::V2 { raw: vec![], cached: cached },
            //         phys_prop: PhysicalProperties::V2(props2),
            //         count_queue_family_prop: queue_prop_len,
            //     }
            // }
        }
    }
}

