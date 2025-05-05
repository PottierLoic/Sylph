# Sylph Game Engine - Roadmap

## 1. Core Architecture
- [x] Entity-Component-System (ECS)
  - [x] Entity manager with ID recycling
  - [x] Component storage (dense hashmap)
  - [x] Labeling system for named entities
  - [x] Manual system execution
- [ ] System runner and scheduler
  - [ ] Support for RunMode (EveryTick, Once, Every(n))
  - [ ] Dependency ordering

## 2. Rendering
- [ ] Vulkan rendering backend
  - [ ] Window creation with winit
  - [ ] Swapchain and basic pipeline
  - [ ] Uploading meshes and textures
  - [ ] Camera and transform components
  - [ ] 2D sprite rendering pass
  - [ ] 3D mesh rendering pass

## 3. Scene & Runtime
- [ ] Scene loading and saving (serialization)
- [ ] Basic scene format (TOML or JSON)
- [ ] Runtime execution loop
- [ ] Time and delta time tracking

## 4. Scripting
- [ ] Script component
- [ ] ScriptSystem to run user-defined behavior
- [ ] Hot-reload or plugin interface (VERY optional)

## 5. Physics
- [ ] 2D physics stub
- [ ] 3D physics stub

## 6. UI
- [ ] Immediate mode UI with egui or imgui
- [ ] Integration with world/components
- [ ] Debug panels (inspector, entity list, performance)

## 7. Tooling & Editor (later)
- [ ] In-engine editor UI
- [ ] Scene graph visualization
- [ ] Component inspector
- [ ] Asset viewer

## 8. Platform & Build
- [x] Modular crate structure (`crates/`)
- [x] Workspace configuration
- [ ] Build targets: Windows, Linux, macOS
- [ ] GitHub Actions CI
  - [ ] Build all crates
  - [ ] Run example tests
  - [ ] Release packaging

