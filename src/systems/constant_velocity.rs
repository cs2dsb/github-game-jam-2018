use amethyst::{
  ecs::prelude::*,
  assets::AssetStorage,
  audio::{
    Source,
    output::Output,
  },
};

use ::{
  components::{
    Collider,
    ConstantVelocity as ConstantVelocityComponent,
  },
  resources::{
    PhysicsWorld,
    Sounds,
  },
};

//Applies a constant velocity to any collider with a constant velocity component
#[derive(Default)]
pub struct ConstantVelocity;

impl<'s> System<'s> for ConstantVelocity {
  type SystemData = (
    ReadStorage<'s, ConstantVelocityComponent>,
    ReadStorage<'s, Collider>,
    Write<'s, PhysicsWorld>,
    ReadExpect<'s, Sounds>,
    Read<'s, AssetStorage<Source>>,
    Option<Read<'s, Output>>,
  );

  fn run(&mut self, (constant_velocity_components, colliders, mut physics_world, _sounds, _source_storage, _output): Self::SystemData) {
    for (cv, collider) in (&constant_velocity_components, &colliders).join() {
      if let Some(body) = physics_world.world.rigid_body_mut(collider.body_handle) {
        body.set_velocity(cv.velocity);
      }
    }
  }
}