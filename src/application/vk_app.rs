use std::collections::HashSet;

use log::debug;

use ash::{
    ext::debug_utils,
    khr::{surface, swapchain},
    vk::{
        self, ClearColorValue, CommandBuffer, CommandBufferBeginInfo, CommandBufferResetFlags,
        CommandBufferSubmitInfo, CommandBufferUsageFlags, CommandPool, Extent2D, Fence, Format,
        Image, ImageAspectFlags, ImageLayout, ImageSubresourceRange, ImageView,
        PipelineStageFlags2, PresentInfoKHR, PresentModeKHR, Queue, Semaphore, SemaphoreSubmitInfo,
        SubmitInfo2, SurfaceCapabilitiesKHR, SurfaceFormatKHR, SwapchainKHR,
        REMAINING_ARRAY_LAYERS, REMAINING_MIP_LEVELS,
    },
    Device, Entry, Instance,
};

use vk_mem::Allocator;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoopWindowTarget,
    keyboard::{Key, NamedKey},
};

/// Structure to hold application parameters such as name, window width, and window height.
pub struct AppParameters {
    pub name: String,
    pub window_width: i32,
    pub window_height: i32,
}

impl Default for AppParameters {
    /// Provides default values for the `AppParameters` structure.
    fn default() -> Self {
        Self {
            name: String::from("Rulkan"),
            window_width: 1280,
            window_height: 720,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct FrameData {
    pub command_pool: CommandPool,
    pub command_buffer: CommandBuffer,
    pub swapchain_semaphore: Semaphore,
    pub render_semaphore: Semaphore,
    pub render_fence: Fence,
}

impl FrameData {
    pub fn check(&self) {
        assert!(self.render_fence != Fence::null());
        assert!(self.render_semaphore != Semaphore::null());
        assert!(self.swapchain_semaphore != Semaphore::null());
        assert!(self.command_pool != CommandPool::null());
        assert!(self.command_buffer != CommandBuffer::null());
    }
}

pub const FRAME_OVERLAP: usize = 2;

/// Main structure to hold Vulkan application components.
pub struct VulkanApp {
    pub app_params: AppParameters,
    pub entry: Entry,
    pub instance: Instance,
    pub debug_utils_loader: debug_utils::Instance,
    pub debug_call_back: vk::DebugUtilsMessengerEXT,

    pub surface: vk::SurfaceKHR,
    pub surface_loader: surface::Instance,
    pub physical_device: vk::PhysicalDevice,
    pub queue_families: QueueFamilyIndices,
    pub device: Device,

    pub swapchain_loader: swapchain::Device,
    pub swapchain: SwapchainKHR,
    pub swapchain_images: Vec<Image>,
    pub swapchain_image_format: Format,
    pub swapchain_extent: Extent2D,
    pub swapchain_image_views: Vec<ImageView>,

    pub frames: [FrameData; FRAME_OVERLAP],
    pub frame_number: usize,

    pub allocator: Allocator,
}

pub const DEVICE_EXTENSION_NAMES_RAW: [*const i8; 1] = [swapchain::NAME.as_ptr()];

pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
    pub graphics_queue: Queue,
    pub present_family: Option<u32>,
    pub present_queue: Queue,
}

impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }

    pub fn get_unique_queues(&self) -> HashSet<u32> {
        let mut set: HashSet<u32> = HashSet::new();
        set.insert(self.graphics_family.unwrap());
        set.insert(self.present_family.unwrap());
        set
    }
}

pub struct SwapChainSupportDetails {
    pub capabilities: SurfaceCapabilitiesKHR,
    pub formats: Vec<SurfaceFormatKHR>,
    pub present_modes: Vec<PresentModeKHR>,
}

impl SwapChainSupportDetails {
    pub fn is_complete(&self) -> bool {
        !self.formats.is_empty() && !self.present_modes.is_empty()
    }
}

