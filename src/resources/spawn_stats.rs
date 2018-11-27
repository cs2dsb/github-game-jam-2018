///Resource that tracks how many creeps have been spawned/died/saved/etc.
#[derive(Default)]
pub struct SpawnStats {
  pub total: u32,
  pub spawned: u32,
  pub killed: u32,
  pub saved: u32,
}