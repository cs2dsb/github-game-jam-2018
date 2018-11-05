use amethyst::{
  core::{
    //transform::components::Transform,
    timing::Time,
  },
  ecs::prelude::*,
};

use ::resources::PhysicsWorld;

pub struct PhysicsStep;

impl<'s> System<'s> for PhysicsStep {
  type SystemData = (
    //WriteStorage<'s, Transform>,
    Read<'s, Time>,
    Write<'s, PhysicsWorld>,
  );

  fn run(&mut self, (/*mut transforms,*/ time, mut physics_world): Self::SystemData) {
    let delta = time.delta_seconds();
    physics_world.step(delta);

    //for (transform, velocity) in (&mut transforms, &basic_velocities).join() {
    //  transform.translation += velocity.velocity * delta;
    //}
  }
}