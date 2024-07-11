use ash::{vk::{DescriptorImageInfo, DescriptorPool, DescriptorPoolCreateInfo, DescriptorPoolResetFlags, DescriptorPoolSize, DescriptorSet, DescriptorSetAllocateInfo, DescriptorSetLayout, DescriptorSetLayoutBinding, DescriptorSetLayoutCreateFlags, DescriptorSetLayoutCreateInfo, DescriptorType, ImageLayout, ShaderStageFlags, WriteDescriptorSet}, Device};

use crate::application::vk_app::{AllocatedImage, Descriptors, VulkanApp};

#[derive(Default)]
pub struct DescriptorLayoutBuilder<'a> {
    pub bindings: Vec<DescriptorSetLayoutBinding<'a>>,
}

impl <'a> DescriptorLayoutBuilder<'a>{
    pub fn add_binding(&mut self, binding: u32, descriptor_type: DescriptorType){
        let new_bind = DescriptorSetLayoutBinding::default()
            .binding(binding)
            .descriptor_count(1)
            .descriptor_type(descriptor_type)
        ;
        self.bindings.push(new_bind);
    }

    pub fn build(&mut self, device: &Device, shader_stages: ShaderStageFlags, flags: DescriptorSetLayoutCreateFlags) -> DescriptorSetLayout {
        self.bindings.iter_mut().for_each(|binding| {
            binding.stage_flags |= shader_stages;
        });

        let create_info = DescriptorSetLayoutCreateInfo::default()
            .bindings(&self.bindings)
            .flags(flags);
        
        unsafe {
            device.create_descriptor_set_layout(&create_info, None).unwrap()
        }
    }
}

#[derive(Default)]
pub struct PoolSizeRatio{
    descriptor_type: DescriptorType,
    ratio: f32
}

#[derive(Default)]
pub struct DescriptorAllocator {
    descriptor_pool: DescriptorPool,
}

impl DescriptorAllocator {
    pub fn init_pool(&mut self, device: &Device, max_sets: u32, pool_ratios: &[PoolSizeRatio]){
        let pool_sizes = pool_ratios.into_iter().map(|ratio|{
            DescriptorPoolSize::default()
                .ty(ratio.descriptor_type)
                .descriptor_count((ratio.ratio * (max_sets as f32)) as u32)
        }).collect::<Vec<DescriptorPoolSize>>();
        
        let pool_create_info = DescriptorPoolCreateInfo::default()
            .max_sets(max_sets)
            .pool_sizes(&pool_sizes)
        ;

        unsafe {
            self.descriptor_pool = device.create_descriptor_pool(&pool_create_info, None).unwrap();
        }
    }

    pub fn clear_descriptors(&mut self, device: &Device){
        unsafe { device.reset_descriptor_pool(self.descriptor_pool, DescriptorPoolResetFlags::empty()).unwrap(); }
    }

    pub fn clear_pool(&mut self, device: &Device){
        unsafe { device.destroy_descriptor_pool(self.descriptor_pool, None); }
    }

    pub fn allocate(&mut self, device: &Device, descriptor_set_layout: &DescriptorSetLayout) -> DescriptorSet {
        let layouts = [*descriptor_set_layout];
        let allocate_info = DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.descriptor_pool)
            .set_layouts(&layouts)
        ;
        unsafe { device.allocate_descriptor_sets(&allocate_info).unwrap()[0] }
    }
}

impl VulkanApp{
    pub fn init_descriptors(device: &Device, draw_image: &AllocatedImage) -> Descriptors {
        // create a descriptor pool that will hold 10 sets with 1 image each
        let pool_size_ratios = [
            PoolSizeRatio{ descriptor_type: DescriptorType::STORAGE_IMAGE, ratio: 1.0 }
        ];

        let mut global_allocator_descriptor = DescriptorAllocator::default();
        global_allocator_descriptor.init_pool(device, 10, &pool_size_ratios);


        // make the descriptor set layout for our compute draw
        let mut builder = DescriptorLayoutBuilder::default();
        builder.add_binding(0, DescriptorType::STORAGE_IMAGE);
        let draw_image_descriptor_layout = builder.build(
            device, 
            ShaderStageFlags::COMPUTE, 
            DescriptorSetLayoutCreateFlags::empty()
        );


        // allocate a descriptor set for our draw image
        let draw_image_descriptors = global_allocator_descriptor.allocate(device, &draw_image_descriptor_layout);

        let descriptor_image_info = [DescriptorImageInfo::default()
            .image_view(draw_image.image_view)
            .image_layout(ImageLayout::GENERAL)
        ];
        let descriptor_writes = [ WriteDescriptorSet::default()
            .dst_binding(0)
            .dst_set(draw_image_descriptors)
            .descriptor_count(1)
            .descriptor_type(DescriptorType::STORAGE_IMAGE)
            .image_info(&descriptor_image_info)
        ];

        unsafe {
            device.update_descriptor_sets(&descriptor_writes, &[]);
        }

        Descriptors {
            global_allocator_descriptor,
            draw_image_descriptors,
            draw_image_descriptor_layout,
        }

    }

    pub fn clear_descriptors(&mut self){
        unsafe {
            self.descriptors.global_allocator_descriptor.clear_pool(&self.device);
            self.device.destroy_descriptor_set_layout(self.descriptors.draw_image_descriptor_layout, None);
        }
    }
}