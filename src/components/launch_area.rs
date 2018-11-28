use std::collections::HashSet;

use amethyst::ecs::prelude::*;

use super::Direction;

///Component that launches a walker when their physics bodies overlap. Replaces force generator based lift
#[derive(Debug, Clone, Default)]
pub struct LaunchArea {
  pub direction: Direction,
  //This is mainly used to stop the lift sound playing over and over
  pub already_launched: HashSet<Entity>,
}

impl LaunchArea {
  pub fn new(direction: Direction) -> Self {
    Self {
      direction,
      already_launched: HashSet::new(),
    }
  }
}