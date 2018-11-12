use amethyst::{
  controls::FlyControlTag,
  core::{
    transform::components::Transform,
    cgmath::Vector3,
    Time,
  },
  ecs::prelude::*,
};

use ::{
  components::{
    Matriarch,
    Walker,
    Direction,
  },
  config::CameraConfig,
};

#[derive(Default)]
pub struct CameraMovement;

impl<'s> System<'s> for CameraMovement {
  type SystemData = (
    Read<'s, Time>,
    WriteStorage<'s, Transform>,
    ReadStorage<'s, FlyControlTag>,
    ReadStorage<'s, Matriarch>,
    ReadStorage<'s, Walker>,
    Read<'s, CameraConfig>,
  );

  fn run(&mut self, (time, mut transforms, fly_tags, matriarchs, walkers, camera_config): Self::SystemData) {
    let mut matriarch_translation = Vector3::new(0.0, 0.0, 0.0);
    let mut num_matriarchs = 0;

    for (t, _matriarch, w) in (&transforms, &matriarchs, &walkers).join() {
      num_matriarchs += 1;
      matriarch_translation += t.translation;
      match w.direction {
        Direction::Right => matriarch_translation += camera_config.offset,
        Direction::Left => matriarch_translation -= camera_config.offset,
      }
    }

    if num_matriarchs > 0 {
      matriarch_translation /= num_matriarchs as f32;
      let delta = time.delta_seconds();
      for (t, _tag) in (&mut transforms, &fly_tags).join() {
        t.translation.x += (matriarch_translation.x - t.translation.x) * delta * camera_config.convergence_speed;
        t.translation.y += (matriarch_translation.y - t.translation.y) * delta * camera_config.convergence_speed;
      }
    }
  }
}