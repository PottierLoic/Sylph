use anyhow::Result;
use std::cmp::{max, min};
use vulkanalia::Device;
use vulkanalia::Instance;
use vulkanalia::vk;
use vulkanalia::vk::DeviceV1_0;
use vulkanalia::vk::Handle;
use vulkanalia::vk::HasBuilder;
use vulkanalia::vk::KhrSurfaceExtension;
use vulkanalia::vk::KhrSwapchainExtension;
use winit::window::Window;

pub unsafe fn create_swapchain(
  instance: &Instance,
  device: &Device,
  physical_device: vk::PhysicalDevice,
  surface: vk::SurfaceKHR,
  window: &Window,
) -> Result<(
  vk::SwapchainKHR,
  vk::Format,
  vk::Extent2D,
  Vec<vk::ImageView>,
)> {
  unsafe {
    let surface_caps =
      instance.get_physical_device_surface_capabilities_khr(physical_device, surface)?;
    let surface_formats =
      instance.get_physical_device_surface_formats_khr(physical_device, surface)?;
    let surface_present_modes =
      instance.get_physical_device_surface_present_modes_khr(physical_device, surface)?;

    let surface_format = surface_formats
      .iter()
      .cloned()
      .find(|sf| {
        sf.format == vk::Format::B8G8R8A8_UNORM
          && sf.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
      })
      .unwrap_or(surface_formats[0]);

    let present_mode = surface_present_modes
      .iter()
      .cloned()
      .find(|&pm| pm == vk::PresentModeKHR::MAILBOX)
      .unwrap_or(vk::PresentModeKHR::FIFO);

    let mut extent = surface_caps.current_extent;
    if extent.width == u32::MAX {
      let size = window.inner_size();
      extent.width = max(
        surface_caps.min_image_extent.width,
        min(surface_caps.max_image_extent.width, size.width),
      );
      extent.height = max(
        surface_caps.min_image_extent.height,
        min(surface_caps.max_image_extent.height, size.height),
      );
    }

    let image_count = surface_caps.min_image_count + 1;
    let image_count = if surface_caps.max_image_count > 0 {
      image_count.min(surface_caps.max_image_count)
    } else {
      image_count
    };

    let queue_family_indices = [0];

    let info = vk::SwapchainCreateInfoKHR::builder()
      .surface(surface)
      .min_image_count(image_count)
      .image_format(surface_format.format)
      .image_color_space(surface_format.color_space)
      .image_extent(extent)
      .image_array_layers(1)
      .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
      .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
      .queue_family_indices(&queue_family_indices)
      .pre_transform(surface_caps.current_transform)
      .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
      .present_mode(present_mode)
      .clipped(true)
      .old_swapchain(vk::SwapchainKHR::null());

    let swapchain = device.create_swapchain_khr(&info, None)?;

    let images = device.get_swapchain_images_khr(swapchain)?;

    let image_views = images
      .iter()
      .map(|&image| {
        let view_info = vk::ImageViewCreateInfo::builder()
          .image(image)
          .view_type(vk::ImageViewType::_2D)
          .format(surface_format.format)
          .components(vk::ComponentMapping {
            r: vk::ComponentSwizzle::IDENTITY,
            g: vk::ComponentSwizzle::IDENTITY,
            b: vk::ComponentSwizzle::IDENTITY,
            a: vk::ComponentSwizzle::IDENTITY,
          })
          .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
          });

        device.create_image_view(&view_info, None)
      })
      .collect::<Result<Vec<_>, _>>()?;

    Ok((swapchain, surface_format.format, extent, image_views))
  }
}
