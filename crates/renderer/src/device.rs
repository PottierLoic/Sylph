use anyhow::Result;
use std::ffi::CString;
use vulkanalia::Device;
use vulkanalia::Instance;
use vulkanalia::vk;
use vulkanalia::vk::HasBuilder;
use vulkanalia::vk::InstanceV1_0;
use vulkanalia::vk::KHR_PORTABILITY_SUBSET_EXTENSION;
use vulkanalia::vk::KHR_SWAPCHAIN_EXTENSION;
use vulkanalia::vk::KhrSurfaceExtension;

pub unsafe fn pick_physical_device(
  instance: &Instance,
  surface: vk::SurfaceKHR,
) -> Option<vk::PhysicalDevice> {
  unsafe {
    let devices = instance.enumerate_physical_devices().ok()?;

    for &device in &devices {
      let queue_families = instance.get_physical_device_queue_family_properties(device);

      let mut graphics_index = None;
      let mut present_index = None;

      for (i, prop) in queue_families.iter().enumerate() {
        if prop.queue_flags.contains(vk::QueueFlags::GRAPHICS) && graphics_index.is_none() {
          graphics_index = Some(i as u32);
        }

        if instance
          .get_physical_device_surface_support_khr(device, i as u32, surface)
          .unwrap_or(false)
          && present_index.is_none()
        {
          present_index = Some(i as u32);
        }
      }

      if graphics_index.is_some() && present_index.is_some() {
        return Some(device);
      }
    }

    None
  }
}

pub unsafe fn create_logical_device(
  instance: &Instance,
  physical_device: vk::PhysicalDevice,
  surface: vk::SurfaceKHR,
  enable_validation: bool,
) -> Result<(Device, u32, u32)> {
  unsafe {
    let queue_families = instance.get_physical_device_queue_family_properties(physical_device);

    let mut graphics_index = None;
    let mut present_index = None;

    for (i, prop) in queue_families.iter().enumerate() {
      if prop.queue_flags.contains(vk::QueueFlags::GRAPHICS) && graphics_index.is_none() {
        graphics_index = Some(i as u32);
      }

      if instance
        .get_physical_device_surface_support_khr(physical_device, i as u32, surface)
        .unwrap_or(false)
        && present_index.is_none()
      {
        present_index = Some(i as u32);
      }
    }

    let graphics_index = graphics_index.unwrap();
    let present_index = present_index.unwrap();

    let unique_indices = if graphics_index != present_index {
      vec![graphics_index, present_index]
    } else {
      vec![graphics_index]
    };

    let priorities = [1.0f32];
    let queue_infos: Vec<_> = unique_indices
      .iter()
      .map(|&index| {
        vk::DeviceQueueCreateInfo::builder()
          .queue_family_index(index)
          .queue_priorities(&priorities)
          .build()
      })
      .collect();

    let mut device_extensions = vec![KHR_SWAPCHAIN_EXTENSION.name.as_ptr()];
    if cfg!(target_os = "macos") {
      device_extensions.push(KHR_PORTABILITY_SUBSET_EXTENSION.name.as_ptr());
    }

    let features = vk::PhysicalDeviceFeatures::builder();

    let layers = if enable_validation {
      vec![
        CString::new("VK_LAYER_KHRONOS_validation")
          .unwrap()
          .as_ptr(),
      ]
    } else {
      Vec::new()
    };

    let device_info = vk::DeviceCreateInfo::builder()
      .queue_create_infos(&queue_infos)
      .enabled_extension_names(&device_extensions)
      .enabled_layer_names(&layers)
      .enabled_features(&features);

    let device = instance
      .create_device(physical_device, &device_info, None)
      .map_err(|e| anyhow::anyhow!("Failed to create device: {}", e))?;

    Ok((device, graphics_index, present_index))
  }
}
