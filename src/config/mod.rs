mod pawn;
pub use self::pawn::PawnConfig;

mod physics;
pub use self::physics::PhysicsConfig;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GameConfig {
  pub pawn: PawnConfig,
  pub physics: PhysicsConfig,
}