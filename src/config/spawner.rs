#[derive(Debug, Deserialize, Serialize)]
pub struct SpawnerConfig {
  pub frequency_min: f32,
  pub frequency_max: f32,
  //What the frequency is set to unless specified by the level
  pub frequency_default: f32,
  //What the max spawns is set to unless specified by the level
  pub max_default: u32,
}

impl Default for SpawnerConfig {
  fn default() -> Self {
    Self {
      frequency_min: 0.2,
      frequency_max: 10.0,
      frequency_default: 2.0,
      max_default: 50,
    }
  }
}