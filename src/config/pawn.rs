use amethyst::core::cgmath::Vector3;

#[derive(Debug, Deserialize, Serialize)]
pub struct PawnConfig {
  pub velocity: Vector3<f32>,
}

impl Default for PawnConfig {
  fn default() -> Self {
    Self {
      velocity: Vector3::new(10.0, 0.0, 10.0),
    }
  }
}