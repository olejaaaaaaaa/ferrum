use crate::SurfaceBuilder;

pub struct WithEntry<'n> {
    pub entry: &'n ash::Entry
}

impl SurfaceBuilder<()> {
    pub fn with_entry<'n>(self, entry: &'n ash::Entry) -> SurfaceBuilder<WithEntry<'n>> {
        SurfaceBuilder { state: WithEntry { entry } }
    }
}