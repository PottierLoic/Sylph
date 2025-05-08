use renderer::Renderer;
use winit::{
  event::{Event, WindowEvent},
  event_loop::EventLoopBuilder,
  window::WindowBuilder,
};

fn main() -> anyhow::Result<()> {
  let event_loop = EventLoopBuilder::new().build()?;

  let window = WindowBuilder::new()
    .with_title("Sylph")
    .build(&event_loop)?;

  let mut renderer = unsafe { Renderer::new(&window, true)? };

  let _ = event_loop.run(move |event, target| match event {
    Event::WindowEvent {
      event: WindowEvent::CloseRequested,
      ..
    } => target.exit(),

    Event::AboutToWait => {
      renderer.render().unwrap();
    }

    _ => {}
  });
  Ok(())
}
