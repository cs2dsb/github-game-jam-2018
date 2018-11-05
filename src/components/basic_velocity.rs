use amethyst::core::cgmath::Vector3;

///Simple movement component, the value of velocity will be added to the local transform every frame
#[derive(Debug, Clone)]
pub struct BasicVelocity {
  pub velocity: Vector3<f32>,
}