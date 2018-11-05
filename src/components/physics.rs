use nphysics2d::object::{
  BodyHandle,
  ColliderHandle,
};

#[derive(Debug, Clone)]
pub struct Collider {
  pub body_handle: BodyHandle,
  pub collider_handle: ColliderHandle,
}