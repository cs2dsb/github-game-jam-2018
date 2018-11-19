use amethyst::ecs::prelude::*;

use ::{
  config::PhysicsConfig,
  components::{
    Walker as WalkerComponent,
    Direction,
    Collider,
  },
  resources::PhysicsWorld,
};

use nphysics2d::force_generator::{
  ForceGeneratorHandle,
  ConstantAcceleration,
};
use nalgebra::{
  Vector2,
  zero,
};

pub struct Walker {
  right_force: Option<ForceGeneratorHandle>,
  left_force: Option<ForceGeneratorHandle>,
}

impl Default for Walker {
  fn default() -> Self {
    Self {
      right_force: None,
      left_force: None,
    }
  }
}

impl<'s> System<'s> for Walker {
  type SystemData = (
    ReadStorage<'s, WalkerComponent>,
    ReadStorage<'s, Collider>,
    Write<'s, PhysicsWorld>,
    Read<'s, PhysicsConfig>,
  );

  fn run(&mut self, (walkers, colliders, mut physics_world, physics_config): Self::SystemData) {
    //TODO: Couldn't find a way to add and remove body parts from force generators so destroying them every frame instead
    // This doesn't seem great
    if let Some(right) = self.right_force.take() {
      physics_world.world.remove_force_generator(right);
    }

    if let Some(left) = self.left_force.take() {
      physics_world.world.remove_force_generator(left);
    }

    let mut left = None;
    let mut right = None;

    for (walker, collider) in (&walkers, &colliders).join() {
      //There is some flakeyness around waking bodies so this makes sure no
      //walkers ever go to sleep
      physics_world.world.activate_body(collider.body_handle);

      if let Some(dir) = match walker.direction {
        Direction::Right => {
          if right.is_none() {
            right = Some(ConstantAcceleration::new(Vector2::new(physics_config.walker_force, 0.0), zero()));
          }
          &mut right
        },
        Direction::Left => {
          if left.is_none() {
            left = Some(ConstantAcceleration::new(Vector2::new(-physics_config.walker_force, 0.0), zero()));
          }
          &mut left
        },
      } {
        dir.add_body_part(collider.body_handle);
      }
    }

    if let Some(right) = right {
      self.right_force = Some(physics_world.world.add_force_generator(right));
    }

    if let Some(left) = left {
      self.left_force = Some(physics_world.world.add_force_generator(left));
    }
  }
}