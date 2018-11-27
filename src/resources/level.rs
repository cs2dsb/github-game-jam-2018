use ::config::{
  LevelConfig,
  CameraOverrides,
};

#[derive(PartialEq)]
pub enum LoadState {
  NeedConfig,
  NeedLoad,
  Loaded,
  PhysicsCleanup,
}

///Resource that holds the list of levels and tracks which one is loaded
pub struct Level {
  pub current_level: usize,
  pub levels: Vec<LevelConfig>,
  pub prev_camera_settings: Option<CameraOverrides>,
  pub load_state: LoadState,
  pub runtime: f32,
}

impl Default for Level {
  fn default() -> Self {
    Self {
      current_level: 0,
      levels: Vec::new(),
      prev_camera_settings: None,
      load_state: LoadState::NeedConfig,
      runtime: 0.0,
    }
  }
}