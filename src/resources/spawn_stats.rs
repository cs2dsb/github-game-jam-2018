///Resource that tracks how many creeps have been spawned/died/saved/etc.
#[derive(Default)]
pub struct SpawnStats {
  pub total: u32,
  pub spawned: u32,
  pub killed: u32,
  pub saved: u32,
  pub win_ratio: f32,
}

impl SpawnStats {
  pub fn saved_ratio(&self) -> f32 {
    if self.total == 0 {
      0.0
    } else {
      self.saved as f32 / self.total as f32
    }
  }
}