use amethyst::ecs::prelude::*;

pub trait Level {
  fn create_entities(&self, world: &mut World);
}

mod level1;

pub use self::level1::*;
