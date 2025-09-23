use super::PhysicalDeviceBuilder;

pub struct WithInstance<'n> {
    pub instance: &'n ash::Instance
}

impl PhysicalDeviceBuilder<()> {
    pub fn with_instance(self, instance: &ash::Instance) -> PhysicalDeviceBuilder<WithInstance> {
            PhysicalDeviceBuilder {

            state: WithInstance {
                instance
            },
            
            fn_select_phys_dev: self.fn_select_phys_dev
        }
    }
}