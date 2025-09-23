use super::instance::WithInstance;
use super::PhysicalDeviceBuilder;

pub struct WithApiVersion<'n> {
    pub instance: &'n ash::Instance,
    pub api_version: u32
}

impl<'n> PhysicalDeviceBuilder<WithInstance<'n>> {
    pub fn with_api_version(self, api_version: u32) -> PhysicalDeviceBuilder<WithApiVersion<'n>> {
        PhysicalDeviceBuilder {

            state: WithApiVersion {
                instance: self.state.instance,
                api_version
            },

            fn_select_phys_dev: self.fn_select_phys_dev

        }
    }
}