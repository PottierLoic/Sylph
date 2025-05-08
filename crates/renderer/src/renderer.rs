use vulkanalia::Entry;
use vulkanalia::loader::LibloadingLoader;
use vulkanalia::vk::Handle;
use vulkanalia::vk::KhrSwapchainExtension;
use vulkanalia::vk::{self, DeviceV1_0, HasBuilder};
use vulkanalia::window as vk_window;

use anyhow::{Result, anyhow};
use winit::window::Window;

use crate::context::VulkanContext;
use crate::device::{create_logical_device, pick_physical_device};
use crate::instance::create_instance;
use crate::pipeline::create_render_pass;
use crate::swapchain::create_swapchain;

pub struct Renderer {
  pub context: VulkanContext,
  pub format: vk::Format,
  pub extent: vk::Extent2D,
  pub swapchain: vk::SwapchainKHR,
  pub image_views: Vec<vk::ImageView>,
  pub render_pass: vk::RenderPass,
  pub framebuffers: Vec<vk::Framebuffer>,
  pub command_buffer: vk::CommandBuffer,
  pub image_available_semaphore: vk::Semaphore,
  pub render_finished_semaphore: vk::Semaphore,
  pub images: Vec<vk::Image>,
}

impl Renderer {
  pub unsafe fn new(window: &Window, enable_validation: bool) -> Result<Self> {
    unsafe {
      let loader = LibloadingLoader::new("vulkan-1.dll")?;
      let entry =
        Entry::new(loader).map_err(|e| anyhow!("Failed to create Vulkan entry: {}", e))?;
      let instance = create_instance(&entry, window, enable_validation)?;
      let surface = vk_window::create_surface(&instance, window, window)?;

      let physical_device = pick_physical_device(&instance, surface)
        .ok_or_else(|| anyhow!("No suitable physical device found"))?;

      let (device, graphics_queue_family, present_queue_family) =
        create_logical_device(&instance, physical_device, surface, enable_validation)?;

      let graphics_queue = device.get_device_queue(graphics_queue_family, 0);
      let present_queue = device.get_device_queue(present_queue_family, 0);

      let (swapchain, format, extent, image_views) =
        create_swapchain(&instance, &device, physical_device, surface, window)?;

      let render_pass = create_render_pass(&device, format)?;
      let framebuffers = image_views
        .iter()
        .map(|&view| {
          let attachments = [view];
          let framebuffer_info = vk::FramebufferCreateInfo::builder()
            .render_pass(render_pass)
            .attachments(&attachments)
            .width(extent.width)
            .height(extent.height)
            .layers(1);
          device.create_framebuffer(&framebuffer_info, None)
        })
        .collect::<Result<Vec<_>, _>>()?;

      let command_pool_info = vk::CommandPoolCreateInfo::builder()
        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
        .queue_family_index(graphics_queue_family);
      let command_pool = device.create_command_pool(&command_pool_info, None)?;

      let allocate_info = vk::CommandBufferAllocateInfo::builder()
        .command_pool(command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);

      let command_buffer = device.allocate_command_buffers(&allocate_info)?[0];

      let semaphore_info = vk::SemaphoreCreateInfo::builder();
      let image_available_semaphore = device.create_semaphore(&semaphore_info, None)?;
      let render_finished_semaphore = device.create_semaphore(&semaphore_info, None)?;

      let images = device.get_swapchain_images_khr(swapchain)?;

      let context = VulkanContext {
        entry,
        instance,
        device,
        physical_device,
        graphics_queue,
        present_queue,
        graphics_queue_family,
        present_queue_family,
        surface,
      };

      Ok(Self {
        context,
        format,
        extent,
        swapchain,
        image_views,
        render_pass,
        framebuffers,
        command_buffer,
        image_available_semaphore,
        render_finished_semaphore,
        images,
      })
    }
  }

  pub fn render(&mut self) -> Result<()> {
    unsafe {
      let (image_index, _) = self.context.device.acquire_next_image_khr(
        self.swapchain,
        u64::MAX,
        self.image_available_semaphore,
        vk::Fence::null(),
      )?;

      let begin_info =
        vk::CommandBufferBeginInfo::builder().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
      self
        .context
        .device
        .begin_command_buffer(self.command_buffer, &begin_info)?;

      let clear_values = [vk::ClearValue {
        color: vk::ClearColorValue {
          float32: [0.0, 0.0, 1.0, 1.0],
        },
      }];

      let barrier = vk::ImageMemoryBarrier::builder()
        .old_layout(vk::ImageLayout::UNDEFINED)
        .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
        .image(self.images[image_index as usize])
        .subresource_range(vk::ImageSubresourceRange {
          aspect_mask: vk::ImageAspectFlags::COLOR,
          base_mip_level: 0,
          level_count: 1,
          base_array_layer: 0,
          layer_count: 1,
        });

      self.context.device.cmd_pipeline_barrier(
        self.command_buffer,
        vk::PipelineStageFlags::TOP_OF_PIPE,
        vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        vk::DependencyFlags::empty(),
        &[] as &[vk::MemoryBarrier],
        &[] as &[vk::BufferMemoryBarrier],
        &[barrier.build()],
      );

      let render_pass_info = vk::RenderPassBeginInfo::builder()
        .render_pass(self.render_pass)
        .framebuffer(self.framebuffers[image_index as usize])
        .render_area(vk::Rect2D {
          offset: vk::Offset2D { x: 0, y: 0 },
          extent: self.extent,
        })
        .clear_values(&clear_values);

      self.context.device.cmd_begin_render_pass(
        self.command_buffer,
        &render_pass_info,
        vk::SubpassContents::INLINE,
      );

      self.context.device.cmd_end_render_pass(self.command_buffer);
      self
        .context
        .device
        .end_command_buffer(self.command_buffer)?;

      let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
      let submit_info = vk::SubmitInfo::builder()
        .wait_semaphores(std::slice::from_ref(&self.image_available_semaphore))
        .wait_dst_stage_mask(&wait_stages)
        .command_buffers(std::slice::from_ref(&self.command_buffer))
        .signal_semaphores(std::slice::from_ref(&self.render_finished_semaphore));

      self.context.device.queue_submit(
        self.context.graphics_queue,
        &[submit_info.build()],
        vk::Fence::null(),
      )?;

      let present_info = vk::PresentInfoKHR::builder()
        .wait_semaphores(std::slice::from_ref(&self.render_finished_semaphore))
        .swapchains(std::slice::from_ref(&self.swapchain))
        .image_indices(std::slice::from_ref(&image_index));

      self
        .context
        .device
        .queue_present_khr(self.context.present_queue, &present_info)?;
      self
        .context
        .device
        .queue_wait_idle(self.context.present_queue)?;
    }

    Ok(())
  }
}
