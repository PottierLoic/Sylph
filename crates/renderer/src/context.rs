use vulkanalia::Device;
use vulkanalia::Entry;
use vulkanalia::Instance;
use vulkanalia::vk;

#[derive(Clone)]
pub struct VulkanContext {
  pub entry: Entry,
  pub instance: Instance,
  pub device: Device,
  pub physical_device: vk::PhysicalDevice,
  pub graphics_queue: vk::Queue,
  pub present_queue: vk::Queue,
  pub graphics_queue_family: u32,
  pub present_queue_family: u32,
  pub surface: vk::SurfaceKHR,
}
