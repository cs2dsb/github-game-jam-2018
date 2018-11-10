#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
  Right,
  Left,
}

impl Direction {
  pub fn reversed(&self) -> Self {
    match self {
      Direction::Right => Direction::Left,
      Direction::Left => Direction::Right,
    }
  }
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

///Component that changes a walkers direction when their physics bodies overlap
#[derive(Debug, Clone)]
pub struct ChangeDirection {
  pub direction: Direction,
}