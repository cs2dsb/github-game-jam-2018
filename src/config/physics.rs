use amethyst::core::cgmath::Vector2;

#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicsConfig {
  pub gravity: f32,
  pub lift_width: f32,
  pub lift_height: f32,
  pub lift_y_offset: f32,
  pub lift_velocity: Vector2<f32>,
  pub lift_velocity_rotation: f32,
  pub change_direction_width: f32,
  pub change_direction_height: f32,
  pub exit_width: f32,
  pub exit_height: f32,
  //How much acceleration the walker system applies
  pub walker_force: f32,
  pub ram_velocity: Vector2<f32>,
  pub ram_density: f32,
  pub ram_life: f32,
}

impl Default for PhysicsConfig {
  fn default() -> Self {
    Self {
      gravity: -9.81,
      lift_width: 50.0,
      lift_height: 50.0,
      lift_y_offset: -20.0,
      lift_velocity: Vector2::new(1.0, 1.0),
      lift_velocity_rotation: 0.1,
      change_direction_width: 50.0,
      change_direction_height: 50.0,
      exit_width: 50.0,
      exit_height: 50.0,
      walker_force: 5.0,
      ram_velocity: Vector2::new(10.0, 0.0),
      ram_density: 100.0,
      ram_life: 0.5,
    }
  }
}