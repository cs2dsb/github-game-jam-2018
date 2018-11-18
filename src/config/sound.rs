#[derive(Debug, Deserialize, Serialize)]
pub struct SoundConfig {
  pub volume: f32,
}

impl Default for SoundConfig {
  fn default() -> Self {
    Self {
      volume: 1.0,
    }
  }
}