impl VulkanApp {
    pub fn draw(&mut self) {
        let current_frame = self.get_current_frame();

        let fences = &[current_frame.render_fence];
        let timeout = 1e9 as u64; // in nanoseconds

        // wait until the gpu has finished rendering the last frame. Timeout of 1 second
        unsafe {
            self.device.wait_for_fences(fences, true, timeout).unwrap();
            self.device.reset_fences(fences).unwrap();
        }

        //request image from the swapchain
        let swaphchain_semaphore = current_frame.swapchain_semaphore;
        let swapchain_image_index = unsafe {
            self.swapchain_loader
                .acquire_next_image(self.swapchain, timeout, swaphchain_semaphore, Fence::null())
                .unwrap()
                .0 as usize
        };

        let command_buffer = current_frame.command_buffer;

        // now that we are sure that the commands finished executing, we can safely
        // reset the command buffer to begin recording again.
        unsafe {
            self.device
                .reset_command_buffer(command_buffer, CommandBufferResetFlags::empty())
                .unwrap()
        };

        // begin the command buffer recording. We will use this command buffer exactly once, so we want to let vulkan know that
        let command_buffer_begin_info =
            CommandBufferBeginInfo::default().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        // start the command buffer recording
        unsafe {
            self.device
                .begin_command_buffer(command_buffer, &command_buffer_begin_info)
                .unwrap()
        };

        // make the swapchain image into writeable mode before rendering
        let image = self.swapchain_images[swapchain_image_index];
        Self::transition_image(
            &self.device,
            &command_buffer,
            &image,
            &ImageLayout::UNDEFINED,
            &ImageLayout::GENERAL,
        );

        // background color
        let flash = (self.frame_number as f32 / 120.).sin().abs();
        let clear_color_value: ClearColorValue = ClearColorValue {
            float32: [0., 0., flash, 1.],
        };

        let clear_range = ImageSubresourceRange::default()
            .aspect_mask(ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(REMAINING_MIP_LEVELS)
            .base_array_layer(0)
            .layer_count(REMAINING_ARRAY_LAYERS);

        // clear image
        let clear_ranges = [clear_range];
        unsafe {
            self.device.cmd_clear_color_image(
                command_buffer,
                image,
                ImageLayout::GENERAL,
                &clear_color_value,
                &clear_ranges,
            );
        }

        // make the swapchain image into presentable mode
        Self::transition_image(
            &self.device,
            &command_buffer,
            &image,
            &ImageLayout::GENERAL,
            &ImageLayout::PRESENT_SRC_KHR,
        );

        // finalize the command buffer
        unsafe { self.device.end_command_buffer(command_buffer).unwrap() };

        // prepare the submission to the queue
        // wait on the present semaphore (signaled when the swapchain is ready)
        // signal the render semaphore (to signal that rendering has finished)
        let command_buffer_submit_infos = [CommandBufferSubmitInfo::default()
            .command_buffer(command_buffer)
            .device_mask(0)];
        let wait_swapchain_semaphore_infos = [SemaphoreSubmitInfo::default()
            .semaphore(current_frame.swapchain_semaphore)
            .stage_mask(PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT_KHR)];
        let signal_rendering_done_infos = [SemaphoreSubmitInfo::default()
            .semaphore(current_frame.render_semaphore)
            .stage_mask(PipelineStageFlags2::ALL_GRAPHICS)];
        let submit_infos = [SubmitInfo2::default()
            .wait_semaphore_infos(&wait_swapchain_semaphore_infos)
            .signal_semaphore_infos(&signal_rendering_done_infos)
            .command_buffer_infos(&command_buffer_submit_infos)];

        // submit command buffer to the queue and execute it
        // render fence will now block until the graphic commands finish execution
        unsafe {
            self.device
                .queue_submit2(
                    self.queue_families.graphics_queue,
                    &submit_infos,
                    current_frame.render_fence,
                )
                .unwrap()
        };

        // prepare present
        // this will put the image we just rendered to into the visible window.
        // we want to wait on the _renderSemaphore for that,
        // as its necessary that drawing commands have finished before the image is displayed to the user
        let present_info_swapchains = [self.swapchain];
        let present_info_wait_semaphores = [current_frame.render_semaphore];
        let present_info_image_indices: [u32; 1] = [swapchain_image_index as u32];
        let present_info = PresentInfoKHR::default()
            .swapchains(&present_info_swapchains)
            .wait_semaphores(&present_info_wait_semaphores)
            .image_indices(&present_info_image_indices);

        unsafe {
            self.swapchain_loader
                .queue_present(self.queue_families.graphics_queue, &present_info)
                .unwrap();
        }

        self.frame_number += 1;
    }

    pub fn input_handler(&mut self, event: &Event<()>, elwt: &EventLoopWindowTarget<()>) {
        if let Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            logical_key: Key::Named(NamedKey::Escape),
                            ..
                        },
                    ..
                },
            ..
        } = event
        {
            elwt.exit();
        }
    }

    pub fn run(app_params: AppParameters) {
        debug!("Init Event Loop...");
        let event_loop = Self::init_event_loop();
        debug!("Ok\n");
        debug!("Init Window...");
        let window = Self::init_window(&app_params, &event_loop);
        debug!("Ok\n");

        let mut application = Self::init(app_params, &window);

        let _ = event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    elwt.exit();
                }
                Event::AboutToWait => {
                    window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    application.draw();
                }
                _ => (),
            };
            application.input_handler(&event, elwt);
        });
    }
}
