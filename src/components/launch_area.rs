use super::Direction;

///Component that launches a walker when their physics bodies overlap. Replaces force generator based lift
#[derive(Debug, Clone, Default)]
pub struct LaunchArea {
  pub direction: Direction,
}