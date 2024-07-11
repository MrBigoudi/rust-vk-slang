use ash::{
    vk::{
        DescriptorPool, DescriptorPoolCreateInfo, DescriptorPoolResetFlags, DescriptorPoolSize,
        DescriptorSet, DescriptorSetAllocateInfo, DescriptorSetLayout, DescriptorSetLayoutBinding,
        DescriptorSetLayoutCreateFlags, DescriptorSetLayoutCreateInfo, DescriptorType,
        ShaderStageFlags,
    },
    Device,
};

#[derive(Default)]
pub struct DescriptorLayoutBuilder<'a> {
    pub bindings: Vec<DescriptorSetLayoutBinding<'a>>,
}

impl<'a> DescriptorLayoutBuilder<'a> {
    pub fn add_binding(&mut self, binding: u32, descriptor_type: DescriptorType) {
        let new_bind = DescriptorSetLayoutBinding::default()
            .binding(binding)
            .descriptor_count(1)
            .descriptor_type(descriptor_type);
        self.bindings.push(new_bind);
    }

    pub fn build(
        &mut self,
        device: &Device,
        shader_stages: ShaderStageFlags,
        flags: DescriptorSetLayoutCreateFlags,
    ) -> DescriptorSetLayout {
        self.bindings.iter_mut().for_each(|binding| {
            binding.stage_flags |= shader_stages;
        });

        let create_info = DescriptorSetLayoutCreateInfo::default()
            .bindings(&self.bindings)
            .flags(flags);

        unsafe {
            device
                .create_descriptor_set_layout(&create_info, None)
                .unwrap()
        }
    }
}

#[derive(Default)]
pub struct PoolSizeRatio {
    pub descriptor_type: DescriptorType,
    pub ratio: f32,
}

#[derive(Default)]
pub struct DescriptorAllocator {
    pub descriptor_pool: DescriptorPool,
}

impl DescriptorAllocator {
    pub fn init_pool(&mut self, device: &Device, max_sets: u32, pool_ratios: &[PoolSizeRatio]) {
        let pool_sizes = pool_ratios
            .iter()
            .map(|ratio| {
                DescriptorPoolSize::default()
                    .ty(ratio.descriptor_type)
                    .descriptor_count((ratio.ratio * (max_sets as f32)) as u32)
            })
            .collect::<Vec<DescriptorPoolSize>>();

        let pool_create_info = DescriptorPoolCreateInfo::default()
            .max_sets(max_sets)
            .pool_sizes(&pool_sizes);

        unsafe {
            self.descriptor_pool = device
                .create_descriptor_pool(&pool_create_info, None)
                .unwrap();
        }
    }

    pub fn clear_descriptors(&mut self, device: &Device) {
        unsafe {
            device
                .reset_descriptor_pool(self.descriptor_pool, DescriptorPoolResetFlags::empty())
                .unwrap();
        }
    }

    pub fn clear_pool(&mut self, device: &Device) {
        unsafe {
            device.destroy_descriptor_pool(self.descriptor_pool, None);
        }
    }

    pub fn allocate(
        &mut self,
        device: &Device,
        descriptor_set_layout: &DescriptorSetLayout,
    ) -> DescriptorSet {
        let layouts = [*descriptor_set_layout];
        let allocate_info = DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.descriptor_pool)
            .set_layouts(&layouts);
        unsafe { device.allocate_descriptor_sets(&allocate_info).unwrap()[0] }
    }
}
