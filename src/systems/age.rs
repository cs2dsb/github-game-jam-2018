use amethyst::{
  core::timing::Time,
  ecs::prelude::*,
};

use ::components::Age as AgeComponent;

#[derive(Default)]
pub struct Age;

impl<'s> System<'s> for Age {
  type SystemData = (
    Read<'s, Time>,
    WriteStorage<'s, AgeComponent>,
  );

  fn run(&mut self, (time, mut age): Self::SystemData) {
    let delta = time.delta_seconds();

    for a in (&mut age).join() {
      a.seconds += delta;
    }
  }
}