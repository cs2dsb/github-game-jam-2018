use nphysics2d::{
  object::{
    BodyHandle,
    ColliderHandle,
  },
  force_generator::{
    ForceGeneratorHandle,
  }
};

#[derive(Debug, Clone)]
pub struct Collider {
  pub body_handle: BodyHandle,
  pub collider_handle: ColliderHandle,
}

#[derive(Debug, Clone)]
pub struct ForceGenerator {
  pub force_generator_handle: ForceGeneratorHandle,
}