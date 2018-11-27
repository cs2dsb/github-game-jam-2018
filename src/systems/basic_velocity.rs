use amethyst::{
  core::{
    transform::components::Transform,
    timing::Time,
  },
  ecs::prelude::*,
};

use ::components::BasicVelocity as BasicVelocityComponent;

///Adds velocity*delta to transform each frame
#[derive(Default)]
pub struct BasicVelocity;

impl<'s> System<'s> for BasicVelocity {
  type SystemData = (
    WriteStorage<'s, Transform>,
    Read<'s, Time>,
    ReadStorage<'s, BasicVelocityComponent>,
  );

  fn run(&mut self, (mut transforms, time, basic_velocities): Self::SystemData) {
    let delta = time.delta_seconds();
    for (transform, velocity) in (&mut transforms, &basic_velocities).join() {
      transform.translation += velocity.velocity * delta;
    }
  }
}