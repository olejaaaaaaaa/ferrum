use ash::vk;

use crate::VulkanResult;

pub struct Sampler {
    pub raw: ash::vk::Sampler,
}

pub struct SamplerBuilder<'n> {
    device: &'n ash::Device,
    mag_filter: vk::Filter,
    min_filter: vk::Filter,
    address_mode_u: vk::SamplerAddressMode,
    address_mode_v: vk::SamplerAddressMode,
    address_mode_w: vk::SamplerAddressMode,
    anisotropy_enable: bool,
    max_anisotropy: f32,
    border_color: vk::BorderColor,
    unnormalized_coordinates: bool,
    compare_enable: bool,
    compare_op: vk::CompareOp,
    mipmap_mode: vk::SamplerMipmapMode,
    mip_lod_bias: f32,
    min_lod: f32,
    max_lod: f32,
}

impl<'n> SamplerBuilder<'n> {

    pub fn new(device: &'n ash::Device) -> Self {
        Self {
            mag_filter: vk::Filter::LINEAR,
            min_filter: vk::Filter::LINEAR,
            address_mode_u: vk::SamplerAddressMode::REPEAT,
            address_mode_v: vk::SamplerAddressMode::REPEAT,
            address_mode_w: vk::SamplerAddressMode::REPEAT,
            anisotropy_enable: false,
            max_anisotropy: 1.0,
            border_color: vk::BorderColor::INT_OPAQUE_BLACK,
            unnormalized_coordinates: false,
            compare_enable: false,
            compare_op: vk::CompareOp::ALWAYS,
            mipmap_mode: vk::SamplerMipmapMode::LINEAR,
            mip_lod_bias: 0.0,
            min_lod: 0.0,
            max_lod: 0.0,
            device,
        }
    }

    pub fn mag_filter(mut self, value: vk::Filter) -> Self {
        self.mag_filter = value;
        self
    }

    pub fn min_filter(mut self, value: vk::Filter) -> Self {
        self.min_filter = value;
        self
    }

    pub fn address_mode_u(mut self, value: vk::SamplerAddressMode) -> Self {
        self.address_mode_u = value;
        self
    }

    pub fn address_mode_v(mut self, value: vk::SamplerAddressMode) -> Self {
        self.address_mode_v = value;
        self
    }

    pub fn address_mode_w(mut self, value: vk::SamplerAddressMode) -> Self {
        self.address_mode_w = value;
        self
    }

    pub fn anisotropy_enable(mut self, value: bool) -> Self {
        self.anisotropy_enable = value;
        self
    }

    pub fn max_anisotropy(mut self, value: f32) -> Self {
        self.max_anisotropy = value;
        self
    }

    pub fn border_color(mut self, value: vk::BorderColor) -> Self {
        self.border_color = value;
        self
    }

    pub fn unnormalized_coordinates(mut self, value: bool) -> Self {
        self.unnormalized_coordinates = value;
        self
    }

    pub fn compare_enable(mut self, value: bool) -> Self {
        self.compare_enable = value;
        self
    }

    pub fn compare_op(mut self, value: vk::CompareOp) -> Self {
        self.compare_op = value;
        self
    }

    pub fn mipmap_mode(mut self, value: vk::SamplerMipmapMode) -> Self {
        self.mipmap_mode = value;
        self
    }

    pub fn mip_lod_bias(mut self, value: f32) -> Self {
        self.mip_lod_bias = value;
        self
    }

    pub fn min_lod(mut self, value: f32) -> Self {
        self.min_lod = value;
        self
    }

    pub fn max_lod(mut self, value: f32) -> Self {
        self.max_lod = value;
        self
    }

    pub fn build(self) -> VulkanResult<Sampler> {

        let create_info = vk::SamplerCreateInfo::default()
            .mag_filter(self.mag_filter)
            .min_filter(self.min_filter)
            .address_mode_u(self.address_mode_u)
            .address_mode_v(self.address_mode_v)
            .address_mode_w(self.address_mode_w)
            .anisotropy_enable(self.anisotropy_enable)
            .max_anisotropy(self.max_anisotropy)
            .border_color(self.border_color)
            .unnormalized_coordinates(self.unnormalized_coordinates)
            .compare_enable(self.compare_enable)
            .compare_op(self.compare_op)
            .mipmap_mode(self.mipmap_mode)
            .mip_lod_bias(self.mip_lod_bias)
            .min_lod(self.min_lod)
            .max_lod(self.max_lod);

        let sampler = unsafe {
            self.device.create_sampler(&create_info, None).map_err(|_| crate::VulkanError::Unknown)?
        };

        Ok(Sampler { raw: sampler })
    }
}