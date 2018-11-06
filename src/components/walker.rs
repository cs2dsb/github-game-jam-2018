#[derive(Debug, Clone)]
pub enum Direction {
  Right,
  Left,
}

impl Default for Direction {
  fn default() -> Self {
    Direction::Right
  }
}

///Component that applies a force to an entity to cause it to move in one direction until it hits a wall
#[derive(Debug, Clone)]
pub struct Walker {
  //force is current set system wide in the Walker system... pub force: f32,
  pub direction: Direction,
}

impl Default for Walker {
  fn default() -> Self {
    Self {
      //force: 1.0,
      direction: Default::default(),
    }
  }
}