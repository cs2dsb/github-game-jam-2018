use log::LevelFilter;

mod pawn;
mod physics;
mod camera;
mod spawner;

pub use self::pawn::PawnConfig;
pub use self::physics::PhysicsConfig;
pub use self::camera::CameraConfig;
pub use self::spawner::SpawnerConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct GameConfig {
  pub log_level: LevelFilter,
  pub pawn: PawnConfig,
  pub physics: PhysicsConfig,
  pub camera: CameraConfig,
  pub spawner: SpawnerConfig,
}

impl Default for GameConfig {
  fn default() -> Self {
    Self {
      log_level: LevelFilter::Debug,
      pawn: Default::default(),
      physics: Default::default(),
      camera: Default::default(),
      spawner: Default::default(),
    }
  }
}