mod pawn;
mod physics;
mod camera;

pub use self::pawn::PawnConfig;
pub use self::physics::PhysicsConfig;
pub use self::camera::CameraConfig;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GameConfig {
  pub pawn: PawnConfig,
  pub physics: PhysicsConfig,
  pub camera: CameraConfig,
}