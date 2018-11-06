use amethyst::{
  core::{
    timing::Time,
  },
  ecs::prelude::*,
};

use ::components::{
  Family as FamilyComponent,
  Matriarch,
};

pub struct Family {
  elapsed: f32,
}

impl Default for Family {
  fn default() -> Self {
    Self {
      elapsed: 0.0,
    }
  }
}

impl<'s> System<'s> for Family {
  type SystemData = (
    Read<'s, Time>,
    Entities<'s>,
    ReadStorage<'s, FamilyComponent>,
    ReadStorage<'s, Matriarch>,
    Read<'s, LazyUpdate>,
  );

  fn run(&mut self, (time, entities, family_components, matriarchs, updater): Self::SystemData) {
    self.elapsed += time.delta_seconds();

    if self.elapsed >= 5.0 {
      self.elapsed -= 5.0;
      for (e, _) in (&entities, &matriarchs).join() {
        if entities.is_alive(e) {
          info!("Murdering Matriarch {:?}", e);
          let mut next_matriarch = None;
          if let Some(fam) = family_components.get(e) {
            next_matriarch = fam.next;
          }
          if let Some(next_matriarch) = next_matriarch {
            info!("Promoted {:?} to Matriarch", next_matriarch);
            updater.insert(next_matriarch, Matriarch);
          }
          entities.delete(e).expect("Failed to delete entity");
        }
      }
    }
  }
}