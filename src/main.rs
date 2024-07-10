use application::vk_app::VulkanApp;

pub mod application;

fn main() {
    env_logger::init();
    // run the app
    VulkanApp::run(Default::default());
}
