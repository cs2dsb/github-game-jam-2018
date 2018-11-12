use amethyst::core::cgmath::Vector2;

#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicsConfig {
  pub gravity: f32,
  pub lift_width: f32,
  pub lift_height: f32,
  //Expressed as an inertia independent force (so it's not the same scale as gravity)
  //x component is multiplied by walk direction
  pub lift_force: Vector2<f32>,
  pub change_direction_width: f32,
  pub change_direction_height: f32,
  //How much acceleration the walker system applies
  pub walker_force: f32,
}

impl Default for PhysicsConfig {
  fn default() -> Self {
    Self {
      gravity: -9.81,
      lift_width: 50.0,
      lift_height: 50.0,
      lift_force: Vector2::new(1.0, 1.0),
      change_direction_width: 50.0,
      change_direction_height: 50.0,
      walker_force: 5.0,
    }
  }
}