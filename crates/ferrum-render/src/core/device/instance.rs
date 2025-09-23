use super::DeviceBuilder;
pub struct WithInstance<'n> {
    pub instance: &'n ash::Instance
}

impl<'n> DeviceBuilder<()> {
    pub fn with_instance(self, instance: &'n ash::Instance) -> DeviceBuilder<WithInstance<'n>> {
        DeviceBuilder { state: WithInstance {
            instance
        }}
    }
}