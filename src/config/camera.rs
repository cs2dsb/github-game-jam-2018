use amethyst::core::cgmath::Vector3;

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraConfig {
  //Multiplied by time to give a fraction of how much the target location contributes to the new
  // location of the camera.
  pub convergence_speed: f32,
  //How much to offset the camera by (expressed for left to right moving target)
  pub offset: Vector3<f32>,
}

impl Default for CameraConfig {
  fn default() -> Self {
    Self {
      convergence_speed: 2.0,
      offset: Vector3::new(0.0, 0.0, 0.0),
    }
  }
}