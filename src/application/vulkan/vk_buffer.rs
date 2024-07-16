use ash::vk::{BufferCopy, BufferCreateInfo, BufferUsageFlags, DeviceAddress};
use vk_mem::Alloc;
use vk_mem::{AllocationCreateFlags, AllocationCreateInfo, MemoryUsage};

use crate::application::vk_app::{AllocatedBuffer, VulkanApp};

pub struct BufferGPU {
    pub buffer: AllocatedBuffer,
    pub buffer_address: DeviceAddress,
}

impl VulkanApp {
    pub fn create_buffer(
        &self,
        alloc_size: usize,
        usage_flags: BufferUsageFlags,
        memory_usage: MemoryUsage,
    ) -> AllocatedBuffer {
        let buffer_create_info = BufferCreateInfo::default()
            .size(alloc_size as u64)
            .usage(usage_flags);

        let allocation_create_info = AllocationCreateInfo {
            flags: AllocationCreateFlags::MAPPED,
            usage: memory_usage,
            ..Default::default()
        };

        let (buffer, allocation) = unsafe {
            let allocator = self.allocator.allocator.lock().unwrap();
            allocator
                .create_buffer(&buffer_create_info, &allocation_create_info)
                .unwrap()
        };

        AllocatedBuffer { buffer, allocation }
    }

    pub fn clear_buffer(allocator: &vk_mem::Allocator, buffer: &mut AllocatedBuffer) {
        unsafe {
            allocator.destroy_buffer(buffer.buffer, &mut buffer.allocation);
        }
    }
}

impl BufferGPU {
    pub fn upload_elements<T>(elements: &[T], application: &VulkanApp) -> BufferGPU {
        let size = std::mem::size_of_val(elements);

        // create triangles buffer
        let buffer = application.create_buffer(
            size,
            BufferUsageFlags::STORAGE_BUFFER | BufferUsageFlags::TRANSFER_DST,
            // | BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            vk_mem::MemoryUsage::GpuOnly,
        );

        // // find the address of the buffer
        // let buffer_address_info = BufferDeviceAddressInfo::default()
        //     .buffer(buffer.buffer)
        // ;

        // let buffer_address = unsafe {
        //     application
        //         .device
        //         .get_buffer_device_address(&buffer_address_info)
        // };

        let surface = BufferGPU {
            buffer,
            buffer_address: DeviceAddress::default(),
        };

        let mut staging = application.create_buffer(
            size,
            BufferUsageFlags::TRANSFER_SRC,
            vk_mem::MemoryUsage::CpuOnly,
        );

        // Lock the allocator and map the staging buffer
        {
            let allocator = application.allocator.allocator.lock().unwrap();
            let data = allocator
                .get_allocation_info(&staging.allocation)
                .mapped_data as *mut u8;

            // copy buffers
            unsafe {
                let data_slice = std::slice::from_raw_parts_mut(data, size);
                let input_slice = std::slice::from_raw_parts(elements.as_ptr() as *const u8, size);
                data_slice.copy_from_slice(input_slice);
            };
        } // The lock on the allocator is released here

        application.immediate_submit(&|application, cmd| {
            let elements_copy = [BufferCopy::default()
                .dst_offset(0)
                .src_offset(0)
                .size(size as u64)];

            let device = &application.device;
            unsafe {
                device.cmd_copy_buffer(cmd, staging.buffer, surface.buffer.buffer, &elements_copy);
            };
        });

        // Lock the allocator again to destroy the staging buffer
        VulkanApp::clear_buffer(
            &application.allocator.allocator.lock().unwrap(),
            &mut staging,
        );

        surface
    }
}
