use nphysics2d::math::Velocity;

///Component gives a physics object a constant velocity
#[derive(Debug, Clone, Copy)]
pub struct ConstantVelocity {
  pub velocity: Velocity<f32>,
}