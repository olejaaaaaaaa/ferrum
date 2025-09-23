use std::any::Any;
use std::sync::Arc;
use ash::vk;

use ferrum_render::descriptor_pool::DescriptorPoolBuilder;
use ferrum_render::descriptor_set_layout::DescriptorSetLayoutBuilder;
use ferrum_render::GraphicsDevice;

pub struct DescriptorPool {
    raw: vk::DescriptorPool,
    sizes: Vec<vk::DescriptorPoolSize>,
    available_sets: u64
}

#[derive(Clone, Copy)]
pub struct DescriptorSet {
    pub raw: vk::DescriptorSet,
}

pub struct DescriptorManager {
    device: Arc<GraphicsDevice>,
    pools: Vec<DescriptorPool>,
    sets: Vec<DescriptorSet>,
    layout: Vec<vk::DescriptorSetLayout>
}

impl DescriptorManager {

    pub fn new(device: Arc<GraphicsDevice>) -> Self {
        Self {
            pools: vec![],
            sets: vec![],
            layout: vec![],
            device
        }
    }

    pub fn builder(&mut self) -> DescriptorRequest {
        DescriptorRequest::new(self)
    }

    // Добавляем методы для управления ресурсами
    pub fn cleanup(&mut self) {
        unsafe {
            // Очищаем пулы дескрипторов
            for pool in &self.pools {
                self.device.logical_device.raw
                    .destroy_descriptor_pool(pool.raw, None);
            }
            
            // Очищаем лейауты
            for layout in &self.layout {
                self.device.logical_device.raw
                    .destroy_descriptor_set_layout(*layout, None);
            }
        }
        
        self.pools.clear();
        self.sets.clear();
        self.layout.clear();
    }
}

pub struct DescriptorRequest<'n> {
    manager: &'n mut DescriptorManager,
    binds: Vec<(u32, vk::DescriptorType, vk::ShaderStageFlags, &'n dyn Any)>
}

impl<'n> DescriptorRequest<'n> {
    fn new(manager: &'n mut DescriptorManager) -> Self {
        Self {
            manager,
            binds: vec![]
        }
    }

    pub fn with(&mut self, bind: u32, _type: vk::DescriptorType, stage: vk::ShaderStageFlags, data: &'n dyn Any) -> &mut Self {
        self.binds.push((bind, _type, stage, data));
        self
    }

    pub fn send(&mut self) -> Result<usize, String> {
        let mut bindings = vec![];

        for (bind, _type, stage, _) in &self.binds {
            bindings.push(
                vk::DescriptorSetLayoutBinding::default()
                    .binding(*bind)
                    .descriptor_type(*_type)
                    .descriptor_count(1)
                    .stage_flags(*stage)
            );
        }

        let layout = DescriptorSetLayoutBuilder::new()
            .with_device(self.manager.device.raw_device())
            .with_bindings(&bindings)
            .build();

        let mut pool_sizes = vec![];

        for (_bind, _type, _stage, _) in &self.binds {
            pool_sizes.push(
                vk::DescriptorPoolSize::default()
                    .ty(*_type)
                    .descriptor_count(1)
            );
        }

        let descriptor_pool = DescriptorPoolBuilder::new()
            .with_device(self.manager.device.raw_device())
            .with_max_sets(1)
            .with_pool_sizes(&pool_sizes)
            .build();

        let layout_slice = std::slice::from_ref(&layout.raw);
        let allocate_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(descriptor_pool.raw)
            .set_layouts(layout_slice);

        let descriptor_set = unsafe {
            self.manager.device.logical_device.raw
                .allocate_descriptor_sets(&allocate_info)
                .map_err(|e| format!("Failed to allocate descriptor sets: {:?}", e))?[0]
        };

        // Обновляем дескрипторные сеты
        let mut writes = vec![];

        for (bind, _type, _stage, data) in &self.binds {
            match *_type {
                vk::DescriptorType::UNIFORM_BUFFER => {
                    if let Some(buffer_info) = data.downcast_ref::<vk::DescriptorBufferInfo>() {
                        writes.push(
                            vk::WriteDescriptorSet::default()
                                .dst_set(descriptor_set)
                                .dst_binding(*bind)
                                .dst_array_element(0)
                                .descriptor_type(*_type)
                                .buffer_info(std::slice::from_ref(buffer_info))
                        );
                    }
                },
                vk::DescriptorType::COMBINED_IMAGE_SAMPLER => {
                    if let Some(image_info) = data.downcast_ref::<vk::DescriptorImageInfo>() {
                        writes.push(
                            vk::WriteDescriptorSet::default()
                                .dst_set(descriptor_set)
                                .dst_binding(*bind)
                                .dst_array_element(0)
                                .descriptor_type(*_type)
                                .image_info(std::slice::from_ref(image_info))
                        );
                    }
                },
                vk::DescriptorType::STORAGE_BUFFER => {
                    if let Some(buffer_info) = data.downcast_ref::<vk::DescriptorBufferInfo>() {
                        writes.push(
                            vk::WriteDescriptorSet::default()
                                .dst_set(descriptor_set)
                                .dst_binding(*bind)
                                .dst_array_element(0)
                                .descriptor_type(*_type)
                                .buffer_info(std::slice::from_ref(buffer_info))
                        );
                    }
                },
                _ => {
                    return Err(format!("Unsupported descriptor type: {:?}", _type));
                }
            }
        }

        unsafe {
            self.manager.device.logical_device.raw
                .update_descriptor_sets(&writes, &[]);
        }

        let pool = DescriptorPool {
            raw: descriptor_pool.raw,
            sizes: pool_sizes,
            available_sets: 0,
        };

        let set = DescriptorSet {
            raw: descriptor_set,
        };

        self.manager.pools.push(pool);
        self.manager.sets.push(set);
        self.manager.layout.push(layout.raw);

        // Возвращаем индекс созданного дескрипторного сета
        Ok(self.manager.sets.len() - 1)
    }
}

// Дополнительные утилитарные методы для DescriptorManager
impl DescriptorManager {
    pub fn get_descriptor_set(&self, index: usize) -> Option<DescriptorSet> {
        self.sets.get(index).copied()
    }

    pub fn get_layout(&self, index: usize) -> Option<vk::DescriptorSetLayout> {
        self.layout.get(index).copied()
    }

    pub fn sets_count(&self) -> usize {
        self.sets.len()
    }

    pub fn pools_count(&self) -> usize {
        self.pools.len()
    }
}


// Пример использования:
/*
let buffer_info = vk::DescriptorBufferInfo::default()
    .buffer(uniform_buffer)
    .offset(0)
    .range(vk::WHOLE_SIZE);

let image_info = vk::DescriptorImageInfo::default()
    .sampler(sampler)
    .image_view(image_view)
    .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);

let descriptor_set_index = descriptor_manager
    .builder()
    .with(0, vk::DescriptorType::UNIFORM_BUFFER, vk::ShaderStageFlags::VERTEX, &buffer_info)
    .with(1, vk::DescriptorType::COMBINED_IMAGE_SAMPLER, vk::ShaderStageFlags::FRAGMENT, &image_info)
    .send()?;
*/