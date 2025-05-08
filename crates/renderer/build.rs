use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let out_dir = env::var("OUT_DIR")?;
  let shader_dir = Path::new("src/shaders");
  let out_shader_dir = Path::new(&out_dir).join("shaders");
  fs::create_dir_all(&out_shader_dir)?;

  let compiler = shaderc::Compiler::new().ok_or("Failed to create shader compiler")?;
  let mut options = shaderc::CompileOptions::new().ok_or("Failed to create compile options")?;
  options.set_target_env(
    shaderc::TargetEnv::Vulkan,
    shaderc::EnvVersion::Vulkan1_0 as u32,
  );
  options.set_optimization_level(shaderc::OptimizationLevel::Zero);
  options.set_generate_debug_info();

  for entry in fs::read_dir(shader_dir)? {
    let entry = entry?;
    let path = entry.path();
    if path.extension().and_then(|s| s.to_str()) == Some("vert")
      || path.extension().and_then(|s| s.to_str()) == Some("frag")
    {
      let shader_type = if path.extension().unwrap() == "vert" {
        shaderc::ShaderKind::Vertex
      } else {
        shaderc::ShaderKind::Fragment
      };

      let source = fs::read_to_string(&path)?;
      let compiled = compiler.compile_into_spirv(
        &source,
        shader_type,
        path.file_name().unwrap().to_str().unwrap(),
        "main",
        Some(&options),
      )?;

      let binary = compiled.as_binary_u8();
      let stem = path.file_stem().unwrap().to_str().unwrap();
      let ext = path.extension().unwrap().to_str().unwrap();
      let out_filename = format!("{}.{}.spv", stem, ext);
      let out_path = out_shader_dir.join(out_filename);

      fs::write(&out_path, binary)?;
    }
  }

  println!("cargo:rerun-if-changed=src/shaders");
  Ok(())
}
