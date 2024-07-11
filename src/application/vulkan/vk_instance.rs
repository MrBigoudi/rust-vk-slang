use log::debug;

use winit::{raw_window_handle::HasDisplayHandle, window::Window};

use std::ffi::{c_char, CStr, CString};

use crate::application::vk_app::{AppParameters, VulkanApp};
use ash::{
    ext::debug_utils,
    vk::{self, API_VERSION_1_3},
    Entry, Instance,
};

impl VulkanApp {
    /// Initializes the Vulkan instance with the required extensions and layers.
    pub fn init_instance(app_params: &AppParameters, window: &Window, entry: &Entry) -> Instance {
        // Get the required extensions
        let mut required_extensions = ash_window::enumerate_required_extensions(
            window
                .display_handle()
                .unwrap_or_else(|err| panic!("Failed to get the required extensions: {:?}\n", err))
                .as_raw(),
        )
        .unwrap()
        .to_vec();
        required_extensions.push(debug_utils::NAME.as_ptr());
        debug!("Extensions:");
        for extension in &required_extensions {
            let extension_name = unsafe { CStr::from_ptr(*extension).to_string_lossy() };
            debug!("\t{}", extension_name);
        }

        // Get the required layers
        let required_layers_cstr = [
            unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0") },
            // unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_RENDERDOC_Capture\0") },
        ];
        let required_layers: Vec<*const c_char> = required_layers_cstr
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();
        debug!("Layers:");
        for layer in required_layers_cstr {
            debug!("\t{:?}", layer);
        }
        let available_layers = unsafe {
            entry
                .enumerate_instance_layer_properties()
                .unwrap_or_else(|err| {
                    panic!("Failed to enumerate the available layers: {:?}\n", err)
                })
        };
        for required in required_layers_cstr {
            let mut is_available = false;
            'inner: for available in &available_layers {
                let name = available.layer_name_as_c_str().unwrap();
                if name == required {
                    is_available = true;
                    break 'inner;
                }
            }
            if !is_available {
                panic!("The required layer {:?} is not available!\n", required);
            }
        }

        // Init the Vulkan instance
        let binding = CString::new(app_params.name.clone()).unwrap();
        let application_info = vk::ApplicationInfo::default()
            .application_name(binding.as_c_str())
            .api_version(API_VERSION_1_3);

        let create_instance_info = vk::InstanceCreateInfo::default()
            .application_info(&application_info)
            .enabled_extension_names(&required_extensions)
            .enabled_layer_names(&required_layers);
        unsafe {
            entry
                .create_instance(&create_instance_info, None)
                .unwrap_or_else(|err| panic!("Failed to create the instance: {:?}\n", err))
        }
    }

    pub fn clear_instance(&self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
