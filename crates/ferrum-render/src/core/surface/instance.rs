use super::entry::WithEntry;
use super::SurfaceBuilder;

pub struct WithInstance<'n> {
    pub entry: &'n ash::Entry,
    pub instance: &'n ash::Instance
}

impl<'n> SurfaceBuilder<WithEntry<'n>> {
    pub fn with_instance(self, instance: &'n ash::Instance) -> SurfaceBuilder<WithInstance<'n>> {
        SurfaceBuilder { state: WithInstance {
            entry: self.state.entry,
            instance
        }}
    }
}