use std::ffi::CString;
use std::fs::File;
use std::io::Read;

use anyhow::Result;
use vulkanalia::Device;
use vulkanalia::vk;
use vulkanalia::vk::DeviceV1_0;
use vulkanalia::vk::Handle;
use vulkanalia::vk::HasBuilder;

pub unsafe fn create_render_pass(device: &Device, format: vk::Format) -> Result<vk::RenderPass> {
  unsafe {
    let color_attachment = vk::AttachmentDescription::builder()
      .format(format)
      .samples(vk::SampleCountFlags::_1)
      .load_op(vk::AttachmentLoadOp::CLEAR)
      .store_op(vk::AttachmentStoreOp::STORE)
      .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
      .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
      .initial_layout(vk::ImageLayout::UNDEFINED)
      .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

    let color_attachment_ref = vk::AttachmentReference::builder()
      .attachment(0)
      .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

    let subpass = vk::SubpassDescription::builder()
      .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
      .color_attachments(std::slice::from_ref(&color_attachment_ref));

    let dependency = vk::SubpassDependency::builder()
      .src_subpass(vk::SUBPASS_EXTERNAL)
      .dst_subpass(0)
      .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
      .src_access_mask(vk::AccessFlags::empty())
      .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
      .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE);

    let info = vk::RenderPassCreateInfo::builder()
      .attachments(std::slice::from_ref(&color_attachment))
      .subpasses(std::slice::from_ref(&subpass))
      .dependencies(std::slice::from_ref(&dependency));

    Ok(device.create_render_pass(&info, None)?)
  }
}

pub unsafe fn create_pipeline_layout(device: &Device) -> Result<vk::PipelineLayout> {
  unsafe {
    let info = vk::PipelineLayoutCreateInfo::builder();
    Ok(device.create_pipeline_layout(&info, None)?)
  }
}

pub unsafe fn create_graphics_pipeline(
  device: &Device,
  render_pass: vk::RenderPass,
  layout: vk::PipelineLayout,
  extent: vk::Extent2D,
  vert_path: &str,
  frag_path: &str,
) -> Result<vk::Pipeline> {
  unsafe {
    let vert_code = read_spv(vert_path)?;
    let frag_code = read_spv(frag_path)?;

    let vert_module = create_shader_module(device, &vert_code)?;
    let frag_module = create_shader_module(device, &frag_code)?;

    let entry_point = CString::new("main").unwrap();

    let stage_infos = [
      vk::PipelineShaderStageCreateInfo::builder()
        .stage(vk::ShaderStageFlags::VERTEX)
        .module(vert_module)
        .name(entry_point.as_bytes())
        .build(),
      vk::PipelineShaderStageCreateInfo::builder()
        .stage(vk::ShaderStageFlags::FRAGMENT)
        .module(frag_module)
        .name(entry_point.as_bytes())
        .build(),
    ];

    let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::builder();
    let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::builder()
      .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
      .primitive_restart_enable(false);

    let viewport = vk::Viewport::builder()
      .x(0.0)
      .y(0.0)
      .width(extent.width as f32)
      .height(extent.height as f32)
      .min_depth(0.0)
      .max_depth(1.0);

    let scissor = vk::Rect2D::builder()
      .offset(vk::Offset2D { x: 0, y: 0 })
      .extent(extent);

    let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
      .viewports(std::slice::from_ref(&viewport))
      .scissors(std::slice::from_ref(&scissor));

    let rasterizer = vk::PipelineRasterizationStateCreateInfo::builder()
      .depth_clamp_enable(false)
      .rasterizer_discard_enable(false)
      .polygon_mode(vk::PolygonMode::FILL)
      .line_width(1.0)
      .cull_mode(vk::CullModeFlags::BACK)
      .front_face(vk::FrontFace::CLOCKWISE)
      .depth_bias_enable(false);

    let multisampling = vk::PipelineMultisampleStateCreateInfo::builder()
      .sample_shading_enable(false)
      .rasterization_samples(vk::SampleCountFlags::_1);

    let color_blend_attachment = vk::PipelineColorBlendAttachmentState::builder()
      .color_write_mask(
        vk::ColorComponentFlags::R
          | vk::ColorComponentFlags::G
          | vk::ColorComponentFlags::B
          | vk::ColorComponentFlags::A,
      )
      .blend_enable(false);

    let color_blending = vk::PipelineColorBlendStateCreateInfo::builder()
      .logic_op_enable(false)
      .attachments(std::slice::from_ref(&color_blend_attachment));

    let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
      .stages(&stage_infos)
      .vertex_input_state(&vertex_input_info)
      .input_assembly_state(&input_assembly)
      .viewport_state(&viewport_state)
      .rasterization_state(&rasterizer)
      .multisample_state(&multisampling)
      .color_blend_state(&color_blending)
      .layout(layout)
      .render_pass(render_pass)
      .subpass(0);

    let (pipelines, _) = device.create_graphics_pipelines(
      vk::PipelineCache::null(),
      std::slice::from_ref(&pipeline_info),
      None,
    )?;
    device.destroy_shader_module(vert_module, None);
    device.destroy_shader_module(frag_module, None);
    Ok(
      *pipelines
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("No pipeline created"))?,
    )
  }
}

unsafe fn create_shader_module(device: &Device, code: &[u8]) -> Result<vk::ShaderModule> {
  unsafe {
    let info = vk::ShaderModuleCreateInfo::builder().code(bytemuck::cast_slice(code));
    Ok(device.create_shader_module(&info, None)?)
  }
}

fn read_spv(path: &str) -> Result<Vec<u8>> {
  let mut file = File::open(path)?;
  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;
  Ok(buffer)
}
