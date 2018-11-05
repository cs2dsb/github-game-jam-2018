#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicsConfig {
  pub gravity: f32,
}

impl Default for PhysicsConfig {
  fn default() -> Self {
    Self {
      gravity: -9.81,
    }
  }
}