use log::LevelFilter;
use amethyst::{
  prelude::*,
  utils::{
    application_root_dir,
  },
  config::ConfigError,
};

mod pawn;
mod physics;
mod camera;
mod spawner;
mod levels;
mod sound;
mod sprite;

pub use self::pawn::PawnConfig;
pub use self::physics::PhysicsConfig;
pub use self::camera::CameraConfig;
pub use self::spawner::SpawnerConfig;
pub use self::levels::*;
pub use self::sound::SoundConfig;
pub use self::sprite::SpritesConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct GameConfig {
  pub log_level: LevelFilter,
  pub pawn: PawnConfig,
  pub physics: PhysicsConfig,
  pub camera: CameraConfig,
  pub spawner: SpawnerConfig,
  pub sound: SoundConfig,
  pub levels: LevelsConfig,
  pub sprites: SpritesConfig,
}

impl Default for GameConfig {
  fn default() -> Self {
    Self {
      log_level: LevelFilter::Debug,
      pawn: Default::default(),
      physics: Default::default(),
      camera: Default::default(),
      spawner: Default::default(),
      sound: Default::default(),
      levels: Default::default(),
      sprites: Default::default(),
    }
  }
}

pub fn load_game_config() -> Result<GameConfig, ConfigError> {
  GameConfig::load_no_fallback(&format!("{}/resources/config.ron", application_root_dir()))
}