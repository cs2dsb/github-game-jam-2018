use amethyst::{
  controls::FlyControlTag,
  core::{
    cgmath::{
      Vector3,
    },
    timing::Time,
    transform::components::Transform,
  },
  ecs::prelude::*,
  input::InputHandler,
};
use std::ops::Mul;

use ::config::PawnConfig;

pub struct PawnMovement;

impl<'s> System<'s> for PawnMovement {
  type SystemData = (
    WriteStorage<'s, Transform>,
    Read<'s, Time>,
    Read<'s, InputHandler<String, String>>,
    ReadStorage<'s, FlyControlTag>,
    Read<'s, PawnConfig>,
  );

  fn run(&mut self, (mut transforms, time, input, fly_tags, pawn_config): Self::SystemData) {
    let delta = time.delta_seconds();
    if let (Some(x), Some(y), Some(z)) = (
        input.axis_value("move_x"),
        input.axis_value("move_y"),
        input.axis_value("move_z"))
    {

      //TODO: x+z movement doesn't move pawn parallel to the axis, probably due to cameras rotation in y not being accounted for
      let movement = Vector3 {
        x: x as f32 * delta * pawn_config.velocity.x,
        y: 0.0,
        z: z as f32 * delta * pawn_config.velocity.z,
      };

      for (_tag, transform) in (&fly_tags, &mut transforms).join() {
        let mut rotated_movement = transform.rotation.mul(movement);
        //The rotation will create a movement in y, we don't want that
        rotated_movement.y = 0.0;
        rotated_movement.y = y as f32 * delta * pawn_config.velocity.y;

        transform.translation += rotated_movement
      }
    }
  }
}