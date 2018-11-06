use amethyst::ecs::prelude::*;

use ::{
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

const FORCE: f32 = 5.0;

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
  );

  fn run(&mut self, (walkers, colliders, mut physics_world): Self::SystemData) {
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
      if let Some(dir) = match walker.direction {
        Direction::Right => {
          if right.is_none() {
            right = Some(ConstantAcceleration::new(Vector2::new(FORCE, 0.0), zero()));
          }
          &mut right
        },
        Direction::Left => {
          if left.is_none() {
            left = Some(ConstantAcceleration::new(Vector2::new(-FORCE, 0.0), zero()));
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