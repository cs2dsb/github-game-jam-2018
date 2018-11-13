use amethyst::{
  core::cgmath::Vector3,
  ecs::prelude::*,
};

///Component represents an indicator pointing at an entity
#[derive(Debug, Clone)]
pub struct Indicator {
  pub target: Entity,
  pub offset: Vector3<f32>,
}