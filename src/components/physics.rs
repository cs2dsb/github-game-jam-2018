use std::mem::swap;

use amethyst::core::{
  transform::components::Transform,
  cgmath::{
    Quaternion,
    Euler,
    Rad,
  },
};

use nphysics2d::{
  object::{
    BodyHandle,
    ColliderHandle,
  },
  force_generator::{
    ForceGeneratorHandle,
  }
};

use nalgebra::{
  Point2 as naPoint2,
  Isometry2,
};

use ::resources::SCALE_PIXELS_PER_METER;

#[derive(Debug, Clone)]
pub struct Collider {
  pub body_handle: BodyHandle,
  pub collider_handle: ColliderHandle,
  //Used for lerping between steps
  pub transform_current: Transform,
  pub transform_next: Transform,
}

impl Collider {
  pub fn new(body_handle: BodyHandle, collider_handle: ColliderHandle) -> Self {
    Self {
      body_handle,
      collider_handle,
      transform_current: Transform::default(),
      transform_next: Transform::default(),
    }
  }

  //Moves next to current and updates next
  pub fn update_transform(&mut self, isometry: &Isometry2<f32>) {
    //Swap next and current
    swap(&mut self.transform_current, &mut self.transform_next);

    //Update next
    let transform = &mut self.transform_next;
    let point = naPoint2::from(isometry.translation.vector);
    transform.translation.x = point.x * SCALE_PIXELS_PER_METER;
    transform.translation.y = point.y * SCALE_PIXELS_PER_METER;
    transform.rotation = Quaternion::from(Euler {
      x: Rad(0.0),
      y: Rad(0.0),
      z: Rad(isometry.rotation.angle()),
    });
  }

  pub fn lerp_transform(&self, transform: &mut Transform, alpha: f32) {
    //TODO: This is all pretty hideous with all the copy pasta, there must be a cleaner way to do this

    //There's no lerp for vectors in cgmath that I could find
    transform.translation.x = lerp(0.0, 1.0, self.transform_current.translation.x, self.transform_next.translation.x, alpha);
    transform.translation.y = lerp(0.0, 1.0, self.transform_current.translation.y, self.transform_next.translation.y, alpha);
    transform.translation.z = lerp(0.0, 1.0, self.transform_current.translation.z, self.transform_next.translation.z, alpha);

    transform.rotation.v.x = self.transform_current.rotation.v.x;
    transform.rotation.v.y = self.transform_current.rotation.v.y;
    transform.rotation.v.z = self.transform_current.rotation.v.z;
    transform.rotation.s = self.transform_current.rotation.s;
    //TODO: I don't know what this will do if alpha is outside 0-1
    transform.rotation.nlerp(self.transform_next.rotation, alpha);
  }
}

#[derive(Debug, Clone)]
pub struct ForceGenerator {
  pub force_generator_handle: ForceGeneratorHandle,
}

//Non clamping lerp. Allows output to go beyond y2 which is probably reasonable for physics lerping but not everything
fn lerp(x1: f32, x2: f32, y1: f32, y2: f32, x: f32) -> f32 {
  let xd = x2-x1;
  assert!(xd != 0.0);

  ((x - x1)*(y2-y1))/xd + y1
}