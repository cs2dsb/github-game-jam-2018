use amethyst::{
  core::timing::Time,
  ecs::prelude::*,
};

use ::{
  components::{
    Age as AgeComponent,
    Family,
  },
  resources::SpawnStats,
};

#[derive(Default)]
pub struct Age;

impl<'s> System<'s> for Age {
  type SystemData = (
    Entities<'s>,
    Read<'s, Time>,
    WriteStorage<'s, AgeComponent>,
    ReadStorage<'s, Family>,
    Write<'s, SpawnStats>,
  );

  fn run(&mut self, (entities, time, mut age, family, mut spawn_stats): Self::SystemData) {
    let delta = time.delta_seconds();

    for (e, a) in (&entities, &mut age).join() {
      if entities.is_alive(e) {
        a.seconds += delta;
        if let Some(max) = a.max {
          if a.seconds > max {
            if family.contains(e) {
              spawn_stats.killed += 1;
            }
            entities
              .delete(e)
              .expect("Failed to delete entity");
          }
        }
      }
    }
  }
}