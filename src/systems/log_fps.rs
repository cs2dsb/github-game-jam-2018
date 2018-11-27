use amethyst::{
  core::Time,
  ecs::prelude::{Read, Write, System},
  utils::fps_counter::FPSCounter,
};

pub struct PeriodicLogTimer {
  elapsed: f32,
}

impl PeriodicLogTimer {
  fn new() -> Self {
    Self {
      elapsed: 0.0,
    }
  }
}

impl Default for PeriodicLogTimer {
  fn default() -> Self {
    Self::new()
  }
}

///Logs the fps value to stdout every 2 seconds
#[derive(Default)]
pub struct LogFps;

impl<'a> System<'a> for LogFps {
  type SystemData = (
    Read<'a, Time>,
    Read<'a, FPSCounter>,
    Write<'a, PeriodicLogTimer>,
  );

  fn run(&mut self, (time, fps_counter, mut periodic): Self::SystemData) {
    periodic.elapsed += time.delta_seconds();

    if periodic.elapsed > 2.0 {
      let fps = fps_counter.sampled_fps();
      info!("FPS: {}", fps);
      periodic.elapsed = 0.0;
    }
  }
}
