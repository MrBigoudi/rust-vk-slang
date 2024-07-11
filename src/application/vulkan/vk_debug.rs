use std::{borrow::Cow, ffi::CStr};

use ash::{ext::debug_utils, vk, Entry, Instance};

use crate::application::vk_app::VulkanApp;

/// Callback function for Vulkan debug messages.
unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{message_severity:?}:\n{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n",
    );

    vk::FALSE
}

impl VulkanApp {
    /// Sets up the Vulkan debug callback for error, warning, and info messages.
    pub fn init_debug_callback(
        entry: &Entry,
        instance: &Instance,
    ) -> (debug_utils::Instance, vk::DebugUtilsMessengerEXT) {
        // Setup debug callback
        let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(vulkan_debug_callback));

        let debug_utils_loader = debug_utils::Instance::new(entry, instance);
        let debug_call_back = unsafe {
            debug_utils_loader
                .create_debug_utils_messenger(&debug_info, None)
                .unwrap()
        };
        (debug_utils_loader, debug_call_back)
    }

    pub fn clear_debug_callback(&self){
        unsafe {
            self.debug_utils_loader
                .destroy_debug_utils_messenger(self.debug_call_back, None);
        }
    }
}
