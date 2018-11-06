#[derive(Debug, Deserialize, Serialize)]
pub struct CameraConfig {
  //Multiplied by time to give a fraction of how much the target location contributes to the new
  // location of the camera.
  pub convergence_speed: f32,
}

impl Default for CameraConfig {
  fn default() -> Self {
    Self {
      convergence_speed: 2.0,
    }
  }
}