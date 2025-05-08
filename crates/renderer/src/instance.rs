use vulkanalia::Entry;
use vulkanalia::Instance;
use vulkanalia::vk::{self, HasBuilder};

use winit::window::Window;

use anyhow::Result;
use std::ffi::CString;

#[cfg(target_os = "windows")]
pub const VK_LIBRARY: &str = "vulkan-1.dll";
#[cfg(target_os = "linux")]
pub const VK_LIBRARY: &str = "libvulkan.so.1";
#[cfg(target_os = "macos")]
pub const VK_LIBRARY: &str = "libvulkan.dylib";

pub unsafe fn create_instance(
  entry: &Entry,
  window: &Window,
  enable_validation: bool,
) -> Result<Instance> {
  unsafe {
    let app_name = CString::new("Sylph").unwrap();
    let engine_name = CString::new("Sylph Engine").unwrap();

    let app_info = vk::ApplicationInfo::builder()
      .application_name(app_name.as_bytes_with_nul())
      .application_version(vk::make_version(1, 0, 0))
      .engine_name(engine_name.as_bytes_with_nul())
      .engine_version(vk::make_version(1, 0, 0))
      .api_version(vk::make_version(1, 3, 0));

    let mut extensions = vulkanalia::window::get_required_instance_extensions(window)
      .iter()
      .map(|s| s.as_ptr())
      .collect::<Vec<*const i8>>();

    if enable_validation {
      extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
    }

    let layers = if enable_validation {
      vec![b"VK_LAYER_KHRONOS_validation\0".as_ptr() as *const i8]
    } else {
      Vec::new()
    };

    let info = vk::InstanceCreateInfo::builder()
      .application_info(&app_info)
      .enabled_layer_names(&layers)
      .enabled_extension_names(&extensions);

    let instance = entry.create_instance(&info, None)?;
    Ok(instance)
  }
}
