
pub struct SubpassDescription<'n> {
    pub raw: ash::vk::SubpassDescription<'n>
}

impl<'n> SubpassDescription<'n> {
    pub fn default() -> Self {
        Self {
            raw: ash::vk::SubpassDescription::default()
        }
    }
}

