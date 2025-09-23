use crate::RenderContext;
use crate::VulkanResult;
use crate::{SamplerBuilder, Sampler};
use crate::VulkanError;

impl RenderContext {
    pub fn create_default_sampler(&self) -> VulkanResult<Sampler> {

        let sampler = SamplerBuilder::new(self.device.raw_device())
            .build()
            .map_err(|e| VulkanError::Unknown)?;

        Ok(sampler)
    }
}