use application::vk_app::VulkanApp;

pub mod application;

fn main() {
    env_logger::init();
    // init the app
    let _vk_app: VulkanApp = VulkanApp::new(Default::default());
}
