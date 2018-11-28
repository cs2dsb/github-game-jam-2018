use amethyst::core::cgmath::Vector3;

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraConfig {
  //Multiplied by time to give a fraction of how much the target location contributes to the new
  // location of the camera.
  pub convergence_speed: f32,
  //How much to offset the camera by (expressed for left to right moving target)
  pub offset: Vector3<f32>,
  pub z_min: f32,
  pub z_default: f32,
  pub z_max: f32,
  pub zoom_speed: f32,
  //Turned off if <= 0
  pub gridline_width: f32,
  pub final_position: Option<Vector3<f32>>,
}

impl Default for CameraConfig {
  fn default() -> Self {
    Self {
      convergence_speed: 2.0,
      offset: Vector3::new(0.0, 0.0, 0.0),
      z_min: 10.0,
      z_default: 300.0,
      z_max: 500.0,
      zoom_speed: 100.0,
      gridline_width: 1.0,
      final_position: None,
    }
  }
}