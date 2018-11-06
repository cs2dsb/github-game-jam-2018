use amethyst::{
  controls::FlyControlTag,
  core::{
    transform::components::Transform,
    Time,
  },
  ecs::prelude::*,
};

use ::components::Matriarch;

//Multiplied by time to give a fraction of how much the target location contributes to the new
// location of the camera.
const CONVERGENCE_SPEED: f32 = 2.0;

pub struct CameraMovement;

impl<'s> System<'s> for CameraMovement {
  type SystemData = (
    Read<'s, Time>,
    WriteStorage<'s, Transform>,
    ReadStorage<'s, FlyControlTag>,
    ReadStorage<'s, Matriarch>,
  );

  fn run(&mut self, (time, mut transforms, fly_tags, matriarchs): Self::SystemData) {
    let mut matriarch_transform = None;
    for (t, _matriarch) in (&transforms, &matriarchs).join() {
      matriarch_transform = Some(t.clone());
    }

    if let Some(matriarch_transform) = matriarch_transform {
      let delta = time.delta_seconds();
      for (t, _tag) in (&mut transforms, &fly_tags).join() {
        t.translation.x += (matriarch_transform.translation.x - t.translation.x) * delta * CONVERGENCE_SPEED;
        t.translation.y += (matriarch_transform.translation.y - t.translation.y) * delta * CONVERGENCE_SPEED;
      }
    }


    /*
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
    }*/
  }
}